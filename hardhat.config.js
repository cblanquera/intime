require('@nomiclabs/hardhat-etherscan');
require('@nomiclabs/hardhat-waffle');
require('@nomiclabs/hardhat-truffle5');
require('hardhat-contract-sizer');
require('hardhat-gas-reporter');
require('dotenv').config();

// This is a sample Hardhat task. To learn how to create your own go to
// https://hardhat.org/guides/create-task.html
task('accounts', 'Prints the list of accounts', async () => {
  const accounts = await ethers.getSigners();

  for (const account of accounts) {
    console.log(account.address);
  }
});

// You need to export an object to set up your config
// Go to https://hardhat.org/config/ to learn more

/**
 * @type import('hardhat/config').HardhatUserConfig
 */
module.exports = {
  defaultNetwork: process.env.BLOCKCHAIN_NETWORK,
  networks: {
    hardhat: {
      chainId: 1337,
      mining: {
        //set this to false if you want localhost to mimick a real blockchain
        auto: true,
        interval: 5000
      }
    },
    goerli: {
      url: 'https://goerli.infura.io/v3/2a7154bb1cf244d9a412d1925398058c',
      chainId: 5,
      scanner: 'https://goerli.etherscan.io',
      opensea: 'https://testnets.opensea.io',
      accounts: [process.env.BLOCKCHAIN_GOERLI_PRIVATE_KEY],
      contracts: {
        token: '0x87A59a8be11649d5464CB2c70e66E1bC0F1D985f',
        bank: '0x4f4Cd0f04c914DF9CF66A1Cf806E6104f99B1808'
      }
    },
    ethereum: {
      url: 'https://mainnet.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161',
      chainId: 1,
      scanner: 'https://etherscan.io',
      opensea: 'https://opensea.io',
      accounts: [process.env.BLOCKCHAIN_ETHEREUM_PRIVATE_KEY],
      contracts: {}
    },
  },
  solidity: {
    version: '0.8.9',
    settings: {
      optimizer: {
        enabled: true,
        runs: 200
      }
    }
  },
  paths: {
    sources: './contracts',
    tests: './tests',
    cache: './cache',
    artifacts: './artifacts'
  },
  mocha: {
    timeout: 20000
  },
  gasReporter: {
    currency: 'USD',
    coinmarketcap: process.env.BLOCKCHAIN_CMC_KEY,
    gasPrice: 50
  },
  etherscan: {
    // Your API key for Etherscan
    // Obtain one at https://etherscan.io/
    apiKey: process.env.BLOCKCHAIN_SCANNER_KEY
  },
  contractSizer: {
    //see: https://www.npmjs.com/package/hardhat-contract-sizer
    runOnCompile: true
  }
};
