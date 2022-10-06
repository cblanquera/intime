// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/security/Pausable.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/utils/Address.sol";
import "@openzeppelin/contracts/utils/math/SafeMath.sol";

// ============ Errors ============

error InvalidAmount();

// ============ Inferfaces ============

interface IERC20Capped is IERC20 {
  function cap() external returns(uint256);
}

// ============ Contract ============

contract TokenSwap is
  AccessControl,
  ReentrancyGuard,
  Pausable
{
  using Address for address;
  using SafeMath for uint256;

  // ============ Events ============

  event ERC20Received(address indexed sender, uint256 amount);
  event ERC20Sent(address indexed recipient, uint256 amount);
  event DepositReceived(address from, uint256 amount);

  // ============ Constants ============

  bytes32 private constant _PAUSER_ROLE = keccak256("PAUSER_ROLE");
  bytes32 private constant _CURATOR_ROLE = keccak256("CURATOR_ROLE");

  //this is the contract address for Token
  IERC20Capped public immutable TOKEN;
  //this is the contract address for the Token treasury
  address public immutable TREASURY;
  //this is the token cap of Token
  uint256 public immutable TOKEN_CAP;
  //this is the initial token seed amount
  uint256 public immutable SEED;

  // ============ Store ============

  //where 5000 = 50.00%
  uint16 private _interest = 5000;
  //where 20000 = 200.00%
  uint16 private _sellFor = 20000;
  //where 5000 = 50.00%
  uint16 private _buyFor = 5000;

  // ============ Deploy ============

  constructor(
    IERC20Capped token, 
    address treasury,
    uint256 seed,
    address admin
  ) {
    //set up roles for the contract creator
    _setupRole(DEFAULT_ADMIN_ROLE, admin);
    _setupRole(_PAUSER_ROLE, admin);
    //set the Token addresses
    TOKEN = token;
    TREASURY = treasury;
    //set the token cap
    TOKEN_CAP = token.cap();
    //set the seed
    SEED = seed;
    //start paused
    _pause();
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
    emit DepositReceived(_msgSender(), msg.value);
  }

  // ============ Read Methods ============

  /**
   * @dev Returns the ether balance
   */
  function balanceEther() public view returns(uint256) {
    return address(this).balance;
  }

  /**
   * @dev Returns the Token token balance
   */
  function balanceToken() public view returns(uint256) {
    return TOKEN.balanceOf(address(this));
  }

  /**
   * @dev Returns the ether amount we are willing to buy Token for
   */
  function buyingFor(uint256 amount) external view returns(uint256) {
    return _buyingFor(amount, balanceEther());
  }

  /**
   * @dev Returns the ether amount we are willing to sell Token for
   */
  function sellingFor(uint256 amount) external view returns(uint256) {
    return _sellingFor(amount, balanceEther());
  }

  /**
   * @dev Returns the token value
   */
  function tokenValue() external view returns(uint256) {
    //TokenValue = eth balance / (cap + cap - token balance)
    return _tokenValue(balanceEther());
  }

  // ============ Write Methods ============

  /**
   * @dev Buys `amount` of Token 
   */
  function buy(
    address recipient, 
    uint256 amount
  ) external payable whenNotPaused nonReentrant {
    uint256 value = _sellingFor(amount, balanceEther() - msg.value);
    if (value == 0 
      || msg.value < value
      || balanceToken() < amount
    ) revert InvalidAmount();
    //we already received the ether
    //so just send the tokens
    SafeERC20.safeTransfer(TOKEN, recipient, amount);
    //send the interest
    Address.sendValue(
      payable(TREASURY),
      msg.value.mul(_interest).div(10000)
    );
    emit ERC20Sent(recipient, amount);
  }

  /**
   * @dev Sells `amount` of Token 
   */
  function sell(
    address recipient, 
    uint256 amount
  ) external whenNotPaused nonReentrant {
    //check allowance
    if(TOKEN.allowance(recipient, address(this)) < amount) 
      revert InvalidAmount();
    //send the ether
    Address.sendValue(payable(recipient), _buyingFor(amount, balanceEther()));
    //now accept the payment
    SafeERC20.safeTransferFrom(
      TOKEN, 
      recipient, 
      address(this), 
      amount
    );
    emit ERC20Received(recipient, amount);
  }

  // ============ Admin Methods ============

  /**
   * @dev Sets the buy for percent
   */
  function buyFor(uint16 percent) external onlyRole(_CURATOR_ROLE) {
    _buyFor = percent;
  }

  /**
   * @dev Sets the interest
   */
  function interest(uint16 percent) external onlyRole(_CURATOR_ROLE) {
    _interest = percent;
  }

  /**
   * @dev Pauses all token transfers.
   */
  function pause() external onlyRole(_PAUSER_ROLE) {
    _pause();
  }

  /**
   * @dev Sets the sell for percent
   */
  function sellFor(uint16 percent) external onlyRole(_CURATOR_ROLE) {
    _sellFor = percent;
  }

  /**
   * @dev Unpauses all token transfers.
   */
  function unpause() external onlyRole(_PAUSER_ROLE) {
    _unpause();
  }

  // ============ Internal Methods ============

  /**
   * @dev Returns the token value
   */
  function _tokenValue(
    uint256 ethBalance
  ) private view returns(uint256) {
    //TokenValue = eth balance / (cap + cap - token balance)
    return ethBalance.mul(1 ether).div(
      TOKEN_CAP.add(SEED).sub(balanceToken())
    );
  }
  
  /**
   * @dev Returns the ether amount we are willing to buy Token for
   */
  function _buyingFor(
    uint256 amount, 
    uint256 ethBalance
  ) private view returns(uint256) {
    //TokenValue * percent * amount 
    return _tokenValue(ethBalance)
      .mul(amount)
      .mul(_buyFor)
      .div(10000)
      .div(1 ether);
  }

  /**
   * @dev Returns the ether amount we are willing to sell Token for
   */
  function _sellingFor(
    uint256 amount, 
    uint256 ethBalance
  ) private view returns(uint256) {
    //TokenValue * percent * amount 
    return _tokenValue(ethBalance)
      .mul(amount)
      .mul(_sellFor)
      .div(10000)
      .div(1 ether);
  }
}