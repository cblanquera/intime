// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

/**
 * @dev Required interface of an IERC721TransferFees compliant contract.
 */
interface IERC721TransferFees  {
  /**
   * @dev Returns the fee of a `recipient`
   */
  function feeOf(address recipient) external view returns(uint256);

  /**
   * @dev Returns total fees
   */
  function totalFees() external view returns(uint256);
}
