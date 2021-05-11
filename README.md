# web3-flashbots

This library is a fork of [rust-web3](https://github.com/tomusdrw/rust-web3), extended to support `mev-geth` RPC methods: `eth_callBundle` and `eth_sendBundle`

For more information about the added methods, see https://github.com/flashbots/mev-relay-js

## Example

```rust
#[tokio::main]
async fn main() -> web3::Result<()> {
    // Address & private key to sign flashbots bundle
    let address = "0x1111111111111111111111111111111111111111".parse().unwrap();
    let key = "1111111111111111111111111111111111111111111111111111111111111111".parse().unwrap();

    let transport = web3::transports::Flashbots::new("https://relay.flashbots.net", address, key)?;
    let web3 = web3::Web3::new(transport);

    // Send an empty bundle
    web3.flashbots().send_bundle(&[], None, None, None).await?;

    Ok(())
}
```
