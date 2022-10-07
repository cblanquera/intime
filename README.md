# In Time

Inspired by the movie [In Time](https://www.youtube.com/watch?v=6zB6wZKEObc),
`SEC` is a currency that automatically burns tokens by the second. You can earn
seconds throught the time faucet.

## SPEC: ERC20 In Time Token

An ERC20 compliant In Time implementation of the burning token protocol. 
Tokens can be freely traded with any ERC20 compliant wallet holders.

### Time Bank

A stable ERC20 version of In Time where you can store your time when 
you are not using it. When time is deposited here it will not burn.

 - You can only transfer to open bank accounts
 - By depositing your time, you actually convert your time to this token.
 - You cannot withdraw your time to an account that doesnt have active time left.
 - This is the token that will be used for exchanges

### Time Faucet

A place you can earn time.