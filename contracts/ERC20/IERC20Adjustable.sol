// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

interface IERC20Adjustable {
  function burn(uint256 amount) external;
  function burnFrom(address account, uint256 amount) external;
  function mint(address to, uint256 amount) external;
}