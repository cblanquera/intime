if (process.env.BLOCKCHAIN_NETWORK != 'hardhat') {
  console.error('Exited testing with network:', process.env.BLOCKCHAIN_NETWORK)
  process.exit(1);
}

const { expect } = require('chai')

function role(name) {
  if (!name || name === 'DEFAULT_ADMIN_ROLE') {
    return '0x0000000000000000000000000000000000000000000000000000000000000000'
  }

  return '0x' + Buffer.from(ethers.utils.solidityKeccak256(['string'], [name]).slice(2), 'hex').toString('hex')
}

class Artifact {
  static get(name) {
    const artifact = new Artifact(name)
    return new Proxy(artifact, {
      get(target, method) {
        if (target[method]) return target[method]
        return async function(...args) {
          const Contract = await ethers.getContractFactory(target.name, artifact._admin.address);
          const instance = await Contract.attach(target.address);
          return await instance[method](...args)
        }
      }
    }) 
  }

  constructor(name) {
    this.name = name
  }

  owner(signer) {
    this.signer = signer
    return this
  }

  async deploy(...args) {
    const Factory = await ethers.getContractFactory(
      this.name, 
      this.signer
    )
    const contract = await Factory.deploy(...args)
    await contract.deployed()

    const instance = new Contract(this, contract)

    return new Proxy(instance, {
      get(target, method) {
        if (target[method]) return target[method]
        if (method === 'then') return undefined
        return async function(...args) {
          const Factory = await ethers.getContractFactory(
            target.name, 
            target.signer
          )
          const instance = await Factory.attach(target.address);
          return await instance[method](...args)
        }
      }
    })
  }
}

class Contract {
  constructor(artifact, contract) {
    this._name = artifact.name
    this._owner = artifact.signer
    this._signer = artifact.signer
    this._resource = contract
  }

  get address() {
    return this._resource.address
  }

  get name() {
    return this._name
  }

  get owner() {
    return this._owner
  }

  get resource() {
    return this._resource
  }

  get signer() {
    return this._signer
  }

  with(signer) {
    this._signer = signer
    return this 
  }
}

const signers = async (scope, labels = []) => {
  const signers = await ethers.getSigners()
  if (!scope && !labels.lengh) return signers
  if (Array.isArray(scope)) {
    labels = scope
    scope = null
  }
  const labeled = {}
  for (let i = 0; i < labels.length; i++) labeled[labels[i]] = signers[i]
  if (!scope) return labeled
  for (const label in labeled) {
    scope[label] = labeled[label]
  }
  return scope
}

module.exports = { expect, Artifact, Contract, role, signers }