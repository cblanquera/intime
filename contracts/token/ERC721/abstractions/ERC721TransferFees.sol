// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

//IERC721MultiClass interface
import "./../interfaces/IERC721TransferFees.sol";

/**
 * @dev Abstract extension of ERC721TransferFees that attaches royalty fees
 */
abstract contract ERC721TransferFees is IERC721TransferFees {
  //10000 means 100.00%
  uint256 private constant TOTAL_ALLOWABLE_FEES = 10000;
  //mapping of `classId` to total fees (could be problematic if not synced)
  uint256 private _fees;
  //mapping of `recipient` to fee
  mapping(address => uint256) private _fee;
  //index mapping of `classId` to recipients (so we can loop the map)
  address[] private _recipients;

  /**
   * @dev Returns the fee of a `recipient`
   */
  function feeOf(address recipient)
    public view override returns(uint256)
  {
    return _fee[recipient];
  }

  /**
   * @dev Returns total fees
   */
  function totalFees() public view override returns(uint256) {
    return _fees;
  }

  /**
   * @dev Sets a fee that will be collected during the exchange method
   */
  function _allocateFee(address recipient, uint256 fee) internal virtual {
    require(fee > 0, "ERC721TransferFees: Fee should be more than 0");

    //if no recipient
    if (_fee[recipient] == 0) {
      //add recipient
      _recipients.push(recipient);
      //map fee
      _fee[recipient] = fee;
      //add to total fee
      _fees += fee;
    //else there"s already an existing recipient
    } else {
      //remove old fee from total fee
      _fees -= _fee[recipient];
      //map fee
      _fee[recipient] = fee;
      //add to total fee
      _fees += fee;
    }

    //safe check
    require(
      _fees <= TOTAL_ALLOWABLE_FEES,
      "ERC721TransferFees: Exceeds allowable fees"
    );
  }

  /**
   * @dev Removes a fee
   */
  function _deallocateFee(address recipient) internal virtual {
    //this is for the benefit of the sender so they
    //dont have to pay gas on things that dont matter
    require(_fee[recipient] != 0, "ERC721TransferFees: Recipient has no fees");
    //deduct total fees
    _fees -= _fee[recipient];
    //remove fees from the map
    delete _fee[recipient];
    //Tricky logic to remove an element from an array...
    //if there are at least 2 elements in the array,
    if (_recipients.length > 1) {
      //find the recipient
      for (uint i = 0; i < _recipients.length; i++) {
        if(_recipients[i] == recipient) {
          //move the last element to the deleted element
          uint last = _recipients.length - 1;
          _recipients[i] = _recipients[last];
          break;
        }
      }
    }

    //either way remove the last element
    _recipients.pop();
  }

  /**
   * @dev Pays the amount to the recipients
   */
  function _escrowFees(uint256 amount)
    internal virtual returns(uint256)
  {
    //placeholder for recipient in the loop
    address recipient;
    //release payments to recipients
    for (uint i = 0; i < _recipients.length; i++) {
      //get the recipient
      recipient = _recipients[i];
      // (10 eth * 2000) / 10000 =
      payable(recipient).transfer(
        (amount * _fee[recipient]) / TOTAL_ALLOWABLE_FEES
      );
    }

    //determine the remaining fee percent
    uint256 remainingFee = TOTAL_ALLOWABLE_FEES - _fees;
    //return the remainder amount
    return (amount * remainingFee) / TOTAL_ALLOWABLE_FEES;
  }
}
