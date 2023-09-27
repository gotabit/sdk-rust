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

    // Build a raw transaction. sign and broadcast it
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
        let acct_info = if acct.is_some() {
            acct.unwrap()
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

        // build a single sign
        let signer_info = SignerInfo {
            public_key: Some(sign_key.public_key().into()),
            mode_info: Some(ModeInfo {
                sum: Some(mode_info::Sum::Single(mode_info::Single {
                    mode: SignMode::Direct.into(),
                })),
            }),
            sequence: acct_info.0,
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
            account_number: acct_info.1,
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

    // Call wasm query. and serde json response
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
