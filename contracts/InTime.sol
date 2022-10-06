// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

import "@openzeppelin/contracts/utils/Context.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";

// ============ Errors ============

error InvalidCall();

// ============ Contract ============

contract InTime is Context, AccessControl, ERC20 {
  // ============ Constants ============
  
  //all custom roles
  bytes32 private constant _MINTER_ROLE = keccak256("MINTER_ROLE");

  // ============ Storage ============

  //recipient count
  uint256 private _recipients;

  // ============ Deploy ============

  constructor(address admin) ERC20("In Time", "MIN") {
    //set up roles for the contract creator
    _setupRole(DEFAULT_ADMIN_ROLE, admin);
  }

  // ============ Read Methods ============

  /**
   * @dev Returns the amount of tokens owned by `account`.
   */
  function balanceOf(
    address account
  ) public view virtual override returns(uint256) {
    uint256 balance = super.balanceOf(account);

    if (balance < block.timestamp) {
      return 0;
    }

    return balance - block.timestamp;
  }

  /**
   * @dev Returns the amount of tokens in existence.
   */
  function totalSupply() public view virtual override returns(uint256) {
    uint256 epoch = block.timestamp * _recipients;
    uint256 supply = super.totalSupply();

    if (supply < epoch) {
      return 0;
    }

    return supply - epoch;
  }

  /**
   * @dev Returns the total recipients historically with time
   */
  function totalRecipients() public view returns(uint256) {
    return _recipients;
  }

  // ============ Write Methods ============

  /**
   * @dev Creates `amount` new tokens for `to`.
   */
  function mint(address to, uint256 amount) external onlyRole(_MINTER_ROLE) {
    if (balanceOf(to) == 0) {
      _recipients ++;
      _mint(to, block.timestamp + amount);
      return;
    }

    _mint(to, amount);
  }

  // ============ Internal Methods ============
}
