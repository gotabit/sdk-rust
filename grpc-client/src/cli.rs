use cosmrs::crypto::secp256k1::SigningKey;
use gotabit_sdk_proto::cosmos::auth::v1beta1::{BaseAccount, QueryAccountRequest};
use gotabit_sdk_proto::cosmos::base::v1beta1::Coin;
use gotabit_sdk_proto::cosmos::tx::signing::v1beta1::SignMode;
use gotabit_sdk_proto::cosmos::tx::v1beta1::mode_info::{self};
use gotabit_sdk_proto::cosmos::tx::v1beta1::{
    AuthInfo, BroadcastMode, BroadcastTxRequest, Fee, SignDoc, SignerInfo, Tip, TxBody, TxRaw,
};
use gotabit_sdk_proto::cosmos::tx::v1beta1::{BroadcastTxResponse, ModeInfo};
use gotabit_sdk_proto::cosmwasm::wasm::v1::QuerySmartContractStateRequest;
use std::error::Error;
use tonic::transport::Channel;

use gotabit_sdk_proto::cosmos::{
    app::v1alpha1::query_client::QueryClient as AppCli,
    auth::v1beta1::query_client::QueryClient as AuthCli,
    authz::v1beta1::query_client::QueryClient as AuthzCli,
    bank::v1beta1::query_client::QueryClient as BankCli,
    base::node::v1beta1::service_client::ServiceClient as BasenodeCli,
    base::tendermint::v1beta1::service_client::ServiceClient as TendermintCli,
    distribution::v1beta1::query_client::QueryClient as DistributionCli,
    evidence::v1beta1::query_client::QueryClient as EvidenceCli,
    feegrant::v1beta1::query_client::QueryClient as FeegrantCli,
    gov::v1::query_client::QueryClient as GovCli, group::v1::query_client::QueryClient as GroupCli,
    nft::v1beta1::query_client::QueryClient as NftCli,
    params::v1beta1::query_client::QueryClient as ParamsCli,
    slashing::v1beta1::query_client::QueryClient as SlashingCli,
    staking::v1beta1::query_client::QueryClient as StakingCli,
    tx::v1beta1::service_client::ServiceClient as TxCli,
    upgrade::v1beta1::query_client::QueryClient as UpgradeCli,
};

use gotabit_sdk_proto::gotabit::{
    epochs::query_client::QueryClient as EpochsCli,
    inbox::v1beta1::query_client::QueryClient as InboxCli,
    mint::v1beta1::query_client::QueryClient as MintCli,
};

use gotabit_sdk_proto::cosmwasm::wasm::v1::{
    msg_client::MsgClient as WasmdMsgCli, query_client::QueryClient as WasmdQueryCli,
};

use crate::networks::NetworkInfo;

use gotabit_sdk_proto::traits::MessageExt;

pub struct Cosmos {
    pub app: AppCli<Channel>,
    pub auth: AuthCli<Channel>,
    pub authz: AuthzCli<Channel>,
    pub bank: BankCli<Channel>,
    pub distribution: DistributionCli<Channel>,
    pub evidence: EvidenceCli<Channel>,
    pub feegrant: FeegrantCli<Channel>,
    pub gov: GovCli<Channel>,
    pub params: ParamsCli<Channel>,
    pub slashing: SlashingCli<Channel>,
    pub staking: StakingCli<Channel>,
    pub upgrade: UpgradeCli<Channel>,
    pub tx: TxCli<Channel>,
    pub nft: NftCli<Channel>,
    pub group: GroupCli<Channel>,
    pub tendermint: TendermintCli<Channel>,
    pub basenode: BasenodeCli<Channel>,
}

pub struct Gotabit {
    pub mint: MintCli<Channel>,
    pub epochs: EpochsCli<Channel>,
    pub inbox: InboxCli<Channel>,
}

pub struct Wasmd {
    pub wasmd_query: WasmdQueryCli<Channel>,
    pub wasmd_msg: WasmdMsgCli<Channel>,
}

pub struct StdClients {
    pub cosmos: Cosmos,
    pub gotabit: Gotabit,
    pub wasmd: Wasmd,
}

pub struct GrpcClient {
    _inner: tonic::client::Grpc<Channel>,
    pub chain_id: &'static str,
    pub clients: StdClients,
}

impl GrpcClient {
    /// Create a new instance of the Grpc client.
    ///
    /// # Arguments
    ///
    /// * `network` - The Gotabit network information for the client to connect to.
    ///
    /// # Errors
    ///
    /// * If the Grpc client fails to connect to the network.
    ///
    /// # Examples
    ///
    /// ```
    /// use grpc_client::{cli::GrpcClient, networks};
    /// use grpc_client::networks::NetworkInfo;
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = GrpcClient::new(&networks::TEST_NET).await.unwrap();
    /// }
    /// ```
    pub async fn new(network: &NetworkInfo) -> Result<Self, tonic::transport::Error> {
        let chan = tonic::transport::Channel::from_static(network.grpc_endpoint)
            .connect()
            .await?;

        let cosmos = Cosmos {
            app: AppCli::new(chan.clone()),
            auth: AuthCli::new(chan.clone()),
            authz: AuthzCli::new(chan.clone()),
            bank: BankCli::new(chan.clone()),
            distribution: DistributionCli::new(chan.clone()),
            evidence: EvidenceCli::new(chan.clone()),
            feegrant: FeegrantCli::new(chan.clone()),
            gov: GovCli::new(chan.clone()),
            params: ParamsCli::new(chan.clone()),
            slashing: SlashingCli::new(chan.clone()),
            staking: StakingCli::new(chan.clone()),
            upgrade: UpgradeCli::new(chan.clone()),
            tx: TxCli::new(chan.clone()),
            nft: NftCli::new(chan.clone()),
            group: GroupCli::new(chan.clone()),
            tendermint: TendermintCli::new(chan.clone()),
            basenode: BasenodeCli::new(chan.clone()),
        };

        let gotabit = Gotabit {
            mint: MintCli::new(chan.clone()),
            epochs: EpochsCli::new(chan.clone()),
            inbox: InboxCli::new(chan.clone()),
        };

        let wasmd = Wasmd {
            wasmd_query: WasmdQueryCli::new(chan.clone()),
            wasmd_msg: WasmdMsgCli::new(chan.clone()),
        };

        Ok(Self {
            _inner: tonic::client::Grpc::new(chan.clone()),
            chain_id: network.chain_id,
            clients: StdClients {
                cosmos,
                gotabit,
                wasmd,
            },
        })
    }

    /// Broadcast a transaction synchronously.
    ///
    /// # Arguments
    ///
    /// * `sign_key` - The signing key of the sender.
    /// * `addr` - The address of the sender.
    /// * `msg` - The message to be sent.
    /// * `coin` - The coin to be sent.
    /// * `memo` - The memo of the transaction.
    /// * `timeout_height` - The timeout height of the transaction.
    /// * `gas` - The gas limit of the transaction.
    /// * `acct` - The account number and sequence number of the sender. If set to None, it will query from the gRPC node.
    /// * `tx_tip` - The tip of the transaction.
    ///
    /// # Returns
    ///
    /// * `Result<BroadcastTxResponse, Error>` - The result of the broadcast transaction.
    ///
    /// # Examples
    ///
    /// ```
    /// use grpc_client::{cli::GrpcClient, networks};
    /// use cosmrs::crypto::secp256k1::SigningKey;
    /// use gotabit_sdk_proto::cosmos::base::v1beta1::Coin;
    /// use gotabit_sdk_proto::cosmos::tx::v1beta1::BroadcastTxResponse;
    /// use gotabit_sdk_proto::cosmos::bank::v1beta1::MsgSend;
    ///     /// ```

    /// #[tokio::main]
    /// async fn main() {
    ///     let mut client = GrpcClient::new(&networks::TEST_NET).await.unwrap();
    ///     let sign_key = SigningKey::from_slice(hex::decode("a823f4ae943b511b7f082227c4f2f0737666d625e7b3431f238a7d4d4baec5f2").unwrap().as_slice()).unwrap();
    ///     let addr = "my_address".to_string();
    ///     let msg_send = MsgSend {
    ///         from_address: "from_address".to_string(),
    ///         to_address: "to_address".to_string(),
    ///         amount: vec![],
    ///     };
    ///     let coin = Coin{amount: "1000000000000000000".to_string(), denom: "ugtb".to_string()};
    ///     let memo = "my_memo".to_string();
    ///     let timeout_height = 0;
    ///     let gas = 200000;
    ///     let acct = None;
    ///     let tx_tip = None;
    ///
    ///     let result = client.broadcast_tx_sync(&sign_key, &addr, msg_send, &coin, memo, timeout_height, gas, acct, tx_tip).await;
    /// }
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub async fn broadcast_tx_sync<T>(
        &mut self,
        sign_key: &SigningKey,
        addr: &String,
        msg: T,
        coin: &Coin,
        memo: String,
        timeout_height: u64,
        gas: u64,
        // (seq, account_number), if set none. it will query from grpc node
        acct: Option<(u64, u64)>,
        tx_tip: Option<Tip>,
    ) -> Result<tonic::Response<BroadcastTxResponse>, Box<dyn Error>>
    where
        T: gotabit_sdk_proto::traits::MessageExt + gotabit_sdk_proto::traits::TypeUrl,
    {
        let acct = if let Some(acct_info) = acct {
            acct_info
        } else {
            // get account seq
            let base_account_resp = self
                .clients
                .cosmos
                .auth
                .account(QueryAccountRequest {
                    address: addr.to_owned(),
                })
                .await?
                .into_inner();
            let base_acct: BaseAccount =
                BaseAccount::from_any(&base_account_resp.account.unwrap()).unwrap();
            (base_acct.sequence, base_acct.account_number)
        };
        // build a simple transfer transction and sign
        let tx_body = TxBody {
            messages: vec![msg.to_any().unwrap()],
            memo,
            timeout_height,
            extension_options: Default::default(),
            non_critical_extension_options: Default::default(),
        };

        let pubkey = sign_key.public_key().to_any().unwrap();
        // build a single sign
        let signer_info = SignerInfo {
            public_key: Some(prost_types::Any {
                type_url: pubkey.type_url,
                value: pubkey.value,
            }),
            mode_info: Some(ModeInfo {
                sum: Some(mode_info::Sum::Single(mode_info::Single {
                    mode: SignMode::Direct.into(),
                })),
            }),
            sequence: acct.0,
        };

        let auth_info = AuthInfo {
            signer_infos: vec![signer_info],
            fee: Some(Fee {
                amount: vec![coin.clone()],
                gas_limit: gas,
                payer: Default::default(),
                granter: Default::default(),
            }),
            tip: tx_tip,
        };

        let sign_doc = SignDoc {
            body_bytes: tx_body.to_bytes()?,
            auth_info_bytes: auth_info.to_bytes()?,
            chain_id: self.chain_id.to_string(),
            account_number: acct.1,
        };

        let sign_doc_bytes = sign_doc.to_bytes()?;
        let sign = sign_key.sign(&sign_doc_bytes)?;

        let tx_raw = TxRaw {
            body_bytes: sign_doc.body_bytes,
            auth_info_bytes: auth_info.to_bytes().unwrap(),
            signatures: vec![sign.to_vec()],
        };

        let resp = self
            .clients
            .cosmos
            .tx
            .broadcast_tx(BroadcastTxRequest {
                tx_bytes: tx_raw.to_bytes().unwrap(),
                mode: BroadcastMode::Sync.into(),
            })
            .await?;

        Ok(resp)
    }

    /// Query the contract state.
    ///
    /// # Arguments
    ///
    /// * `contract_addr` - The address of the contract to query.
    /// * `msg` - The query message to send to the contract.
    ///
    /// # Returns
    ///
    /// * `Result<R, Box<dyn Error>>` - The result of the query.
    pub async fn wasm_query<Q: serde::Serialize, R: serde::de::DeserializeOwned>(
        &mut self,
        contract_addr: impl Into<String>,
        msg: Q,
    ) -> Result<R, Box<dyn Error>> {
        let resp = self
            .clients
            .wasmd
            .wasmd_query
            .smart_contract_state(QuerySmartContractStateRequest {
                address: contract_addr.into(),
                query_data: serde_json::to_vec(&msg).unwrap(),
            })
            .await?
            .into_inner();

        let resp = serde_json::from_slice::<R>(&resp.data)?;

        Ok(resp)
    }
}
