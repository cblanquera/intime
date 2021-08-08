// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import "../ERC777.sol";
import "@openzeppelin/contracts/security/Pausable.sol";

/**
 * @dev ERC777 token with pausable token transfers, minting and burning.
 *
 * Useful for scenarios such as preventing trades until the end of an evaluation
 * period, or having an emergency switch for freezing all token transfers in the
 * event of a large bug.
 */
abstract contract ERC777Pausable is ERC777, Pausable {
  /**
   * @dev See {ERC777-_beforeTokenTransfer}.
   */
  function _beforeTokenTransfer(
    address operator,
    address from,
    address to,
    uint256 amount
  ) internal virtual override {
    super._beforeTokenTransfer(operator, from, to, amount);

    require(!paused(), "ERC777Pausable: token transfer while paused");
  }
}
