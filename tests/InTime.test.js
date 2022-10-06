const { expect, Artifact, role, signers } = require('./utils')

const InTime = Artifact.get('InTime')

contract('InTime', _ => {
  before(async function () {
    await signers(this, [ 'admin', 'holder1', 'holder2' ])
    const admin = this.admin.address
    this.token = await InTime.owner(admin).deploy(admin)
    await this.token.with(admin).grantRole(role('MINTER_ROLE'), admin)
  })

  it('Should mint', async function () {
    const { token, admin, holder1, holder2 } = this
    //mint holder1 100 tokens
    await token.with(admin).mint(holder1.address, 100)
    expect(await token.balanceOf(holder1.address)).to.equal(100)
    expect(await token.totalSupply()).to.equal(100)
    //mint holder1 another 100 tokens
    await token.mint(holder1.address, 100)
    expect(await token.balanceOf(holder1.address)).to.equal(199)
    expect(await token.totalSupply()).to.equal(199)
    //mint holder2 200 tokens
    await token.mint(holder2.address, 200)
    expect(await token.balanceOf(holder2.address)).to.equal(200)
    expect(await token.totalSupply()).to.equal(398)
  })

  it('Should fastforward 7 sec later', async function() {
    await ethers.provider.send('evm_increaseTime', [6]); 
    await ethers.provider.send('evm_mine');
  })

  it('Should decrease time', async function () {
    const { token, holder1, holder2 } = this
    
    expect(await token.balanceOf(holder1.address)).to.equal(192)
    expect(await token.balanceOf(holder2.address)).to.equal(194)
    expect(await token.totalSupply()).to.equal(386)
  })

  it('Should fastforward 200 sec later', async function() {
    await ethers.provider.send('evm_increaseTime', [200]); 
    await ethers.provider.send('evm_mine');
  })

  it('Should have no time', async function () {
    const { token, holder1, holder2 } = this
    
    expect(await token.balanceOf(holder1.address)).to.equal(0)
    expect(await token.balanceOf(holder2.address)).to.equal(0)
    expect(await token.totalSupply()).to.equal(0)
  })
})
