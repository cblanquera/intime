# ERC721 Multi Class Token

Native ERC721 token that allows tokens to be categorized in classes.
ERC1155 is very similar to this type of token but the differences is how tokens
are handled. In ERC1155, a recipient and others could have multiple copies of
the same token ID. In ERC721, a unique token ID could only have one owner and in
an ERC721 multi class token a unique token ID is simply assigned to a token
class.

When comparing the two, ERC1155 token IDs are token classes with no unique
tokens inside of them. Instead it basically says:

 - `Jane owns 20 of token 1`,
 - `John owns 30 of token 1` and
 - `James owns 10 of token 2`.

If no one else owned tokens, this means that:

 - `token 1 has 2 owners and 50 copies` and
 - `token 2 has 1 owners and 10 copies`.

In an ERC721 multi-class contract using the same example:

 - `Jane owns token 1 to token 20`,
 - `John owns token 21 to token 50` and
 - `John owns token 51 to token 60`.
 - `Token 1 to token 50 are class 1 tokens` and
 - `Token 51 to token 60 are class 2 tokens`.

Based on the above examples ERC1155 can be thought as monopoly money and an
ERC721 multi-class can be thought as real money with serial numbers. Using an
ERC721 multi-class made it so much easier to attach a decentralized exchange.

Default extensions include the following.

 - **ERC721MultiClassData:** Ability for a class to have a reference *(like a URI)*
 - **IERC721MultiClassSize:** Ability for a class to have a fixed token assignment size
 - **ERC721MultiClassFees:** Ability for a class to have payment fees
 - **ERC721MultiClassExchange:** Ability for tokens to be listed and exchanged *(considering fees)*
 - **ERC721MultiClassDrop:** Ability for tokens in a class to be air dropped and redeemed

## Compatibility

 Solidity ^0.8.0

  - Recommended v0.8.4

## Preset Marketplace

The marketplace is the main contract that implements all the extensions.
Feel free to make your own version and use less or attach other ERC721
extensions to it. The following covers the methods defined in this contract

### Deploy to Blockchain

 - **name (string):** ex. GRYPH Streetwear
 - **symbol (string):** ex. GRYPH

```js
//load the factory
const NFT = await ethers.getContractFactory('ERC721Marketplace')
//deploy the contract
const nft = await NFT.deploy('GRYPH Streetwear', 'GRYPH')
//wait for it to be confirmed
await nft.deployed()

console.log('Contract deployed to:', nft.address)
```

#### Contract Owner Actions

Only the contract owner can do the following actions. These are included in the
project's API.

##### Register

Registers a new `classId` with max token assignment `size` that
references a `uri`.

> A class ID does not need to be registered to use it in other methods.

`register(uint256 classId, uint256 size, string memory uri)`

 - **classId** - arbitrary class ID *(Should be unique)*
 - **size** - Max allowance of tokens that can be assigned
 - **uri** - Any URI that gives more details of this token class

```js
await nft.register(100, 200, 'ipfs://abc123')
```

##### Allocate

Sets a `fee` that will be paid to the `recipient` when a token in a `classId`
is exchanged.

`allocate(uint256 classId, address recipient, uint256 fee)`

 - **classId** - arbitrary class ID
 - **recipient** - Wallet address to pay when a token of the given class is exchanged
 - **fee** - The percent of the amount to give where 10000 means 100.00%

```js
await nft.allocate(100, '0xabc123', 1000) // 1000 is 10.00%
```

##### Deallocate

Removes a `recipient` from the fee table of a `classId`.

`deallocate(uint256 classId, address recipient)`

 - **classId** - arbitrary class ID
 - **recipient** - Wallet address to remove from the fee table

```js
await nft.deallocate(100, '0xabc123')
```

##### Mint

Mints `tokenId`, classifies it as `classId` and transfers to `recipient`.

`mint(uint256 classId, uint256 tokenId, address recipient)`

 - **classId** - arbitrary class ID
 - **tokenId** - arbitrary token ID *(Should be unique)*
 - **recipient** - Wallet address to send the token to

```js
await nft.mint(100, 200 '0xabc123') // 1000 is 10.00%
```

##### Pause

Pauses all token transfers.

`pause()`

```js
await nft.pause()
```

##### Unpause

Unpauses all token transfers.

`unpause()`

```js
await nft.unpause()
```

#### Token Owner Actions

Only the token owner can do the following actions. These are not included in the
project's API and should be implemented on the client side.

#### Burn

Burns a token.

`burn(uint256 tokenId)`

 - **tokenId** - ex. 1

```js
await nft.burn(1)
```

##### List

Lists `tokenId` on the order book for `amount` in wei.

`list(uint256 tokenId, uint256 amount)`

 - **tokenId** - token ID caller already owns
 - **amount** - Amount in wei to sell it for

```js
await nft.list(200, ethers.utils.parseEther('1.5'))
```

##### Delist

Removes `tokenId` from the order book.

`delist(uint256 tokenId)`

 - **tokenId** - token ID caller already owns

```js
await nft.delist(200)
```

#### Public Actions

Anyone can do the following actions. These are not included in the
project's API and should be implemented on the client side.

##### Lazy Mint

Allows anyone to mint a token

`lazyMint(uint256 classId, uint256 tokenId, address recipient, bytes proof)`

 - **classId** - arbitrary class ID
 - **tokenId** - arbitrary token ID *(Should be unique)*
 - **recipient** - Wallet address to send the token to
 - **proof** - Signature the contract ownner must have signed

```js
await nft.lazyMint(100, 200, '0xabc123', '0xabc123')
```

##### Exchange

Allows for a sender to exchange `tokenId` for the listed amount

`exchange(uint256 tokenId) payable`

 - **tokenId** - token ID to purchase *(Should be listed)*

```js
await nft.delist(200)
```

#### Read Only Methods

Anyone can retrieve the following information from the contract.

##### Get a Token's Class

Returns the class given `tokenId`

`classOf(uint256 tokenId)`

```js
await nft.classOf(200) //--> 100
```

##### Get a Token Class' Reference

Returns the data of `classId`

`referenceOf(uint256 classId)`

```js
await nft.referenceOf(100) //--> ipfs://abc123
```

##### Get a Token Class' Max Size

Returns the total possible supply size of `classId`

`classSize(uint256 classId)`

```js
await nft.classSize(100) //--> 1000
```

##### Check if a Token Class is Full

Returns true if `classId` supply and size are equal

`classFilled(uint256 classId)`

```js
await nft.classFilled(100) //--> 1000
```

##### Check How Many Tokens Are in a Token Class

Returns the current supply size of `classId`

`classSupply(uint256 classId)`

```js
await nft.classSupply(100) //--> 1000
```

##### Get the Fee of a Person

Returns the fee of a `recipient` in `classId`

`classFeeOf(uint256 classId, address recipient)`

```js
await nft.classFeeOf(100, '0x63FC745B5309cE72921ED6dF48D4eAdddEB55f27') //--> 1000 (10.00%)
```

##### Get the Total Fees of a Token Class

Returns the fee of a `recipient` in `classId`

`classFees(uint256 classId)`

```js
await nft.classFees(100) //--> 1500 (15.00%)
```

##### Get a Token's Listing Price

Returns the amount *(in wei)* a `tokenId` is being offered for.

`listingOf(uint256 tokenId)`

```js
await nft.listingOf(200) //--> 1.5 ether
```

## Contributing to the Project

 - Manually assigning IDs is preferred over enumerating *(cheaper gas)*
 - Batch minting is discouraged *(impractical gas cost)*
 - Looping cannot be scaled *(unpredictable gas)*
 - Mappings are preferred over structs *(cheaper gas)*
