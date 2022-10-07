// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

import "./IERC20MintableBurnable.sol";

contract TimeBank is ERC20 {
  // ============ Errors ============

  error InvalidCall();
  error InvalidAmount();
  error InvalidTransfer();

  // ============ Constants ============

  //this is the contract address for Token
  IERC20MintableBurnable public immutable TIME;

  // ============ Depoy ============

  constructor(IERC20MintableBurnable time) ERC20("Time Bank", "SEC") {
    TIME = time;
  }

  // ============ Read Methods ============

  /**
   * @dev Returns the number of decimals used to get its user representation.
   * For example, if `decimals` equals `2`, a balance of `505` tokens should
   * be displayed to a user as `5.05` (`505 / 10 ** 2`).
   */
  function decimals() public pure override returns(uint8) {
    return 3;
  }

  // ============ Write Methods ============

  /**
   * @dev Allows anyone to deposit their `amount` of time.
   * This effectively burns that time and mints time that stands still here.
   */
  function deposit(uint256 amount) external {
    deposit(_msgSender(), amount);
  }

  /**
   * @dev Allows anyone to deposit their `amount` time to any `account`.
   * This effectively burns that time and mints time that stands still here.
   */
  function deposit(address account, uint256 amount) public {
    //burn some time. muhahahaha.
    TIME.burnFrom(_msgSender(), amount);
    //mint some time
    _mint(account, amount);
  }

  /**
   * @dev Allows anyone to withdraw their `amount` of time.
   * This effectively burns this time and mints deflationary time.
   */
  function withdraw(uint256 amount) external {
    withdraw(_msgSender(), amount);
  }

  /**
   * @dev Allows anyone to withdraw their `amount` of time and send to 
   * a `recipient`. This effectively burns this time and mints 
   * deflationary time.
   */
  function withdraw(address recipient, uint256 amount) public {
    address account = _msgSender();
    //burn some time
    _burn(account, amount);
    //make some time
    TIME.mint(recipient, amount);
  }

  // ============ Internal Methods ============

  /**
   * @dev Hook that is called before any transfer of tokens. This 
   * includes minting and burning.
   */
  function _beforeTokenTransfer(
    address from,
    address to,
    uint256
  ) internal virtual override {
    //revert if transferring 
    if (from != address(0) && to != address(0) ) revert InvalidTransfer();
  }
}