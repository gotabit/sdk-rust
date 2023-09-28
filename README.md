# Gotabit rust sdk

Rust resources related to the Gotabit ecosystem

## Modules

### 1. gotabit-sdk-proto

GotaBit network Proto and gRPC definitions using [prost](https://github.com/tokio-rs/prost) and [tonic](https://github.com/hyperium/tonic)

### 2. grpc-client

GotaBit gRPC client interacts with gotabit nodes.

## Using gRPC client

### Build a simple transfer transaction using gotabit gRPC client

```rust
use cosmrs::crypto::secp256k1::SigningKey;
use gotabit_sdk_proto::cosmos::bank::v1beta1::MsgSend;
use gotabit_sdk_proto::cosmos::base::v1beta1::Coin;
use grpc_client::cli::GrpcClient;

#[tokio::main]
async fn main() {
    let mut grpc_cli = GrpcClient::new(&grpc_client::networks::TEST_NET)
        .await
        .unwrap();

    let sender = "gio1h4nr4eez50szxn6darhhqha8x8tv8zvfd5mhet";
    let sign_key = "a823f4ae943b511b7f082227c4f2f0737666d625e7b3431f238a7d4d4baec5f2";
    let sign_key = SigningKey::from_slice(hex::decode(sign_key).unwrap().as_slice()).unwrap();
    let to_addr = "RECIPIENT_ADDRESS";
    let amount = Coin {
        denom: "ugtb".to_string(),
        amount: "100000".to_string(),
    };

    let msg_send = MsgSend {
        from_address: sender.into(),
        to_address: to_addr.to_string(),
        amount: vec![amount.clone()],
    };

    let resp = grpc_cli
        .broadcast_tx_sync(
            &sign_key,
            &sender.to_string(),
            msg_send.clone(),
            &amount,
            "test memo".to_string(),
            999999999,
            100_000u64,
            None,
            None,
        )
        .await
        .unwrap()
        .into_inner();

    if resp.tx_response.as_ref().unwrap().code == 0 {
        println!(
            "Transfer response. status: {}. hash: {}",
            resp.tx_response.as_ref().unwrap().code,
            resp.tx_response.as_ref().unwrap().txhash
        );
    } else {
        println!("{:?}", resp.tx_response.unwrap());
    }
}
```
