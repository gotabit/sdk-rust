// @generated
/// MsgSend defines a message for sending a message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSend {
    /// msg sender address
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    /// msg recipient address
    #[prost(string, tag="2")]
    pub to: ::prost::alloc::string::String,
    /// msg topics
    #[prost(string, tag="3")]
    pub topics: ::prost::alloc::string::String,
    /// msg message
    #[prost(string, tag="4")]
    pub message: ::prost::alloc::string::String,
}
/// MsgSendResponse defines the MsgSend response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSendResponse {
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
include!("gotabit.inbox.tonic.rs");
// @@protoc_insertion_point(module)