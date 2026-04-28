use injective_std_derive::CosmwasmExt;
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.auction.v1beta1.EventSetVoucher")]
pub struct EventSetVoucher {
    /// The bech32 address of the voucher holder.
    #[prost(string, tag = "1")]
    pub addr: ::prost::alloc::string::String,
    /// The new voucher amount. A zero coin signals voucher deletion.
    #[prost(message, optional, tag = "2")]
    pub voucher: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.auction.v1beta1.Params")]
pub struct Params {
    /// auction_period_duration defines the auction period duration
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub auction_period: i64,
    /// min_next_bid_increment_rate defines the minimum increment rate for new bids
    #[prost(string, tag = "2")]
    pub min_next_bid_increment_rate: ::prost::alloc::string::String,
    /// inj_basket_max_cap defines the maximum cap for INJ contained in an auction
    /// basket
    #[prost(string, tag = "3")]
    pub inj_basket_max_cap: ::prost::alloc::string::String,
    /// bidders_whitelist defines the list of addresses that are allowed to bid
    /// if empty, any address can bid; if populated, only whitelisted addresses can
    /// bid
    #[prost(string, repeated, tag = "4")]
    pub bidders_whitelist: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.auction.v1beta1.Bid")]
pub struct Bid {
    #[prost(string, tag = "1")]
    pub bidder: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.auction.v1beta1.LastAuctionResult")]
pub struct LastAuctionResult {
    /// winner describes the address of the winner
    #[prost(string, tag = "1")]
    pub winner: ::prost::alloc::string::String,
    /// amount describes the amount the winner get from the auction
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// round defines the round number of auction
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub round: u64,
}
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.auction.v1beta1.EventBid")]
pub struct EventBid {
    /// bidder describes the address of bidder
    #[prost(string, tag = "1")]
    pub bidder: ::prost::alloc::string::String,
    /// amount describes the amount the bidder put on the auction
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// round defines the round number of auction
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub round: u64,
}
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.auction.v1beta1.EventAuctionResult")]
pub struct EventAuctionResult {
    /// winner describes the address of the winner
    #[prost(string, tag = "1")]
    pub winner: ::prost::alloc::string::String,
    /// amount describes the amount the winner get from the auction
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// round defines the round number of auction
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub round: u64,
}
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.auction.v1beta1.EventAuctionStart")]
pub struct EventAuctionStart {
    /// round defines the round number of auction
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub round: u64,
    /// ending_timestamp describes auction end time
    #[prost(int64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub ending_timestamp: i64,
    /// new_basket describes auction module balance at the time of new auction
    /// start
    #[prost(message, repeated, tag = "3")]
    pub new_basket: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// GenesisState defines the auction module's genesis state.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.auction.v1beta1.GenesisState")]
pub struct GenesisState {
    /// params defines all the parameters of related to auction.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// current auction round
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub auction_round: u64,
    /// current highest bid
    #[prost(message, optional, tag = "3")]
    pub highest_bid: ::core::option::Option<Bid>,
    /// auction ending timestamp
    #[prost(int64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub auction_ending_timestamp: i64,
    /// last auction result
    #[prost(message, optional, tag = "5")]
    pub last_auction_result: ::core::option::Option<LastAuctionResult>,
    /// outstanding vouchers (failed basket deliveries)
    #[prost(message, repeated, tag = "6")]
    pub vouchers: ::prost::alloc::vec::Vec<super::super::common::vouchers::v1::AddressVoucher>,
}
/// QueryAuctionParamsRequest is the request type for the Query/AuctionParams RPC
/// method.
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.auction.v1beta1.QueryAuctionParamsRequest")]
#[proto_query(
    path = "/injective.auction.v1beta1.Query/AuctionParams",
    response_type = QueryAuctionParamsResponse
)]
pub struct QueryAuctionParamsRequest {}
/// QueryAuctionParamsRequest is the response type for the Query/AuctionParams
/// RPC method.
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.auction.v1beta1.QueryAuctionParamsResponse")]
pub struct QueryAuctionParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryCurrentAuctionBasketRequest is the request type for the
/// Query/CurrentAuctionBasket RPC method.
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.auction.v1beta1.QueryCurrentAuctionBasketRequest")]
#[proto_query(
    path = "/injective.auction.v1beta1.Query/CurrentAuctionBasket",
    response_type = QueryCurrentAuctionBasketResponse
)]
pub struct QueryCurrentAuctionBasketRequest {}
/// QueryCurrentAuctionBasketResponse is the response type for the
/// Query/CurrentAuctionBasket RPC method.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.auction.v1beta1.QueryCurrentAuctionBasketResponse")]
pub struct QueryCurrentAuctionBasketResponse {
    /// amount describes the amount put on auction
    #[prost(message, repeated, tag = "1")]
    pub amount: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// auctionRound describes current auction round
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub auction_round: u64,
    /// auctionClosingTime describes auction close time for the round
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub auction_closing_time: u64,
    /// highestBidder describes highest bidder on current round
    #[prost(string, tag = "4")]
    pub highest_bidder: ::prost::alloc::string::String,
    /// highestBidAmount describes highest bid amount on current round
    #[prost(string, tag = "5")]
    pub highest_bid_amount: ::prost::alloc::string::String,
}
/// QueryModuleStateRequest is the request type for the Query/AuctionModuleState
/// RPC method.
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.auction.v1beta1.QueryModuleStateRequest")]
#[proto_query(
    path = "/injective.auction.v1beta1.Query/AuctionModuleState",
    response_type = QueryModuleStateResponse
)]
pub struct QueryModuleStateRequest {}
/// QueryModuleStateResponse is the response type for the
/// Query/AuctionModuleState RPC method.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.auction.v1beta1.QueryModuleStateResponse")]
pub struct QueryModuleStateResponse {
    #[prost(message, optional, tag = "1")]
    pub state: ::core::option::Option<GenesisState>,
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.auction.v1beta1.QueryLastAuctionResultRequest")]
#[proto_query(
    path = "/injective.auction.v1beta1.Query/LastAuctionResult",
    response_type = QueryLastAuctionResultResponse
)]
pub struct QueryLastAuctionResultRequest {}
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.auction.v1beta1.QueryLastAuctionResultResponse")]
pub struct QueryLastAuctionResultResponse {
    #[prost(message, optional, tag = "1")]
    pub last_auction_result: ::core::option::Option<LastAuctionResult>,
}
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.auction.v1beta1.QueryVouchersRequest")]
#[proto_query(
    path = "/injective.auction.v1beta1.Query/Vouchers",
    response_type = QueryVouchersResponse
)]
pub struct QueryVouchersRequest {
    /// denom filter; empty string returns all vouchers
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.auction.v1beta1.QueryVouchersResponse")]
pub struct QueryVouchersResponse {
    /// List of outstanding vouchers matching the request filter.
    #[prost(message, repeated, tag = "1")]
    pub vouchers: ::prost::alloc::vec::Vec<super::super::common::vouchers::v1::AddressVoucher>,
}
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.auction.v1beta1.QueryVoucherRequest")]
#[proto_query(
    path = "/injective.auction.v1beta1.Query/Voucher",
    response_type = QueryVoucherResponse
)]
pub struct QueryVoucherRequest {
    /// Required. The token denom to look up.
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    /// Required. The bech32 address of the voucher holder.
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.auction.v1beta1.QueryVoucherResponse")]
pub struct QueryVoucherResponse {
    /// The outstanding voucher coin for the requested denom and address.
    #[prost(message, optional, tag = "1")]
    pub voucher: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// Bid defines a SDK message for placing a bid for an auction
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.auction.v1beta1.MsgBid")]
pub struct MsgBid {
    /// the sender's Injective address
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// amount of the bid in INJ tokens
    #[prost(message, optional, tag = "2")]
    pub bid_amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// the current auction round being bid on
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub round: u64,
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.auction.v1beta1.MsgBidResponse")]
pub struct MsgBidResponse {}
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.auction.v1beta1.MsgUpdateParams")]
pub struct MsgUpdateParams {
    /// authority is the address of the governance account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the ocr parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.auction.v1beta1.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
/// MsgClaimVoucher defines a message for claiming an outstanding voucher
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.auction.v1beta1.MsgClaimVoucher")]
pub struct MsgClaimVoucher {
    /// The sender's Injective address.
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// The token denom of the voucher to claim.
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.auction.v1beta1.MsgClaimVoucherResponse")]
pub struct MsgClaimVoucherResponse {}
pub struct AuctionQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> AuctionQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn auction_params(&self) -> Result<QueryAuctionParamsResponse, cosmwasm_std::StdError> {
        let request = QueryAuctionParamsRequest {};
        self.querier
            .query::<QueryAuctionParamsResponse>(&cosmwasm_std::QueryRequest::<Q>::Stargate {
                path: "/injective.auction.v1beta1.Query/AuctionParams".to_string(),
                data: request.into(),
            })
    }
    pub fn current_auction_basket(&self) -> Result<QueryCurrentAuctionBasketResponse, cosmwasm_std::StdError> {
        let request = QueryCurrentAuctionBasketRequest {};
        self.querier
            .query::<QueryCurrentAuctionBasketResponse>(&cosmwasm_std::QueryRequest::<Q>::Stargate {
                path: "/injective.auction.v1beta1.Query/CurrentAuctionBasket".to_string(),
                data: request.into(),
            })
    }
    pub fn auction_module_state(&self) -> Result<QueryModuleStateResponse, cosmwasm_std::StdError> {
        let request = QueryModuleStateRequest {};
        self.querier
            .query::<QueryModuleStateResponse>(&cosmwasm_std::QueryRequest::<Q>::Stargate {
                path: "/injective.auction.v1beta1.Query/AuctionModuleState".to_string(),
                data: request.into(),
            })
    }
    pub fn last_auction_result(&self) -> Result<QueryLastAuctionResultResponse, cosmwasm_std::StdError> {
        let request = QueryLastAuctionResultRequest {};
        self.querier
            .query::<QueryLastAuctionResultResponse>(&cosmwasm_std::QueryRequest::<Q>::Stargate {
                path: "/injective.auction.v1beta1.Query/LastAuctionResult".to_string(),
                data: request.into(),
            })
    }
    pub fn vouchers(&self, denom: ::prost::alloc::string::String) -> Result<QueryVouchersResponse, cosmwasm_std::StdError> {
        let request = QueryVouchersRequest { denom };
        self.querier.query::<QueryVouchersResponse>(&cosmwasm_std::QueryRequest::<Q>::Stargate {
            path: "/injective.auction.v1beta1.Query/Vouchers".to_string(),
            data: request.into(),
        })
    }
    pub fn voucher(
        &self,
        denom: ::prost::alloc::string::String,
        address: ::prost::alloc::string::String,
    ) -> Result<QueryVoucherResponse, cosmwasm_std::StdError> {
        let request = QueryVoucherRequest { denom, address };
        self.querier.query::<QueryVoucherResponse>(&cosmwasm_std::QueryRequest::<Q>::Stargate {
            path: "/injective.auction.v1beta1.Query/Voucher".to_string(),
            data: request.into(),
        })
    }
}
