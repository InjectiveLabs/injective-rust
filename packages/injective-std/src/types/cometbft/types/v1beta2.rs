use injective_std_derive::CosmwasmExt;
/// ConsensusParams contains consensus critical parameters that determine the
/// validity of blocks.
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.types.v1beta2.ConsensusParams")]
pub struct ConsensusParams {
    #[prost(message, optional, tag = "1")]
    pub block: ::core::option::Option<BlockParams>,
    #[prost(message, optional, tag = "2")]
    pub evidence: ::core::option::Option<super::v1beta1::EvidenceParams>,
    #[prost(message, optional, tag = "3")]
    pub validator: ::core::option::Option<super::v1beta1::ValidatorParams>,
    #[prost(message, optional, tag = "4")]
    pub version: ::core::option::Option<super::v1beta1::VersionParams>,
}
/// BlockParams contains limits on the block size.
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.types.v1beta2.BlockParams")]
pub struct BlockParams {
    /// Max block size, in bytes.
    /// Note: must be greater than 0
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub max_bytes: i64,
    /// Max gas per block.
    /// Note: must be greater or equal to -1
    #[prost(int64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub max_gas: i64,
}
