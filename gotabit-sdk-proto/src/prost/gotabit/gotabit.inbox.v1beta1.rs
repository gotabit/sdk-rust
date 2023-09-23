// @generated
/// EventMsgSend defines the event for MsgSend.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventMsgSend {
    /// msg sender address
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    /// msg recipient address
    #[prost(string, tag="2")]
    pub receiver: ::prost::alloc::string::String,
    /// msg id
    #[prost(uint64, tag="3")]
    pub id: u64,
}
/// Msg defines the inbox item - msg
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Msg {
    /// msg id
    #[prost(uint64, tag="1")]
    pub id: u64,
    /// msg sender address
    #[prost(string, tag="2")]
    pub sender: ::prost::alloc::string::String,
    /// msg recipient address
    #[prost(string, tag="3")]
    pub to: ::prost::alloc::string::String,
    /// msg topics
    #[prost(string, tag="4")]
    pub topics: ::prost::alloc::string::String,
    /// msg message
    #[prost(string, tag="5")]
    pub message: ::prost::alloc::string::String,
}
/// GenesisState defines the inbox module's genesis state
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, repeated, tag="1")]
    pub messages: ::prost::alloc::vec::Vec<Msg>,
}
/// SentMessagesRequest is request type for the Query/SentMessages RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SentMessagesRequest {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
}
/// SentMessagesResponse is response type for the Query/SentMessages RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SentMessagesResponse {
    #[prost(message, repeated, tag="1")]
    pub messages: ::prost::alloc::vec::Vec<Msg>,
}
/// ReceivedMessagesRequest is request type for the Query/ReceivedMessages RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReceivedMessagesRequest {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub topics: ::prost::alloc::string::String,
}
/// ReceivedMessagesResponse is response type for the Query/ReceivedMessages RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReceivedMessagesResponse {
    #[prost(message, repeated, tag="1")]
    pub messages: ::prost::alloc::vec::Vec<Msg>,
}
include!("gotabit.inbox.v1beta1.tonic.rs");
// @@protoc_insertion_point(module)