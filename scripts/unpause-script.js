//to run this on testnet:
// $ npx hardhat run scripts/unpause-script.js --network mumbai

const hardhat = require('hardhat')

async function main() {
  await hre.run('compile');
  const NFT = await hardhat.ethers.getContractFactory('ERC721Marketplace');
  const nft = await NFT.attach(
    hardhat.config.networks[hardhat.config.defaultNetwork].contracts[0]
  )
  await nft.unpause()
  console.log('NFT contract unpaused')
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().then(() => process.exit(0)).catch(error => {
  console.error(error)
  process.exit(1)
})
