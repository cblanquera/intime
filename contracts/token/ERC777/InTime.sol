// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import "@openzeppelin/contracts/utils/Context.sol";

//implementation of ERC777 Token Standard
import "./ERC777.sol";
import "./extensions/ERC777Pausable.sol";

/**
 * @dev
 */
contract InTime is Context, ERC777, ERC777Pausable {
  //contract owner
  address private _admin;
  //recipient count
  uint256 private _recipients;

  modifier onlyAdmin {
    require(
      _msgSender() == _admin,
      "Time: Restricted method access to only the admin"
    );
    _;
  }

  /**
   * @dev Sets up the admin
   */
  constructor(
    string memory name,
    string memory symbol,
    address[] memory defaultOperators
  ) ERC777(name, symbol, defaultOperators) {
    _admin = _msgSender();
  }

  /**
   * @dev Returns the amount of tokens in existence.
   */
  function totalSupply() public view virtual override returns (uint256) {
    int256 supply = int256(super.totalSupply()) - ((int256(block.timestamp) / 60) * int256(_recipients));
    if (supply < 0) {
      return 0;
    }

    return uint256(supply);
  }

  /**
   * @dev See {IERC777-totalSupply}.
   */
  function totalRecipients() public view returns (uint256) {
    return _recipients;
  }

  /**
   * @dev Returns the amount of tokens owned by `account`.
   */
  function balanceOf(address account)
    public view virtual override returns (uint256)
  {
    int256 balance = int256(super.balanceOf(account)) - (int256(block.timestamp) / 60);
    if (balance < 0) {
      return 0;
    }

    return uint256(balance);
  }

  /**
   * @dev Creates 1 year of new tokens for `to`.
   */
  function mint(address to) public virtual onlyAdmin {
    if (balanceOf(to) == 0) {
      _recipients ++;
    }
    //min hours days
    _mint(to, block.timestamp + (60 * 24 * 256), "", "");
  }

  /**
   * @dev See {IERC777-send}.
   *
   * Also emits a {IERC20-Transfer} event for ERC20 compatibility.
   */
  function send(
    address recipient,
    uint256 amount,
    bytes memory data
  ) public virtual override {
    if (balanceOf(recipient) == 0) {
      _recipients ++;
    }
    super.send(recipient, amount, data);
    if (balanceOf(_msgSender()) == 0) {
      _recipients --;
    }
  }

  /**
   * @dev See {IERC20-transfer}.
   *
   * Unlike `send`, `recipient` is _not_ required to implement the {IERC777Recipient}
   * interface if it is a contract.
   *
   * Also emits a {Sent} event.
   */
  function transfer(address recipient, uint256 amount) public virtual override returns (bool) {
    if (balanceOf(recipient) == 0) {
      _recipients ++;
    }
    super.transfer(recipient, amount);
    if (balanceOf(_msgSender()) == 0) {
      _recipients --;
    }
    return true;
  }

  /**
   * @dev See {IERC777-burn}.
   *
   * Also emits a {IERC20-Transfer} event for ERC20 compatibility.
   */
  function burn(uint256 amount, bytes memory data) public virtual override {
    super.burn(amount, data);
    if (balanceOf(_msgSender()) == 0) {
      _recipients --;
    }
  }

  /**
   * @dev Pauses all token transfers.
   */
  function pause() public virtual onlyAdmin {
    _pause();
  }

  /**
   * @dev Unpauses all token transfers.
   */
  function unpause() public virtual onlyAdmin {
    _unpause();
  }

  function _beforeTokenTransfer(
    address operator,
    address from,
    address to,
    uint256 amount
  ) internal virtual override(ERC777, ERC777Pausable) {
    super._beforeTokenTransfer(operator, from, to, amount);
  }
}
