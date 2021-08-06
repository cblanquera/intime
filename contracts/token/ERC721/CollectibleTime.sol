// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

//implementation of ERC721 Non-Fungible Token Standard
import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
//implementation of ERC721 where tokens can be irreversibly burned (destroyed).
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721Burnable.sol";
//implementation of ERC721 where transers can be paused
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721Pausable.sol";
//For verifying messages in lazyMint
import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
//Abstract that allows tokens to be listed and exchanged considering royalty fees
import "./abstractions/ERC721Exchange.sol";

contract CollectableTime is ERC721, ERC721Burnable, ERC721Pausable, ERC721Exchange {
  //in only the contract owner can add a fee
  address private _admin;

  modifier onlyAdmin {
    require(
      _msgSender() == _admin,
      "Time: Restricted method access to only the admin"
    );
    _;
  }

  /**
   * @dev Constructor function
   */
  constructor (string memory _name, string memory _symbol)
    ERC721(_name, _symbol)
  {
    _admin = _msgSender();
  }

  /**
   * @dev Sets a fee that will be collected during the exchange method
   */
  function allocate(address recipient, uint256 fee)
    external virtual onlyAdmin
  {
    _allocateFee(recipient, fee);
  }

  /**
   * @dev Mints now and transfers to `recipient`
   */
  function autoMint(address recipient)
    external virtual onlyAdmin
  {
    uint256 timestamp = block.timestamp;
    _safeMint(recipient, timestamp);
  }

  /**
   * @dev Removes a fee
   */
  function deallocate(address recipient)
    external virtual onlyAdmin
  {
    _deallocateFee(recipient);
  }

  /**
   * @dev Allows anyone to self mint a token
   */
  function lazyMint(uint256 tokenId, address recipient, bytes calldata proof)
    public virtual
  {
    //make sure the admin signed this off
    require(
      ECDSA.recover(
        ECDSA.toEthSignedMessageHash(
          keccak256(
            abi.encodePacked(tokenId, recipient)
          )
        ),
        proof
      ) == _admin,
      "Time: Invalid proof."
    );

    _safeMint(recipient, tokenId);
  }

  /**
   * @dev Mints `tokenId` and transfers to `recipient`
   */
  function mint(uint256 tokenId, address recipient)
    external virtual onlyAdmin
  {
    _safeMint(recipient, tokenId);
  }

  /**
   * @dev Lists `tokenId` on the order book for `amount` in wei.
   */
  function list(uint256 tokenId, uint256 amount) external virtual {
    _list(tokenId, amount);
  }

  /**
   * @dev Removes `tokenId` from the order book.
   */
  function delist(uint256 tokenId) external virtual {
    _delist(tokenId);
  }

  /**
   * @dev Allows for a sender to exchange `tokenId` for the listed amount
   */
  function exchange(uint256 tokenId) external virtual override payable {
    _exchange(tokenId, msg.value);
  }

  /**
   * @dev Pauses all token transfers.
   *
   * See {ERC721Pausable} and {Pausable-_pause}.
   *
   * Requirements:
   *
   * - the caller must have the `PAUSER_ROLE`.
   */
  function pause() public virtual onlyAdmin {
    _pause();
  }

  /**
   * @dev Returns the total supply of time
   */
  function totalSupply() external view returns(uint256) {
    return block.timestamp;
  }

  /**
   * @dev Unpauses all token transfers.
   *
   * See {ERC721Pausable} and {Pausable-_unpause}.
   *
   * Requirements:
   *
   * - the caller must have the `PAUSER_ROLE`.
   */
  function unpause() public virtual onlyAdmin {
    _unpause();
  }

  /**
   * @dev Resolves duplicate _beforeTokenTransfer method definition
   * between ERC721 and ERC721Pausable
   */
  function _beforeTokenTransfer(
    address from,
    address to,
    uint256 tokenId
  ) internal virtual override(ERC721, ERC721Pausable) {
    super._beforeTokenTransfer(from, to, tokenId);
  }
}
