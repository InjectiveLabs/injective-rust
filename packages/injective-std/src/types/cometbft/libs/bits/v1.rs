use injective_std_derive::CosmwasmExt;
/// BitArray is an array of bits.
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.libs.bits.v1.BitArray")]
pub struct BitArray {
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub bits: i64,
    #[prost(uint64, repeated, tag = "2")]
    pub elems: ::prost::alloc::vec::Vec<u64>,
}
