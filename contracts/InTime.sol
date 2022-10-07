// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";

import "@openzeppelin/contracts/utils/Context.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";

// ============ Contract ============

contract InTime is Context, AccessControl, ERC20Burnable {
  // ============ Constants ============
  
  //all custom roles
  bytes32 private constant _MINTER_ROLE = keccak256("MINTER_ROLE");
  //solidity isnt accurate in math sometimes so we need
  //to store by the nano to make up for this issue
  uint256 private constant _TO_MILLISEC = 1000;

  // ============ Storage ============

  //the last active accounts count
  uint256 private _lastActiveAccounts;
  //the last burned supply 
  uint256 private _lastBurnedSupply;

  // ============ Deploy ============

  constructor(address admin) ERC20("In Time", "SEC") {
    //set up roles for the contract creator
    _setupRole(DEFAULT_ADMIN_ROLE, admin);
  }

  // ============ Read Methods ============

  /**
   * @dev Returns the amount of tokens owned by `account`.
   */
  function balanceOf(
    address account
  ) public view override returns(uint256) {
    uint256 balance = super.balanceOf(account);
    uint256 timestamp = timeNow();
    unchecked {
      return balance > timestamp ? balance - timestamp: 0;
    }
  }

  /**
   * @dev Returns the number of decimals used to get its user representation.
   * For example, if `decimals` equals `2`, a balance of `505` tokens should
   * be displayed to a user as `5.05` (`505 / 10 ** 2`).
   */
  function decimals() public pure override returns(uint8) {
    return 3;
  }

  /**
   * @dev Returns the amount of tokens owned by `account`.
   */
  function expiredOn(address account) public view returns(uint256) {
    uint256 balance = super.balanceOf(account);
    //if they never minted
    if (balance == 0) {
      //they also never expired
      return 0;
    }
    uint256 timestamp = timeNow();
    unchecked {
      return timestamp > balance ? timestamp - balance: 0;
    }
  }

  /**
   * @dev Returns true if the account is expired
   */
  function isExpired(address account) public view returns(bool) {
    return balanceOf(account) == 0 && super.balanceOf(account) > 0;
  }

  /**
   * @dev Returns the end of the `account`
   */
  function endOf(address account) public view returns(uint256) {
    return super.balanceOf(account);
  }

  /**
   * @dev Returns the blocktime now in milliseconds
   */
  function timeNow() public view returns(uint256) {
    return block.timestamp * _TO_MILLISEC;
  }

  /**
   * @dev Returns the amount of tokens in existence.
   */
  function totalSupply() public view override returns(uint256) {
    uint256 supply = super.totalSupply() - _lastBurnedSupply;
    uint256 active = timeNow() * _lastActiveAccounts;
    unchecked {
      return supply > active ? supply - active: 0;
    }
  }

  // ============ Admin Methods ============

  /**
   * @dev Creates `amount` new tokens for `to`.
   */
  function mint(
    address to, 
    uint256 milliSeconds
  ) external onlyRole(_MINTER_ROLE) {
    //if they have no balance
    if (balanceOf(to) == 0) {
      //require that they never had a balance (new account)
      //you only get one life...
      require(
        super.balanceOf(to) == 0
        //or what they are buying restores that account
        || milliSeconds > expiredOn(to), 
        "InTime: minting to expired account"
      );
      //if no more active accounts
      if (totalSupply() == 0) {
        _lastActiveAccounts = 0;
        _lastBurnedSupply = super.totalSupply();
      }
      //now we can welcome a new account
      _lastActiveAccounts ++;
      //set like what time it will expire
      milliSeconds += timeNow();
    }

    //extended time!
    _mint(to, milliSeconds);
  }

  // ============ Internal Methods ============

  /**
   * @dev Hook that is called before any transfer of tokens. This 
   * includes minting and burning.
   */
  function _beforeTokenTransfer(
    address from,
    address to,
    uint256 amount
  ) internal virtual override {
    //if they are not minting
    if (from != address(0)) {
      //require they have enough funds
      require(
        balanceOf(from) >= amount, 
        "InTime: transfer amount exceeds balance"
      );
    }
    //if they are not burning
    if (to != address(0)) {
      //require they are not transferring to an expired account
      require(!isExpired(to), "InTime: transfer to expired account");
    }
  }
}
