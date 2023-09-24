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
}
