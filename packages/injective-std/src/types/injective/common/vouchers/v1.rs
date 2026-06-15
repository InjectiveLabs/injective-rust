use injective_std_derive::CosmwasmExt;
/// AddressVoucher pairs a bech32 address with its outstanding voucher coin.
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.common.vouchers.v1.AddressVoucher")]
pub struct AddressVoucher {
    /// The bech32 address of the voucher holder.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// The outstanding voucher coin.
    #[prost(message, optional, tag = "2")]
    pub voucher: ::core::option::Option<super::super::super::super::cosmos::base::v1beta1::Coin>,
}
