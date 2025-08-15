use injective_std_derive::CosmwasmExt;
/// PingRequest is a request to confirm that the connection is alive.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.rpc.grpc.v1beta1.RequestPing")]
pub struct RequestPing {}
/// RequestBroadcastTx is a request to broadcast the transaction.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.rpc.grpc.v1beta1.RequestBroadcastTx")]
pub struct RequestBroadcastTx {
    #[prost(bytes = "vec", tag = "1")]
    pub tx: ::prost::alloc::vec::Vec<u8>,
}
/// PingResponse is a response to confirm that the connection is alive.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.rpc.grpc.v1beta1.ResponsePing")]
pub struct ResponsePing {}
/// ResponseBroadcastTx is a response of broadcasting the transaction.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.rpc.grpc.v1beta1.ResponseBroadcastTx")]
pub struct ResponseBroadcastTx {
    #[prost(message, optional, tag = "1")]
    pub check_tx: ::core::option::Option<super::super::super::abci::v1beta1::ResponseCheckTx>,
    #[prost(message, optional, tag = "2")]
    pub deliver_tx: ::core::option::Option<super::super::super::abci::v1beta1::ResponseDeliverTx>,
}
