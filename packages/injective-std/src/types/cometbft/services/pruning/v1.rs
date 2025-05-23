use injective_std_derive::CosmwasmExt;
/// SetBlockRetainHeightRequest sets the retain height for blocks.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.services.pruning.v1.SetBlockRetainHeightRequest")]
pub struct SetBlockRetainHeightRequest {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: u64,
}
/// SetBlockRetainHeightResponse is empty.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.services.pruning.v1.SetBlockRetainHeightResponse")]
pub struct SetBlockRetainHeightResponse {}
/// GetBlockRetainHeightRequest is a request for the retain height.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.services.pruning.v1.GetBlockRetainHeightRequest")]
pub struct GetBlockRetainHeightRequest {}
/// GetBlockRetainHeightResponse returns the retain height for blocks.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.services.pruning.v1.GetBlockRetainHeightResponse")]
pub struct GetBlockRetainHeightResponse {
    /// The retain height set by the application.
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub app_retain_height: u64,
    /// The retain height set via the pruning service (e.g. by the data
    /// companion) specifically for blocks.
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pruning_service_retain_height: u64,
}
/// SetBlockResultsRetainHeightRequest sets the retain height for block results.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.services.pruning.v1.SetBlockResultsRetainHeightRequest")]
pub struct SetBlockResultsRetainHeightRequest {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: u64,
}
/// SetBlockResultsRetainHeightResponse is empty.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.services.pruning.v1.SetBlockResultsRetainHeightResponse")]
pub struct SetBlockResultsRetainHeightResponse {}
/// GetBlockResultsRetainHeightRequest is a request for the retain height.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.services.pruning.v1.GetBlockResultsRetainHeightRequest")]
pub struct GetBlockResultsRetainHeightRequest {}
/// GetBlockResultsRetainHeightResponse returns the retain height for block results.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.services.pruning.v1.GetBlockResultsRetainHeightResponse")]
pub struct GetBlockResultsRetainHeightResponse {
    /// The retain height set by the pruning service (e.g. by the data
    /// companion) specifically for block results.
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pruning_service_retain_height: u64,
}
/// SetTxIndexerRetainHeightRequest sets the retain height for the tx indexer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.services.pruning.v1.SetTxIndexerRetainHeightRequest")]
pub struct SetTxIndexerRetainHeightRequest {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: u64,
}
/// SetTxIndexerRetainHeightResponse is empty.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.services.pruning.v1.SetTxIndexerRetainHeightResponse")]
pub struct SetTxIndexerRetainHeightResponse {}
/// GetTxIndexerRetainHeightRequest is a request for the retain height.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.services.pruning.v1.GetTxIndexerRetainHeightRequest")]
pub struct GetTxIndexerRetainHeightRequest {}
/// GetTxIndexerRetainHeightResponse returns the retain height for the tx indexer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.services.pruning.v1.GetTxIndexerRetainHeightResponse")]
pub struct GetTxIndexerRetainHeightResponse {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: u64,
}
/// SetBlockIndexerRetainHeightRequest sets the retain height for the block indexer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.services.pruning.v1.SetBlockIndexerRetainHeightRequest")]
pub struct SetBlockIndexerRetainHeightRequest {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: u64,
}
/// SetBlockIndexerRetainHeightResponse is empty.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.services.pruning.v1.SetBlockIndexerRetainHeightResponse")]
pub struct SetBlockIndexerRetainHeightResponse {}
/// GetBlockIndexerRetainHeightRequest is a request for the retain height.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.services.pruning.v1.GetBlockIndexerRetainHeightRequest")]
pub struct GetBlockIndexerRetainHeightRequest {}
/// GetBlockIndexerRetainHeightResponse returns the retain height for the block indexer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.services.pruning.v1.GetBlockIndexerRetainHeightResponse")]
pub struct GetBlockIndexerRetainHeightResponse {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: u64,
}
