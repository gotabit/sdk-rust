use crate::{cli::GrpcClient, networks::TEST_NET};

use gotabit_sdk_proto::cosmos::bank::v1beta1::MsgSend;
use gotabit_sdk_proto::cosmos::base::v1beta1::Coin;

use cosmrs::crypto::secp256k1::SigningKey;
use gotabit_sdk_proto::cosmos::bank::v1beta1::QueryBalanceRequest;
use gotabit_sdk_proto::cosmwasm::wasm::v1::QueryContractInfoRequest;

const GIO_PREFIX: &'static str = "gio";

const TEST_SENDER_PRIV_KEY: &'static str =
    "a823f4ae943b511b7f082227c4f2f0737666d625e7b3431f238a7d4d4baec5f2";

#[cfg(test)]
#[tokio::test]
async fn test_create_grpc_client_and_query_balance() {
    let mut client = GrpcClient::new(&TEST_NET).await.unwrap();

    let balance_query = QueryBalanceRequest {
        address: "gio1fd6mm6stpe9akw3hxseaxzqrqej92phmflscwg".to_string(),
        denom: "ugtb".to_string(),
    };

    let balance = client
        .clients
        .cosmos
        .bank
        .balance(balance_query)
        .await
        .unwrap()
        .into_inner();

    assert!(balance.balance.is_some());
    assert_eq!(balance.balance.unwrap().denom, "ugtb".to_string());
}

#[tokio::test]
async fn test_submit_tx() {
    let sender_private_key =
        SigningKey::from_slice(hex::decode(TEST_SENDER_PRIV_KEY).unwrap().as_slice()).unwrap();

    //let sender_private_key = secp256k1::SigningKey::from_slice(&vec![]).unwrap();
    let sender_public_key = sender_private_key.public_key();
    let sender_account_id = sender_public_key.account_id(GIO_PREFIX).unwrap();

    let coin = Coin {
        amount: "100000".to_string(),
        denom: "ugtb".to_string(),
    };

    let msg_send = MsgSend {
        from_address: sender_account_id.to_string(),
        to_address: "gio1rrk39fglrfcy6akh5zv4ahr9lx9clx23n2zmhk".to_string(),
        amount: vec![coin.clone()],
    };

    // create grpc client by given address
    let mut client = GrpcClient::new(&TEST_NET).await.unwrap();

    let gas = 100_000u64;
    let timeout_height = 11358142;
    let memo = "example memo".to_string();

    let resp = client
        .broadcast_tx_sync(
            &sender_private_key,
            &sender_account_id.to_string(),
            msg_send,
            &coin,
            memo,
            timeout_height,
            gas,
            None,
            None,
        )
        .await
        .unwrap();

    let response_body = resp.into_inner().tx_response.unwrap();

    assert_eq!(response_body.code, 0);
}

#[tokio::test]
async fn test_wasm_query_contract_info() {
    let mut client = GrpcClient::new(&TEST_NET).await.unwrap();
    let resp = client
        .clients
        .wasmd
        .wasmd_query
        .contract_info(QueryContractInfoRequest {
            address: "gio1up07dctjqud4fns75cnpejr4frmjtddzsmwgcktlyxd4zekhwecqqa2zf3".to_string(),
        })
        .await
        .unwrap()
        .into_inner()
        .contract_info
        .unwrap();

    assert_eq!(
        resp.admin,
        "gio1rrk39fglrfcy6akh5zv4ahr9lx9clx23n2zmhk".to_string()
    );
}
