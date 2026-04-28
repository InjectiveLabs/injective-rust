use injective_std_derive::CosmwasmExt;
/// TokenPair defines an association of bank denom <-> EVM token (erc20 contract
/// address)
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.erc20.v1beta1.TokenPair")]
pub struct TokenPair {
    /// bank denom
    #[prost(string, tag = "1")]
    pub bank_denom: ::prost::alloc::string::String,
    /// address of erc20 smart contract that is backed by
    #[prost(string, tag = "2")]
    pub erc20_address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.erc20.v1beta1.EventCreateTokenPair")]
pub struct EventCreateTokenPair {
    #[prost(string, tag = "1")]
    pub bank_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub erc20_address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.erc20.v1beta1.EventDeleteTokenPair")]
pub struct EventDeleteTokenPair {
    #[prost(string, tag = "1")]
    pub bank_denom: ::prost::alloc::string::String,
}
/// Params defines the parameters for the erc20 module.
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.erc20.v1beta1.Params")]
pub struct Params {
    #[prost(message, optional, tag = "1")]
    pub denom_creation_fee: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// GenesisState defines the erc20 module's genesis state.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.erc20.v1beta1.GenesisState")]
pub struct GenesisState {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag = "2")]
    pub token_pairs: ::prost::alloc::vec::Vec<TokenPair>,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.erc20.v1beta1.QueryParamsRequest")]
#[proto_query(
    path = "/injective.erc20.v1beta1.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.erc20.v1beta1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryAllTokenPairsRequest is the request type for the Query/AllTokenPairs RPC
/// method.
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.erc20.v1beta1.QueryAllTokenPairsRequest")]
#[proto_query(
    path = "/injective.erc20.v1beta1.Query/AllTokenPairs",
    response_type = QueryAllTokenPairsResponse
)]
pub struct QueryAllTokenPairsRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryAllTokenPairsResponse is the response type for the Query/AllTokenPairs
/// RPC method.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.erc20.v1beta1.QueryAllTokenPairsResponse")]
pub struct QueryAllTokenPairsResponse {
    #[prost(message, repeated, tag = "1")]
    pub token_pairs: ::prost::alloc::vec::Vec<TokenPair>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryTokenPairByDenomRequest is the request type for the
/// Query/TokenPairByDenom RPC method.
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.erc20.v1beta1.QueryTokenPairByDenomRequest")]
#[proto_query(
    path = "/injective.erc20.v1beta1.Query/TokenPairByDenom",
    response_type = QueryTokenPairByDenomResponse
)]
pub struct QueryTokenPairByDenomRequest {
    #[prost(string, tag = "1")]
    pub bank_denom: ::prost::alloc::string::String,
}
/// QueryTokenPairByDenomResponse is the response type for the
/// Query/TokenPairByDenom RPC method.
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.erc20.v1beta1.QueryTokenPairByDenomResponse")]
pub struct QueryTokenPairByDenomResponse {
    #[prost(message, optional, tag = "1")]
    pub token_pair: ::core::option::Option<TokenPair>,
}
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.erc20.v1beta1.QueryTokenPairByERC20AddressRequest")]
#[proto_query(
    path = "/injective.erc20.v1beta1.Query/TokenPairByERC20Address",
    response_type = QueryTokenPairByErc20AddressResponse
)]
pub struct QueryTokenPairByErc20AddressRequest {
    #[prost(string, tag = "1")]
    pub erc20_address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.erc20.v1beta1.QueryTokenPairByERC20AddressResponse")]
pub struct QueryTokenPairByErc20AddressResponse {
    #[prost(message, optional, tag = "1")]
    pub token_pair: ::core::option::Option<TokenPair>,
}
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.erc20.v1beta1.MsgUpdateParams")]
pub struct MsgUpdateParams {
    /// authority is the address of the governance account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the erc20 parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.erc20.v1beta1.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.erc20.v1beta1.MsgCreateTokenPair")]
pub struct MsgCreateTokenPair {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub token_pair: ::core::option::Option<TokenPair>,
}
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.erc20.v1beta1.MsgCreateTokenPairResponse")]
pub struct MsgCreateTokenPairResponse {
    #[prost(message, optional, tag = "1")]
    pub token_pair: ::core::option::Option<TokenPair>,
}
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.erc20.v1beta1.MsgDeleteTokenPair")]
pub struct MsgDeleteTokenPair {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// bank denom of the pair to be deleted
    #[prost(string, tag = "2")]
    pub bank_denom: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.erc20.v1beta1.MsgDeleteTokenPairResponse")]
pub struct MsgDeleteTokenPairResponse {}
pub struct Erc20Querier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> Erc20Querier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        let request = QueryParamsRequest {};
        self.querier.query::<QueryParamsResponse>(&cosmwasm_std::QueryRequest::<Q>::Stargate {
            path: "/injective.erc20.v1beta1.Query/Params".to_string(),
            data: request.into(),
        })
    }
    pub fn all_token_pairs(
        &self,
        pagination: ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
    ) -> Result<QueryAllTokenPairsResponse, cosmwasm_std::StdError> {
        let request = QueryAllTokenPairsRequest { pagination };
        self.querier
            .query::<QueryAllTokenPairsResponse>(&cosmwasm_std::QueryRequest::<Q>::Stargate {
                path: "/injective.erc20.v1beta1.Query/AllTokenPairs".to_string(),
                data: request.into(),
            })
    }
    pub fn token_pair_by_denom(&self, bank_denom: ::prost::alloc::string::String) -> Result<QueryTokenPairByDenomResponse, cosmwasm_std::StdError> {
        let request = QueryTokenPairByDenomRequest { bank_denom };
        self.querier
            .query::<QueryTokenPairByDenomResponse>(&cosmwasm_std::QueryRequest::<Q>::Stargate {
                path: "/injective.erc20.v1beta1.Query/TokenPairByDenom".to_string(),
                data: request.into(),
            })
    }
    pub fn token_pair_by_erc20_address(
        &self,
        erc20_address: ::prost::alloc::string::String,
    ) -> Result<QueryTokenPairByErc20AddressResponse, cosmwasm_std::StdError> {
        let request = QueryTokenPairByErc20AddressRequest { erc20_address };
        self.querier
            .query::<QueryTokenPairByErc20AddressResponse>(&cosmwasm_std::QueryRequest::<Q>::Stargate {
                path: "/injective.erc20.v1beta1.Query/TokenPairByERC20Address".to_string(),
                data: request.into(),
            })
    }
}
