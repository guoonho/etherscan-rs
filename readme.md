# etherscan-rs

A simple wrapper for etherscan's api. 

## Config

Each command will require an Etherscan API key and your wallet address. You may
set this ahead of time as environment variables.

```
ETHERSCAN_APIKEY=<etherscan api key>
ETHERSCAN_WALLET=<your wallet address>
```

## Check balance

```
cargo run balance <ETHERSCAN_APIKEY> <ETHERSCAN_WALLET>
```

## Check ETH price (USD)

```
cargo run price <ETHERSCAN_APIKEY> <ETHERSCAN_WALLET>
```