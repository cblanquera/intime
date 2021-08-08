# SPEC: Proof of InTime

Proofs are designed to prevent the congestion of block entries in the
blockchain by simply making it slower to confirm blocks. First to find the
golden nonce wins! The lowest nonce that makes a block hash from the last
block that ends with `0000` is the golden nonce. This favors the best computer
specs that process nonce numbers faster.

Inspired by the movie [In Time](https://www.youtube.com/watch?v=6zB6wZKEObc),
`MIN` is a currency that automatically burns by the minute. You can earn `MIN`
for mining.

## Definitions

 - **Consumer Wallet** - A regular wallet used for consumption.
 - **Mining Wallet** - A consumer wallet linked to mining software.
 - **Time Bank** - A mining wallet usually with high amounts of time used to
 prevent their `MIN` from being burned.
 - **Cooling Period** - The time between a wallet being generated and ready
 for mining.

## Set Up

The mining software will generate a new mining wallet. There is a cooling
period of 1 week before the mining wallet can start mining and receive funds.
During this period, the software will download the blockchain and can verify
newly entered blocks. The cooling period is necessary to determine the
stability of the mining hardware.

> If the mining software goes offline for any reason, the cooling period
restarts

## Mining Wallet

A mining wallet is compensated while online *(1 minute is 1 MIN)*. Since `MIN`
is also burned every minute, this also acts like a time bank
*(like in the movie)* rewarding people that serve the network with token
stability. Additionally, for every block that is successfully mined, 10 years,
calculated by  `~(60 * 24 * 365 * 10) MIN` is awarded to the miner that first
finds the golden nonce.

> The Ethereum Network processes > ~6k blocks a day. In MIN, that's good for
219K years per year or 2.19k lifetimes

### Proof of Being Poor

Miners are encouraged to distribute their wealth to consumers in order to
increase their potential in winning blocks. In order to encourage that, mining
difficulty is relative to how much `MIN` a mining wallet has in that, the more a
mining wallet has, the longer it will take to mine a block. An example
difficulty table can be thought as the following outline.

 - 1-9 years = 2 zeros
 - 10-99 years = 3 zeros
 - 100-499 years = 4 zeros
 - 500-999 years = 5 zeros
 - 1000-1499 years = 6 zeros
 - 1500-1999 years = 7 zeros

> Note: Still need to vary the amounts

### Energy

Miners can set a maximum difficulty attempt in order to save energy.
When a mining wallet funds exceeds this threshold, it will merely act as a
time bank that holds funds and verifies new blocks.

### Transferring

A consumer wallet can transfer time to to a mining wallet keep it safe from
being burned however a mining wallet cannot transfer funds to any kind of
wallet while it's mining. When a mining wallet does it's first transfer out,
it is automatically unlinked from the software and treated as a consumer wallet.
A consumer wallet can never change into a mining wallet. The mining software
will need to generate a new wallet and wait for the cooling period in order
to start mining again.

## Considerations

An automatic burning token overtime is not ideal in scenarios that are built on
stability and shifts the paradigm from having value by having more to having
value by keeping what you already have. The following were considered when
writing this specification.

 - Time is meant to be lost and can only be stable by serving the network.
 - Miners can serve the environment by saving energy at their discretion.
 - Preference to poor miners over rich ones to encourage circulation.
 - Miners cannot purposely be poor in order to win more blocks.
 - Sending funds to competitors in order win blocks is not practical at scale.
 - Mining wallet rules exist to discourage stability, bad miners and bad actors.
 - Time can be infinitely created but with more will be burned with more
 consumer wallets in existence.

## Use Cases

The following describes use cases for this kind of proof.

### Revolving Miner Case

This is the regular scenario.

 1. I earn 10 years from mining
 2. I convert my mining into a consumer wallet
 3. I register a new mining wallet (I wait a week)
 4. I start trading with my consumer wallet
 5. I send funds to my mining wallet
 6. I'm out of funds, so I convert my mining into a consumer wallet...

### Collector Case

It serves the network ideally if everyone was a miner. Having a time bank is
useful for most people but requires to serve the network too.

 1. I setup a time bank
 2. I bought a year
 3. I sent it to my time bank
 4. I bought a year ...

### Traders Case

Most exchanges won't be able to support tokens that burned over time, but the
following is possible.

 1. I created a consumer wallet
 2. I bought a year
 3. I list 6 months for sale
 4. A month later I sold 5 months and I have zero balance

> Note: You can also list your time bank for sale and configure a smart contract
to unlink and transfer funds

### Consumer Case

 - I bought a year
 - I spent 10 months of `MIN` in 2 months
 - 2 months later I have zero balance
 - I bought a year...

# SPEC: ERC777 In Time Token

An ERC777 compliant token that automatically burns by the minute. Inspired by
the movie [In Time](https://www.youtube.com/watch?v=6zB6wZKEObc). The token
has the following concepts.

## Token Relative to Time Now

A wallet **MIN** balance is relative to the time now, meaning tokens get burned
by the minute.

 - For example if you just got minted tokens held them for a year and did
 nothing else, you will have zero **MIN** in the end.
 - For example if you received 25**MIN**, 25 minutes later you will have
 zero **MIN**.

## Trading

Tokens can be freely traded with any ethereum compliant wallet holders.
Exchanges that trade **MIN** would need to comply with the subtraction of
tokens per minute or the contract will error when transferring if the balances
are not the same.

# SPEC: ERC721 Moments

An ERC721 compliant Non-Fungible Token that allows you to own a piece of time.
