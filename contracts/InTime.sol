// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";

import "./ERC20/IERC20Adjustable.sol";
import "./ERC20/ERC20Countdown.sol";

// ============ Contract ============

contract InTime is 
  AccessControl, 
  ERC20Countdown, 
  IERC20Adjustable, 
  IERC20Metadata 
{
  // ============ Constants ============
  
  //all custom roles
  bytes32 private constant _MINTER_ROLE = keccak256("MINTER_ROLE");

  // ============ Deploy ============

  constructor(address admin) {
    //set up roles for the contract creator
    _setupRole(DEFAULT_ADMIN_ROLE, admin);
  }

  // ============ Read Methods ============

  /**
   * @dev Returns the number of decimals 
   * used to get its user representation.
   */
  function decimals() external pure returns(uint8) {
    return 3;
  }

  /**
   * @dev Returns the name of the token.
   */
  function name() external pure returns (string memory) {
    return "In Time";
  }

  /**
   * @dev Returns the symbol of the token, 
   * usually a shorter version of the name.
   */
  function symbol() external pure returns (string memory) {
    return "SEC";
  }

  // ============ Write Methods ============

  /**
   * @dev Destroys `amount` tokens from the caller.
   */
  function burn(uint256 amount) external {
    _burn(_msgSender(), amount);
  }

  /**
   * @dev Destroys `amount` tokens from `account`, deducting from the caller's
   * allowance.
   */
  function burnFrom(address account, uint256 amount) external {
    _spendAllowance(account, _msgSender(), amount);
    _burn(account, amount);
  }

  // ============ Admin Methods ============

  /**
   * @dev Creates `amount` new tokens for `to`.
   */
  function mint(
    address to, 
    uint256 amount
  ) external onlyRole(_MINTER_ROLE) {
    _mint(to, amount);
  }
}