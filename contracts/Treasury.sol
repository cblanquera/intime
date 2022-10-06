// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import "@openzeppelin/contracts/utils/Address.sol";
import "@openzeppelin/contracts/utils/Context.sol";
import "@openzeppelin/contracts/security/Pausable.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/access/AccessControlEnumerable.sol";

// ============ Errors ============

error InvalidCall();

// ============ Contract ============

contract Treasury is
  Context,
  Pausable,
  AccessControlEnumerable,
  ReentrancyGuard
{
  // ============ Events ============

  event FundsReceived(address sender, uint256 amount);
  event FundsRequested(uint256 id);
  event RequestCancelled(uint256 id);
  event FundsApprovedFrom(address approver, uint256 id);
  event FundsApproved(uint256 id);
  event FundsWithdrawn(uint256 id);

  // ============ Structs ============

  //transaction structure
  struct TX {
    uint8 tier;
    address beneficiary;
    uint256 amount;
    string uri;
    uint256 approvals;
    bool withdrawn;
    bool cancelled;
  }

  // ============ Constants ============

  //custom roles
  bytes32 private constant _PAUSER_ROLE = keccak256("PAUSER_ROLE");
  bytes32 private constant _APPROVER_ROLE = keccak256("APPROVER_ROLE");
  bytes32 private constant _REQUESTER_ROLE = keccak256("REQUESTER_ROLE");
  //the minimum approvals needed
  uint256 public constant MINIMUM_APPROVALS = 2;
  //the minimum amount that can be requested
  uint256 public constant MINIMUM_REQUEST = 0.1 ether;
  //the divisor to determine which tier an amount should belong in
  uint256 public constant TIER_RATE = 1 ether;
  //the required amount of approvals needed per tier
  uint256 public constant TIER_APPROVALS = 1;
  //the cooldown increment per tier (1 day)
  uint256 public constant TIER_COOLDOWN = 86400;

  // ============ Storage ============

  //mapping of tx id to tx
  mapping(uint256 => TX) public txs;
  //mapping of tier to next
  mapping(uint256 => uint256) public next;
  //mapping of tx id to approver to approved
  mapping(uint256 => mapping(address => bool)) public approved;

  // ============ Deploy ============

  constructor(address admin) {
    //set up roles for the contract creator
    _setupRole(DEFAULT_ADMIN_ROLE, admin);
    _setupRole(_PAUSER_ROLE, admin);
  }

  /**
   * @dev The Ether received will be logged with {PaymentReceived} 
   * events. Note that these events are not fully reliable: it's 
   * possible for a contract to receive Ether without triggering this 
   * function. This only affects the reliability of the events, and not 
   * the actual splitting of Ether.
   *
   * To learn more about this see the Solidity documentation for
   * https://solidity.readthedocs.io/en/latest/contracts.html#fallback-function[fallback
   * functions].
   */
  receive() external payable {
    emit FundsReceived(_msgSender(), msg.value);
  }

  // ============ Read Methods ============

  /**
   * @dev Returns true if tx is approved
   */
  function isApproved(uint256 id) public view returns(bool) {
    (,uint256 required,) = tierInfo(txs[id]);
    return txs[id].approvals >= required;
  }

  /**
   * @dev Determine the tier information of a given amount
   */
  function tierInfo(TX memory transaction) public pure returns(
    uint256 tier,
    uint256 required,
    uint256 cooldown
  ) {
    return tierInfoByAmount(transaction.amount);
  }

  /**
   * @dev Determine the tier information of a given amount
   */
  function tierInfoByAmount(uint256 amount) public pure returns(
    uint256 tier,
    uint256 required,
    uint256 cooldown
  ) {
    if (amount < MINIMUM_REQUEST) {
      return (0, 0, 0);
    }
    
    tier = amount / TIER_RATE;
    cooldown = tier * TIER_COOLDOWN;
    required = tier * TIER_APPROVALS;
    if (required < MINIMUM_APPROVALS) {
      required = MINIMUM_APPROVALS;
    }
  }

  // ============ Write Methods ============

  /**
   * @dev Approves a transaction
   */
  function approve(uint256 id) public onlyRole(_APPROVER_ROLE) {
    if (paused()
      //check if tx exists
      || txs[id].amount == 0
      //check if cancelled
      || txs[id].cancelled
      //check if withdrawn
      || txs[id].withdrawn
    ) revert InvalidCall();

    address sender = _msgSender();
    //require approver didnt already approve
    if(approved[id][sender]) revert InvalidCall();
    //add to the approval
    txs[id].approvals += 1; 
    approved[id][sender] = true;

    //emit approved
    emit FundsApprovedFrom(sender, id);

    if (isApproved(id)) {
      //emit approved
      emit FundsApproved(id);
    }
  }

  /**
   * @dev Cancels a transaction request
   */
  function cancel(uint256 id) public onlyRole(_REQUESTER_ROLE) {
    if (paused()
      //check if tx exists
      || txs[id].amount == 0
      //check if cancelled
      || txs[id].cancelled
      //check if approvals
      || txs[id].approvals > 0
    ) revert InvalidCall();

    //okay cancel it
    txs[id].cancelled = true;
    //update the cooldown
    (uint256 tier,,) = tierInfo(txs[id]);
    next[tier] = uint64(block.timestamp);
    //emit cancelled
    emit RequestCancelled(id);
  }

  /**
   * @dev Pauses all token transfers.
   */
  function pause() external onlyRole(_PAUSER_ROLE) {
    _pause();
  }

  /**
   * @dev Makes a transaction request
   */
  function request(
    uint256 id, 
    address beneficiary, 
    uint256 amount
  ) public onlyRole(_REQUESTER_ROLE) {
    if (paused()
      //check if amount is more than the balance
      || amount > address(this).balance
      //check to see if tx exists
      || txs[id].amount > 0
      //check for valid address
      || beneficiary == address(0) 
      || beneficiary == address(this)
    ) revert InvalidCall();

    //what tier level is this?
    (uint256 tier,,uint256 cooldown) = tierInfo(txs[id]);
    //check to see if a tier is found
    if(tier == 0) revert InvalidCall();
    //get the time now
    uint64 timenow = uint64(block.timestamp);
    //the time should be greater than the last approved plus the cooldown
    if(timenow < next[tier]) revert InvalidCall();

    //create a new tx
    txs[id].amount = amount;
    txs[id].beneficiary = beneficiary;
    
    //emit funds requested before approved
    emit FundsRequested(id);

    //if this sender is also an approver
    if (hasRole(_APPROVER_ROLE, _msgSender())) {
      //then approve it
      approve(id);
    }

    //update the next time they can make a request
    next[tier] = timenow + cooldown;
  }

  /**
   * @dev Unpauses all token transfers.
   */
  function unpause() external onlyRole(_PAUSER_ROLE) {
    _unpause();
  }

  /**
   * @dev Allows transactions to be withdrawn
   */
  function withdraw(uint256 id) external nonReentrant {
    //check for approved funds
    if (txs[id].amount == 0
      //check to see if withdrawn already
      || txs[id].withdrawn
      //check if amount is less than the balance
      || txs[id].amount > address(this).balance
      //is it even approved?
      || !isApproved(id)
    ) revert InvalidCall();

    //go ahead and transfer it
    txs[id].withdrawn = true;
    Address.sendValue(payable(txs[id].beneficiary), txs[id].amount);

    //emit withdrawn
    emit FundsWithdrawn(id);
  }
}