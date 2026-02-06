use injective_std_derive::CosmwasmExt;
/// ResponseBroadcastTx is a response of broadcasting the transaction.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.rpc.grpc.v1beta3.ResponseBroadcastTx")]
pub struct ResponseBroadcastTx {
    #[prost(message, optional, tag = "1")]
    pub check_tx: ::core::option::Option<super::super::super::abci::v1beta3::ResponseCheckTx>,
    #[prost(message, optional, tag = "2")]
    pub tx_result: ::core::option::Option<super::super::super::abci::v1beta3::ExecTxResult>,
}
