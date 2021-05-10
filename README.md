# web3-flashbots (WIP)

This library is a fork of [rust-web3](https://github.com/tomusdrw/rust-web3), extended to support `mev-geth` RPC methods: `eth_callBundle` and `eth_sendBundle`

For more information about the added methods, see https://github.com/flashbots/mev-relay-js

## TODO

-   [ ] Flashbots transport

## Example

```rust
#[tokio::main]
async fn main() -> web3::Result<()> {
    let transport = web3::transports::Flashbots::new("https://relay.flashbots.net")?;
    let web3 = web3::Web3::new(transport);

    // WIP

    Ok(())
}
```
