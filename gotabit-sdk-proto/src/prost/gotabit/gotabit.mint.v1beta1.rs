// @generated
/// Minter represents the minting state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Minter {
    /// epoch_provisions represent rewards for the current epoch.
    #[prost(string, tag="1")]
    pub epoch_provisions: ::prost::alloc::string::String,
}
/// DistributionProportions defines the distribution proportions of the minted
/// denom. In other words, defines which stakeholders will receive the minted
/// denoms and how much.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DistributionProportions {
    /// staking defines the proportion of the minted mint_denom that is to be
    /// allocated as staking rewards.
    #[prost(string, tag="1")]
    pub staking: ::prost::alloc::string::String,
    /// eco_fund_pool defines the proportion of the minted mint_denom that is
    /// to be allocated as pool incentives.
    #[prost(string, tag="2")]
    pub eco_fund_pool: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub developer_fund_pool: ::prost::alloc::string::String,
    /// community_pool defines the proportion of the minted mint_denom that is
    /// to be allocated to the community pool.
    #[prost(string, tag="4")]
    pub community_pool: ::prost::alloc::string::String,
}
/// Params holds parameters for the x/mint module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// mint_denom is the denom of the coin to mint.
    #[prost(string, tag="1")]
    pub mint_denom: ::prost::alloc::string::String,
    /// genesis_epoch_provisions epoch provisions from the first epoch.
    #[prost(string, tag="2")]
    pub genesis_epoch_provisions: ::prost::alloc::string::String,
    /// epoch_identifier mint epoch identifier e.g. (day, week).
    #[prost(string, tag="3")]
    pub epoch_identifier: ::prost::alloc::string::String,
    /// reduction_period_in_epochs the number of epochs it takes
    /// to reduce the rewards.
    #[prost(int64, tag="4")]
    pub reduction_period_in_epochs: i64,
    /// reduction_factor is the reduction multiplier to execute
    /// at the end of each period set by reduction_period_in_epochs.
    #[prost(string, tag="5")]
    pub reduction_factor: ::prost::alloc::string::String,
    /// distribution_proportions defines the distribution proportions of the minted
    /// denom. In other words, defines which stakeholders will receive the minted
    /// denoms and how much.
    #[prost(message, optional, tag="6")]
    pub distribution_proportions: ::core::option::Option<DistributionProportions>,
    /// minting_rewards_distribution_start_epoch start epoch to distribute minting
    /// rewards
    #[prost(int64, tag="7")]
    pub minting_rewards_distribution_start_epoch: i64,
    #[prost(string, tag="8")]
    pub eco_fund_pool_address: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub developer_fund_pool_address: ::prost::alloc::string::String,
}
/// GenesisState defines the mint module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// minter is an abstraction for holding current rewards information.
    #[prost(message, optional, tag="1")]
    pub minter: ::core::option::Option<Minter>,
    /// params defines all the paramaters of the mint module.
    #[prost(message, optional, tag="2")]
    pub params: ::core::option::Option<Params>,
    /// reduction_started_epoch is the first epoch in which the reduction of mint
    /// begins.
    #[prost(int64, tag="3")]
    pub reduction_started_epoch: i64,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {
}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryEpochProvisionsRequest is the request type for the
/// Query/EpochProvisions RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEpochProvisionsRequest {
}
/// QueryEpochProvisionsResponse is the response type for the
/// Query/EpochProvisions RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEpochProvisionsResponse {
    /// epoch_provisions is the current minting per epoch provisions value.
    #[prost(bytes="bytes", tag="1")]
    pub epoch_provisions: ::prost::bytes::Bytes,
}
include!("gotabit.mint.v1beta1.tonic.rs");
// @@protoc_insertion_point(module)