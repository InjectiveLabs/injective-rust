use injective_std_derive::CosmwasmExt;
/// GetVersionRequest is the request for the ABCI version.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.services.version.v1.GetVersionRequest")]
pub struct GetVersionRequest {}
/// GetVersionResponse contains the ABCI application version info.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.services.version.v1.GetVersionResponse")]
pub struct GetVersionResponse {
    /// The semantic version of the node software.
    #[prost(string, tag = "1")]
    pub node: ::prost::alloc::string::String,
    /// The version of ABCI used by the node.
    #[prost(string, tag = "2")]
    pub abci: ::prost::alloc::string::String,
    /// The version of the P2P protocol.
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub p2p: u64,
    /// The version of the block protocol.
    #[prost(uint64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub block: u64,
}
