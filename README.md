# In Time

Inspired by the movie [In Time](https://www.youtube.com/watch?v=6zB6wZKEObc),
`SEC` is a currency that automatically burns tokens by the second. You can earn
seconds throught the time faucet.

## Terminology

 - **Active Time** - Time that is automatically deflationary by the second
 - **Stable Time** - Time that is store in a Time Bank
 - **Open Account** - A wallet that has a balance in the Time Bank
 - **Closed Account** - A wallet that no longer has a balance in the Time Bank
 - **Negative Balance** - When a wallet runs out of time there's a negative balance that also increases by the second

## SPEC: ERC20 In Time Token

An ERC20 compliant In Time implementation of the countdown protocol. 
Tokens can be freely traded with any ERC20 compliant wallet holders.

 - Once you run out of time you can only gain more time if you can cover 
 the negative balance.

### Time Bank

A stable ERC20 version of In Time where you can store your time when 
you are not using it. When time is deposited here it will not burn.

 - You cannot transfer to other wallets
 - By depositing your active time, you convert it to stable time.
 - You cannot withdraw your stable time to a wallet that doesnt have  
 active time left, unless it covers the negative balance, even if it is  
 your account.
 - This is the token that will be used for exchanges

### Time Faucet

A place you can earn time. Earning time is also deflationary.