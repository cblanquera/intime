//to run this:
// $ npx hardhat run scripts/2-deploy-bank.js

const hardhat = require('hardhat')

async function deploy(name, ...params) {
  //deploy the contract
  const ContractFactory = await hardhat.ethers.getContractFactory(name);
  const contract = await ContractFactory.deploy(...params);
  await contract.deployed();

  return contract;
}

function getRole(name) {
  if (!name || name === 'DEFAULT_ADMIN_ROLE') {
    return '0x0000000000000000000000000000000000000000000000000000000000000000';
  }

  return '0x' + Buffer.from(
    hardhat.ethers.utils.solidityKeccak256(['string'], [name]).slice(2)
    , 'hex'
  ).toString('hex');
}

async function main() {
  //get network and admin
  const network = hardhat.config.networks[hardhat.config.defaultNetwork]
  const token = { address: network.contracts.token }

  console.log('Deploying TimeBank ...')
  const bank = await deploy('TimeBank', token.address)

  console.log('')
  console.log('-----------------------------------')
  console.log('TimeBank deployed to:', bank.address)
  console.log(
    'npx hardhat verify --show-stack-traces --network',
    hardhat.config.defaultNetwork,
    bank.address,
    `"${token.address}"`
  )
  console.log('')
  console.log('-----------------------------------')
  console.log('Next Steps:')
  console.log('In InTime contract, grant MINTER_ROLE, BURNER_ROLE to TimeBank')
  console.log(` - ${network.scanner}/address/${token.address}#writeContract`)
  console.log(` - grantRole( ${getRole('MINTER_ROLE')}, ${bank.address} )`)
  console.log(` - grantRole( ${getRole('BURNER_ROLE')}, ${bank.address} )`)
  console.log('')
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().then(() => process.exit(0)).catch(error => {
  console.error(error)
  process.exit(1)
});