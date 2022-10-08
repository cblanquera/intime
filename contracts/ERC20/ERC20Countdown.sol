// SPDX-License-Identifier: MIT
// OpenZeppelin Contracts (last updated v4.7.0) (token/ERC20/ERC20.sol)

pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/utils/Context.sol";

import "hardhat/console.sol";

/**
 * @dev Implementation of the IERC20 interface with countdown protocol.
 */
contract ERC20Countdown is Context, IERC20 {
  // ============ Errors ============

  error InvalidCall();
  error InvalidAllowance();
  error AmountExceedsBalance();
  
  // ============ Constants ============

  //solidity isnt accurate in math sometimes so we need
  //to store by the millisecond to make up for this issue
  uint256 internal constant _TO_MILLISEC = 1000;

  // ============ Storage ============

  //table of allowances between owners and operators
  mapping(address => mapping(address => uint256)) private _allowances;
  //mapping of addresses to their end time
  mapping(address => uint256) private _endTimes;
  //total number of open accounts
  uint256 private _openAccounts;
  //the sum of all end times
  uint256 private _totalTime;
  //the sum of all expired end times
  uint256 private _expiredTime;

  // ============ Read Methods ============

  /**
   * @dev Returns the remaining number of tokens that `spender` will be
   * allowed to spend on behalf of `owner` through {transferFrom}. This 
   * is zero by default.
   */
  function allowance(
    address owner, 
    address spender
  ) public view returns(uint256) {
    return _allowances[owner][spender];
  }

  /**
   * @dev Returns the amount of tokens owned by `account`.
   */
  function balanceOf(address account) public view returns(uint256) {
    uint256 timestamp = timeNow();
    unchecked {
      return _endTimes[account] > timestamp ? _endTimes[account] - timestamp: 0;
    }
  }

  /**
   * @dev Returns the time of an account. Allows negative returns.
   */
  function timeOf(address account) public view returns(int256) {
    //if they never started
    if (_endTimes[account] == 0) {
      return 0;
    }
    return int256(_endTimes[account]) - int256(timeNow());
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
  function totalSupply() public view returns(uint256) {
    uint256 timestamp = timeNow() * _openAccounts;
    unchecked {
      uint256 endings = _totalTime - _expiredTime;
      return endings > timestamp ? endings - timestamp: 0;
    }
  }

  // ============ Allowance Methods ============

  /**
   * @dev Sets `amount` as the allowance of `spender` 
   * over the caller's tokens.
   */
  function approve(
    address spender, 
    uint256 amount
  ) external returns(bool) {
    address owner = _msgSender();
    _approve(owner, spender, amount);
    return true;
  }

  /**
   * @dev Atomically decreases the allowance 
   * granted to `spender` by the caller.
   */
  function decreaseAllowance(
    address spender, 
    uint256 subtractedValue
  ) external returns(bool) {
    address owner = _msgSender();
    uint256 currentAllowance = allowance(owner, spender);
    if (currentAllowance < subtractedValue) revert InvalidAllowance();
    unchecked {
      _approve(owner, spender, currentAllowance - subtractedValue);
    }

    return true;
  }

  /**
   * @dev Atomically increases the allowance 
   * granted to `spender` by the caller.
   */
  function increaseAllowance(
    address spender, 
    uint256 addedValue
  ) external returns(bool) {
    address owner = _msgSender();
    _approve(owner, spender, allowance(owner, spender) + addedValue);
    return true;
  }
  
  /**
   * @dev Sets `amount` as the allowance of 
   * `spender` over the `owner` s tokens.
   */
  function _approve(
    address owner,
    address spender,
    uint256 amount
  ) internal {
    if (owner == address(0) 
      || spender == address(0)
    ) revert InvalidAllowance();

    _allowances[owner][spender] = amount;
    emit Approval(owner, spender, amount);
  }

  /**
   * @dev Updates `owner` s allowance for 
   * `spender` based on spent `amount`.
   */
  function _spendAllowance(
    address owner,
    address spender,
    uint256 amount
  ) internal {
    uint256 currentAllowance = allowance(owner, spender);
    if (currentAllowance != type(uint256).max) {
      if (currentAllowance < amount) revert InvalidAllowance();
      unchecked {
        _approve(owner, spender, currentAllowance - amount);
      }
    }
  }

  // ============ Transfer Methods ============

  /**
   * @dev Moves `amount` tokens from the caller's account to `to`.
   */
  function transfer(address to, uint256 amount) external returns(bool) {
    address owner = _msgSender();
    _transfer(owner, to, amount);
    return true;
  }

  /**
   * @dev Moves `amount` tokens from `from` to `to` using the
   * allowance mechanism. `amount` is then deducted from the caller's
   * allowance.
   */
  function transferFrom(
    address from,
    address to,
    uint256 amount
  ) external returns(bool) {
    address spender = _msgSender();
    _spendAllowance(from, spender, amount);
    _transfer(from, to, amount);
    return true;
  }

  /**
   * @dev Destroys `amount` tokens from `account`, 
   * reducing the total supply.
   */
  function _burn(address from, uint256 amount) internal {
    if (from == address(0)) revert InvalidCall();

    _beforeTokenTransfer(from, address(0), amount);

    if (balanceOf(from) < amount) revert AmountExceedsBalance();
    unchecked {
      _endTimes[from] -= amount;
    }
    _expiredTime += amount;

    emit Transfer(from, address(0), amount);

    _afterTokenTransfer(from, address(0), amount);
  }

  /**
   * @dev Creates `amount` tokens and assigns them to `account`,
   * increasing the total supply.
   */
  function _mint(address to, uint256 amount) internal {
    //revert if mint to zero address
    if (to == address(0)) revert InvalidCall();

    _beforeTokenTransfer(address(0), to, amount);

    //revert if account's time + amount is less than zero
    if ((timeOf(to) + int256(amount)) <= 0) revert InvalidCall();

    //only if they havent started
    if (_endTimes[to] == 0) {
      //if no more supply
      if (totalSupply() == 0) {
        //capture the total time
        _expiredTime = _totalTime;
        //reset open accounts
        _openAccounts = 0;
      }
      //map when they started
      uint256 timestamp = timeNow();
      _endTimes[to] = timestamp + amount;
      _totalTime += timestamp + amount;
      _openAccounts++;
    } else {
      _endTimes[to] += amount;
      _totalTime += amount;
    }
    
    emit Transfer(address(0), to, amount);

    _afterTokenTransfer(address(0), to, amount);
  }

  /**
   * @dev Moves `amount` of tokens from `from` to `to`.
   */
  function _transfer(
    address from,
    address to,
    uint256 amount
  ) internal {
    //revert if transfer from or to zero address
    if (from == address(0) || to == address(0)) revert InvalidCall();

    _beforeTokenTransfer(from, to, amount);

    //revert if account's time + amount is less than zero
    if ((timeOf(to) + int256(amount)) <= 0) revert InvalidCall();
    if (balanceOf(from) < amount) revert AmountExceedsBalance();

    //only if they havent started
    if (_endTimes[to] == 0) {
      //map when they started
      uint256 timestamp = timeNow();
      _endTimes[to] = timestamp + amount;
      _totalTime += timestamp;
      _openAccounts++;
    } else {
      _endTimes[to] += amount;
    }
    
    unchecked {
      _endTimes[from] -= amount;
    }

    emit Transfer(from, to, amount);

    _afterTokenTransfer(from, to, amount);
  }

  // ============ Placeholder Methods ============

  /**
   * @dev Hook that is called before any transfer of tokens. 
   * This includes minting and burning.
   */
  function _beforeTokenTransfer(
    address from,
    address to,
    uint256 amount
  ) internal {}

  /**
   * @dev Hook that is called after any transfer of tokens. 
   * This includes minting and burning.
   */
  function _afterTokenTransfer(
    address from,
    address to,
    uint256 amount
  ) internal {}
}
