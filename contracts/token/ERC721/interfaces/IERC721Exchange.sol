// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

//ERC721 interface
import "@openzeppelin/contracts/token/ERC721/IERC721.sol";

/**
 * @dev Required interface of an IERC721Exchange compliant contract.
 */
interface IERC721Exchange is IERC721 {
  /**
   * @dev Emitted when `owner` books their `tokenId` to
   * be sold for `amount` in wei.
   */
  event Listed(
    address indexed owner,
    uint256 indexed tokenId,
    uint256 indexed amount
  );

  /**
   * @dev Emitted when `owner` removes their `tokenId` from the order book.
   */
  event Delisted(address indexed owner, uint256 indexed tokenId);

  /**
   * @dev Returns the amount a `tokenId` is being offered for.
   */
  function listingOf(uint256 tokenId) external view returns(uint256);

  /**
   * @dev Allows for a sender to exchange `tokenId` for the listed amount
   */
  function exchange(uint256 tokenId) external payable;
}
