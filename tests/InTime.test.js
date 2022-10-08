const { expect, Artifact, role, signers } = require('./utils')

const InTime = Artifact.get('InTime')

contract('InTime', _ => {
  before(async function () {
    await signers(this, [ 
      'admin', 
      'holder1', 
      'holder2', 
      'holder3', 
      'holder4', 
      'holder5', 
      'holder6' 
    ])
    const admin = this.admin.address
    this.token = await InTime.owner(admin).deploy(admin)
    await this.token.with(admin).grantRole(role('MINTER_ROLE'), admin)
  })

  it('Should mint', async function () {
    const { token, admin, holder1, holder2 } = this
    //mint holder1 100 seconds (we mint in milliseconds)
    await token.with(admin).mint(holder1.address, 100000)
    expect(await token.balanceOf(holder1.address)).to.equal(100000)
    expect(await token.totalSupply()).to.equal(100000)
    //mint holder1 another 100 seconds
    await token.mint(holder1.address, 100000)
    expect(await token.balanceOf(holder1.address)).to.equal(199000)
    expect(await token.totalSupply()).to.equal(199000)
    //mint holder2 200 seconds
    await token.mint(holder2.address, 200000)
    expect(await token.balanceOf(holder2.address)).to.equal(200000)
    expect(await token.totalSupply()).to.equal(398000)
  })

  it('Should fastforward 7 sec later', async function() {
    await ethers.provider.send('evm_increaseTime', [6]); 
    await ethers.provider.send('evm_mine');
  })

  it('Should decrease time', async function () {
    const { token, holder1, holder2 } = this
    
    expect(await token.balanceOf(holder1.address)).to.equal(192000)
    expect(await token.balanceOf(holder2.address)).to.equal(194000)
    expect(await token.totalSupply()).to.equal(386000)
  })

  it('Should fastforward 200 seconds later', async function() {
    await ethers.provider.send('evm_increaseTime', [199]); 
    await ethers.provider.send('evm_mine');
  })

  it('Should have no time', async function () {
    const { token, holder1, holder2 } = this
    
    expect(await token.balanceOf(holder1.address)).to.equal(0)
    expect(await token.balanceOf(holder2.address)).to.equal(0)
    expect(await token.totalSupply()).to.equal(0)

    await expect(//you only have 1 life
      token.mint(holder1.address, 100)
    ).to.be.revertedWith('InvalidCall()')

    await expect(//you only have 1 life
      token.mint(holder2.address, 100)
    ).to.be.revertedWith('InvalidCall()')
  })

  it('Should add time', async function () {
    const { token, holder3 } = this
    
    //mint holder3 100 sec
    await token.mint(holder3.address, 100000)
    expect(await token.balanceOf(holder3.address)).to.equal(100000)
    expect(await token.totalSupply()).to.equal(100000)
  })

  it('Should fastforward 50 seconds later', async function() {
    await ethers.provider.send('evm_increaseTime', [50]); 
    await ethers.provider.send('evm_mine');
  })

  it('Should add time', async function () {
    const { token, holder3, holder4 } = this

    expect(await token.balanceOf(holder3.address)).to.equal(50000)
    expect(await token.totalSupply()).to.equal(50000)
    
    //mint holder4 150 sec
    await token.mint(holder4.address, 150000)
    expect(await token.balanceOf(holder4.address)).to.equal(150000)
    expect(await token.totalSupply()).to.equal(199000)
  })

  it('Should fastforward 50 seconds later', async function() {
    await ethers.provider.send('evm_increaseTime', [49]); 
    await ethers.provider.send('evm_mine');
  })

  it('Should add time', async function () {
    const { token, holder3, holder4, holder5 } = this

    expect(await token.balanceOf(holder3.address)).to.equal(0)
    expect(await token.balanceOf(holder4.address)).to.equal(101000)
    expect(await token.totalSupply()).to.equal(101000)
    
    //mint holder5 100 sec
    await token.mint(holder5.address, 100000)
    expect(await token.balanceOf(holder5.address)).to.equal(100000)
    expect(await token.totalSupply()).to.equal(199000)
  })

  it('Should not transfer', async function () {
    const { token, holder3, holder5 } = this

    expect(await token.totalSupply()).to.equal(199000)

    await expect(//transferring to expired account
      token.with(holder5).transfer(holder3.address, 10)
    ).to.be.revertedWith('InvalidCall()')

    await expect(//no balance
      token.with(holder5).transfer(holder3.address, 101000)
    ).to.be.revertedWith('AmountExceedsBalance()')

    await expect(//no balance
      token.with(holder3).transfer(holder5.address, 10)
    ).to.be.revertedWith('AmountExceedsBalance()')

    expect(await token.totalSupply()).to.equal(190000)
  })

  it('Should transfer', async function () {
    const { token, holder4, holder5, holder6 } = this

    expect(await token.totalSupply()).to.equal(190000)
    await token.with(holder5).transfer(holder4.address, 20000)
    expect(await token.balanceOf(holder4.address)).to.equal(116000)
    expect(await token.balanceOf(holder5.address)).to.equal(76000)
    expect(await token.totalSupply()).to.equal(187000)

    await token.with(holder5).transfer(holder6.address, 20000)
    expect(await token.balanceOf(holder5.address)).to.equal(55000)
    expect(await token.balanceOf(holder6.address)).to.equal(20000)
    expect(await token.totalSupply()).to.equal(184000)
  })

  it('Should not burn', async function () {
    const { token, holder4 } = this

    expect(await token.totalSupply()).to.equal(184000)

    await expect(//no balance
      token.with(holder4).burn(200000)
    ).to.be.revertedWith('AmountExceedsBalance()')

    expect(await token.totalSupply()).to.equal(180000)
  })

  it('Should burn', async function () {
    const { token, holder4 } = this
    expect(await token.totalSupply()).to.equal(180000)
  
    await token.with(holder4).burn(20000)
    expect(await token.balanceOf(holder4.address)).to.equal(93000)
  
    expect(await token.totalSupply()).to.equal(156000)
  })

  it('Should restore account', async function () {
    const { token, admin, holder1 } = this

    expect(await token.totalSupply()).to.equal(156000)

    const deficit = await token.timeOf(holder1.address) // 118000
    
    await token.with(admin).mint(holder1.address, deficit.mul(-1).add(50000))
    expect(await token.balanceOf(holder1.address)).to.equal(49000)
    expect(await token.totalSupply()).to.equal(320000)
  })
})
