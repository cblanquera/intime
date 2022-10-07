const { expect, Artifact, role, signers } = require('./utils')

const InTime = Artifact.get('InTime')
const TimeBank = Artifact.get('TimeBank')

contract('InTime', _ => {
  before(async function () {
    await signers(this, [ 
      'admin', 
      'holder1', 
      'holder2', 
      'holder3', 
      'holder4' , 
      'holder5' 
    ])
    const admin = this.admin.address
    this.token = await InTime.owner(admin).deploy(admin)
    await this.token.with(admin).grantRole(role('MINTER_ROLE'), admin)
    this.bank = await TimeBank.owner(admin).deploy(this.token.address)
    await this.token.grantRole(role('MINTER_ROLE'), this.bank.address)
  })

  it('Should deposit', async function () {
    const { token, bank, admin, holder1 } = this
    //mint holder1 100 seconds (we mint in milliseconds)
    await token.with(admin).mint(holder1.address, 100000)
    expect(await token.balanceOf(holder1.address)).to.equal(100000)
    expect(await token.totalSupply()).to.equal(100000)
    //deposit holder1 90 seconds
    await token.with(holder1).approve(bank.address, 90000)
    await bank.with(holder1)['deposit(uint256)'](90000)
    expect(await token.balanceOf(holder1.address)).to.equal(8000)
    expect(await token.totalSupply()).to.equal(8000)
  })

  it('Should withdraw', async function () {
    const { token, bank, holder1 } = this
    //withdraw holder1 90 seconds
    await bank.with(holder1)['withdraw(uint256)'](90000)
    expect(await token.balanceOf(holder1.address)).to.equal(97000)
    expect(await token.totalSupply()).to.equal(97000)
  })

  it('Should deposit', async function () {
    const { token, bank, admin, holder2 } = this
    //mint holder2 100 seconds (we mint in milliseconds)
    await token.with(admin).mint(holder2.address, 100000)
    expect(await token.balanceOf(holder2.address)).to.equal(100000)
    expect(await token.totalSupply()).to.equal(196000)
    //deposit holder2 90 seconds
    await token.with(holder2).approve(bank.address, 90000)
    await bank.with(holder2)['deposit(uint256)'](90000)
    expect(await token.balanceOf(holder2.address)).to.equal(8000)
    expect(await token.totalSupply()).to.equal(102000)
  })

  it('Should partial withdraw', async function () {
    const { token, admin, bank, holder2, holder3 } = this
    //withdraw holder2 40 seconds
    await bank.with(holder2)['withdraw(uint256)'](40000)
    expect(await token.balanceOf(holder2.address)).to.equal(47000)
    expect(await token.totalSupply()).to.equal(140000)
    //mint holder3 100 seconds
    await token.with(admin).mint(holder3.address, 100000)
    expect(await token.balanceOf(holder3.address)).to.equal(100000)
    expect(await token.totalSupply()).to.equal(238000)
    //withdraw holder2 50 seconds
    await bank.with(holder2)['withdraw(uint256)'](50000)
    expect(await token.balanceOf(holder2.address)).to.equal(95000)
    expect(await token.totalSupply()).to.equal(285000)
  })

  it('Should not transfer', async function () {
    const { token, bank, holder3, holder4 } = this

    //deposit holder3 90 seconds
    await token.with(holder3).approve(bank.address, 90000)
    await bank.with(holder3)['deposit(uint256)'](90000)

    await expect(
      bank.with(holder3).transfer(holder4.address, 90000)
    ).to.be.revertedWith('InvalidTransfer()')
  })
})
