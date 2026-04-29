use injective_std_derive::CosmwasmExt;
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.insurance.v1beta1.Params")]
pub struct Params {
    /// default_redemption_notice_period_duration defines the default minimum
    /// notice period duration that must pass after an underwriter sends a
    /// redemption request before the underwriter can claim his tokens
    #[prost(message, optional, tag = "1")]
    pub default_redemption_notice_period_duration: ::core::option::Option<crate::shim::Duration>,
}
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.insurance.v1beta1.InsuranceFund")]
pub struct InsuranceFund {
    /// deposit denomination for the given insurance fund
    #[prost(string, tag = "1")]
    pub deposit_denom: ::prost::alloc::string::String,
    /// insurance fund pool token denomination for the given insurance fund
    #[prost(string, tag = "2")]
    pub insurance_pool_token_denom: ::prost::alloc::string::String,
    /// redemption_notice_period_duration defines the minimum notice period
    /// duration that must pass after an underwriter sends a redemption request
    /// before the underwriter can claim his tokens
    #[prost(message, optional, tag = "3")]
    pub redemption_notice_period_duration: ::core::option::Option<crate::shim::Duration>,
    /// balance of fund
    #[prost(string, tag = "4")]
    pub balance: ::prost::alloc::string::String,
    /// total share tokens minted
    #[prost(string, tag = "5")]
    pub total_share: ::prost::alloc::string::String,
    /// marketID of the derivative market
    #[prost(string, tag = "6")]
    #[serde(alias = "marketID")]
    pub market_id: ::prost::alloc::string::String,
    /// ticker of the derivative market
    #[prost(string, tag = "7")]
    pub market_ticker: ::prost::alloc::string::String,
    /// Oracle base currency of the derivative market OR the oracle symbol for the
    /// binary options market.
    #[prost(string, tag = "8")]
    pub oracle_base: ::prost::alloc::string::String,
    /// Oracle quote currency of the derivative market OR the oracle provider for
    /// the binary options market.
    #[prost(string, tag = "9")]
    pub oracle_quote: ::prost::alloc::string::String,
    /// Oracle type of the binary options or derivative market
    #[prost(enumeration = "super::super::oracle::v1beta1::OracleType", tag = "10")]
    #[serde(deserialize_with = "crate::serde::enum_i32::deserialize::<super::super::oracle::v1beta1::OracleType, _>")]
    pub oracle_type: i32,
    /// Expiration time of the derivative market. Should be -1 for perpetual or -2
    /// for binary options markets.
    #[prost(int64, tag = "11")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub expiry: i64,
}
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.insurance.v1beta1.RedemptionSchedule")]
pub struct RedemptionSchedule {
    /// id of redemption schedule
    #[prost(uint64, tag = "1")]
    #[serde(alias = "ID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
    /// marketId of insurance fund for the redemption
    #[prost(string, tag = "2")]
    #[serde(alias = "marketID")]
    pub market_id: ::prost::alloc::string::String,
    /// address of the redeemer
    #[prost(string, tag = "3")]
    pub redeemer: ::prost::alloc::string::String,
    /// the time after which the redemption can be claimed
    #[prost(message, optional, tag = "4")]
    pub claimable_redemption_time: ::core::option::Option<crate::shim::Timestamp>,
    /// the insurance_pool_token amount to redeem
    #[prost(message, optional, tag = "5")]
    pub redemption_amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.insurance.v1beta1.FailedRedemptionSchedule")]
pub struct FailedRedemptionSchedule {
    /// id of the failed redemption
    #[prost(uint64, tag = "1")]
    #[serde(alias = "ID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
    /// the original redemption schedule that failed to execute
    #[prost(message, optional, tag = "2")]
    pub schedule: ::core::option::Option<RedemptionSchedule>,
    /// the error message from the failed withdrawal attempt
    #[prost(string, tag = "3")]
    pub err: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.insurance.v1beta1.EventInsuranceFundUpdate")]
pub struct EventInsuranceFundUpdate {
    #[prost(message, optional, tag = "1")]
    pub fund: ::core::option::Option<InsuranceFund>,
}
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.insurance.v1beta1.EventRequestRedemption")]
pub struct EventRequestRedemption {
    #[prost(message, optional, tag = "1")]
    pub schedule: ::core::option::Option<RedemptionSchedule>,
}
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.insurance.v1beta1.EventWithdrawRedemption")]
pub struct EventWithdrawRedemption {
    /// redemption schedule triggered withdraw
    #[prost(message, optional, tag = "1")]
    pub schedule: ::core::option::Option<RedemptionSchedule>,
    /// redeem coin amount in base_currency
    #[prost(message, optional, tag = "2")]
    pub redeem_coin: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.insurance.v1beta1.EventWithdrawRedemptionFailed")]
pub struct EventWithdrawRedemptionFailed {
    /// redemption schedule that failed to withdraw
    #[prost(message, optional, tag = "1")]
    pub schedule: ::core::option::Option<RedemptionSchedule>,
    /// error produced while processing the redemption withdrawal
    #[prost(string, tag = "2")]
    pub withdraw_err: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.insurance.v1beta1.EventUnderwrite")]
pub struct EventUnderwrite {
    /// address of the underwriter
    #[prost(string, tag = "1")]
    pub underwriter: ::prost::alloc::string::String,
    /// marketId of insurance fund for the redemption
    #[prost(string, tag = "2")]
    #[serde(alias = "marketID")]
    pub market_id: ::prost::alloc::string::String,
    /// deposit coin amount
    #[prost(message, optional, tag = "3")]
    pub deposit: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// share coin amount
    #[prost(message, optional, tag = "4")]
    pub shares: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.insurance.v1beta1.EventInsuranceWithdraw")]
pub struct EventInsuranceWithdraw {
    #[prost(string, tag = "1")]
    #[serde(alias = "marketID")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub market_ticker: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub withdrawal: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.insurance.v1beta1.EventSetVoucher")]
pub struct EventSetVoucher {
    /// The bech32 address of the voucher holder.
    #[prost(string, tag = "1")]
    pub addr: ::prost::alloc::string::String,
    /// The new voucher amount. A zero coin signals voucher deletion.
    #[prost(message, optional, tag = "2")]
    pub voucher: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// GenesisState defines the insurance module's genesis state.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.insurance.v1beta1.GenesisState")]
pub struct GenesisState {
    /// params defines all the parameters of related to insurance.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// insurance_funds describes the insurance funds available for derivative
    /// markets
    #[prost(message, repeated, tag = "2")]
    pub insurance_funds: ::prost::alloc::vec::Vec<InsuranceFund>,
    /// redemption_schedule describes the redemption requests pending
    #[prost(message, repeated, tag = "3")]
    pub redemption_schedule: ::prost::alloc::vec::Vec<RedemptionSchedule>,
    /// next_share_denom_id describes the next share denom id to be used for newly
    /// creating insurance fund incremented by 1 per insurance fund creation
    #[prost(uint64, tag = "4")]
    #[serde(alias = "next_share_denomID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub next_share_denom_id: u64,
    /// next_redemption_schedule_id describes next redemption schedule id to be
    /// used for next schedule incremented by 1 per redemption request
    #[prost(uint64, tag = "5")]
    #[serde(alias = "next_redemption_scheduleID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub next_redemption_schedule_id: u64,
    /// failed_redemption_schedules describes redemptions that failed during
    /// settlement and are preserved for manual resolution
    #[prost(message, repeated, tag = "6")]
    pub failed_redemption_schedules: ::prost::alloc::vec::Vec<FailedRedemptionSchedule>,
    /// next_failed_redemption_schedule_id describes the next id for failed
    /// redemption schedules, incremented by 1 per failed settlement
    #[prost(uint64, tag = "7")]
    #[serde(alias = "next_failed_redemption_scheduleID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub next_failed_redemption_schedule_id: u64,
    /// outstanding vouchers (failed redemption deliveries)
    #[prost(message, repeated, tag = "8")]
    pub vouchers: ::prost::alloc::vec::Vec<super::super::common::vouchers::v1::AddressVoucher>,
}
/// QueryInsuranceParamsRequest is the request type for the Query/InsuranceParams
/// RPC method.
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.insurance.v1beta1.QueryInsuranceParamsRequest")]
#[proto_query(
    path = "/injective.insurance.v1beta1.Query/InsuranceParams",
    response_type = QueryInsuranceParamsResponse
)]
pub struct QueryInsuranceParamsRequest {}
/// QueryInsuranceParamsRequest is the response type for the
/// Query/InsuranceParams RPC method.
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.insurance.v1beta1.QueryInsuranceParamsResponse")]
pub struct QueryInsuranceParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryInsuranceFundRequest is the request type for the Query/InsuranceFunds
/// RPC method.
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.insurance.v1beta1.QueryInsuranceFundRequest")]
#[proto_query(
    path = "/injective.insurance.v1beta1.Query/InsuranceFund",
    response_type = QueryInsuranceFundResponse
)]
pub struct QueryInsuranceFundRequest {
    /// Market ID for the market
    #[prost(string, tag = "1")]
    #[serde(alias = "marketID")]
    pub market_id: ::prost::alloc::string::String,
}
/// QueryInsuranceFundResponse is the response type for the Query/InsuranceFund
/// RPC method.
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.insurance.v1beta1.QueryInsuranceFundResponse")]
pub struct QueryInsuranceFundResponse {
    #[prost(message, optional, tag = "1")]
    pub fund: ::core::option::Option<InsuranceFund>,
}
/// QueryInsuranceFundsRequest is the request type for the Query/InsuranceFunds
/// RPC method.
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.insurance.v1beta1.QueryInsuranceFundsRequest")]
#[proto_query(
    path = "/injective.insurance.v1beta1.Query/InsuranceFunds",
    response_type = QueryInsuranceFundsResponse
)]
pub struct QueryInsuranceFundsRequest {}
/// QueryInsuranceFundsResponse is the response type for the Query/InsuranceFunds
/// RPC method.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.insurance.v1beta1.QueryInsuranceFundsResponse")]
pub struct QueryInsuranceFundsResponse {
    #[prost(message, repeated, tag = "1")]
    pub funds: ::prost::alloc::vec::Vec<InsuranceFund>,
}
/// QueryEstimatedRedemptionsRequest is the request type for the
/// Query/EstimatedRedemptions RPC method.
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.insurance.v1beta1.QueryEstimatedRedemptionsRequest")]
#[proto_query(
    path = "/injective.insurance.v1beta1.Query/EstimatedRedemptions",
    response_type = QueryEstimatedRedemptionsResponse
)]
pub struct QueryEstimatedRedemptionsRequest {
    #[prost(string, tag = "1")]
    #[serde(alias = "marketID")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
}
/// QueryEstimatedRedemptionsResponse is the response type for the
/// Query/EstimatedRedemptions RPC method.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.insurance.v1beta1.QueryEstimatedRedemptionsResponse")]
pub struct QueryEstimatedRedemptionsResponse {
    #[prost(message, repeated, tag = "1")]
    pub amount: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// QueryPendingRedemptionsRequest is the request type for the
/// Query/PendingRedemptions RPC method.
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.insurance.v1beta1.QueryPendingRedemptionsRequest")]
#[proto_query(
    path = "/injective.insurance.v1beta1.Query/PendingRedemptions",
    response_type = QueryPendingRedemptionsResponse
)]
pub struct QueryPendingRedemptionsRequest {
    #[prost(string, tag = "1")]
    #[serde(alias = "marketID")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
}
/// QueryPendingRedemptionsResponse is the response type for the
/// Query/PendingRedemptions RPC method.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.insurance.v1beta1.QueryPendingRedemptionsResponse")]
pub struct QueryPendingRedemptionsResponse {
    #[prost(message, repeated, tag = "1")]
    pub amount: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// QueryModuleStateRequest is the request type for the
/// Query/InsuranceModuleState RPC method.
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.insurance.v1beta1.QueryModuleStateRequest")]
#[proto_query(
    path = "/injective.insurance.v1beta1.Query/InsuranceModuleState",
    response_type = QueryModuleStateResponse
)]
pub struct QueryModuleStateRequest {}
/// QueryModuleStateResponse is the response type for the
/// Query/InsuranceModuleState RPC method.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.insurance.v1beta1.QueryModuleStateResponse")]
pub struct QueryModuleStateResponse {
    #[prost(message, optional, tag = "1")]
    pub state: ::core::option::Option<GenesisState>,
}
/// QueryFailedRedemptionsRequest is the request type for the
/// Query/FailedRedemptions RPC method.
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.insurance.v1beta1.QueryFailedRedemptionsRequest")]
#[proto_query(
    path = "/injective.insurance.v1beta1.Query/FailedRedemptions",
    response_type = QueryFailedRedemptionsResponse
)]
pub struct QueryFailedRedemptionsRequest {}
/// QueryFailedRedemptionsResponse is the response type for the
/// Query/FailedRedemptions RPC method.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.insurance.v1beta1.QueryFailedRedemptionsResponse")]
pub struct QueryFailedRedemptionsResponse {
    #[prost(message, repeated, tag = "1")]
    pub schedules: ::prost::alloc::vec::Vec<FailedRedemptionSchedule>,
}
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.insurance.v1beta1.QueryVouchersRequest")]
#[proto_query(
    path = "/injective.insurance.v1beta1.Query/Vouchers",
    response_type = QueryVouchersResponse
)]
pub struct QueryVouchersRequest {
    /// denom filter; empty string returns all vouchers
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.insurance.v1beta1.QueryVouchersResponse")]
pub struct QueryVouchersResponse {
    /// List of outstanding vouchers matching the request filter.
    #[prost(message, repeated, tag = "1")]
    pub vouchers: ::prost::alloc::vec::Vec<super::super::common::vouchers::v1::AddressVoucher>,
}
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.insurance.v1beta1.QueryVoucherRequest")]
#[proto_query(
    path = "/injective.insurance.v1beta1.Query/Voucher",
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
#[proto_message(type_url = "/injective.insurance.v1beta1.QueryVoucherResponse")]
pub struct QueryVoucherResponse {
    /// The outstanding voucher coin for the requested denom and address.
    #[prost(message, optional, tag = "1")]
    pub voucher: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgCreateInsuranceFund a message to create an insurance fund for a derivative
/// market.
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.insurance.v1beta1.MsgCreateInsuranceFund")]
pub struct MsgCreateInsuranceFund {
    /// Creator of the insurance fund.
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// Ticker for the derivative market.
    #[prost(string, tag = "2")]
    pub ticker: ::prost::alloc::string::String,
    /// Coin denom to use for the market quote denom
    #[prost(string, tag = "3")]
    pub quote_denom: ::prost::alloc::string::String,
    /// Oracle base currency of the derivative market OR the oracle symbol for the
    /// binary options market.
    #[prost(string, tag = "4")]
    pub oracle_base: ::prost::alloc::string::String,
    /// Oracle quote currency of the derivative market OR the oracle provider for
    /// the binary options market.
    #[prost(string, tag = "5")]
    pub oracle_quote: ::prost::alloc::string::String,
    /// Oracle type of the binary options or derivative market
    #[prost(enumeration = "super::super::oracle::v1beta1::OracleType", tag = "6")]
    #[serde(deserialize_with = "crate::serde::enum_i32::deserialize::<super::super::oracle::v1beta1::OracleType, _>")]
    pub oracle_type: i32,
    /// Expiration time of the derivative market. Should be -1 for perpetual or -2
    /// for binary options markets.
    #[prost(int64, tag = "7")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub expiry: i64,
    /// Initial deposit of the insurance fund
    #[prost(message, optional, tag = "8")]
    pub initial_deposit: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.insurance.v1beta1.MsgCreateInsuranceFundResponse")]
pub struct MsgCreateInsuranceFundResponse {}
/// MsgUnderwrite defines a message for depositing coins to underwrite an
/// insurance fund
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.insurance.v1beta1.MsgUnderwrite")]
pub struct MsgUnderwrite {
    /// Address of the underwriter.
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// MarketID of the insurance fund.
    #[prost(string, tag = "2")]
    #[serde(alias = "marketID")]
    pub market_id: ::prost::alloc::string::String,
    /// Amount of quote_denom to underwrite the insurance fund.
    #[prost(message, optional, tag = "3")]
    pub deposit: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.insurance.v1beta1.MsgUnderwriteResponse")]
pub struct MsgUnderwriteResponse {}
/// MsgRequestRedemption defines a message for requesting a redemption of the
/// sender's insurance fund tokens
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.insurance.v1beta1.MsgRequestRedemption")]
pub struct MsgRequestRedemption {
    /// Address of the underwriter requesting a redemption.
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// MarketID of the insurance fund.
    #[prost(string, tag = "2")]
    #[serde(alias = "marketID")]
    pub market_id: ::prost::alloc::string::String,
    /// Insurance fund share token amount to be redeemed.
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.insurance.v1beta1.MsgRequestRedemptionResponse")]
pub struct MsgRequestRedemptionResponse {}
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.insurance.v1beta1.MsgUpdateParams")]
pub struct MsgUpdateParams {
    /// authority is the address of the governance account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the insurance parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.insurance.v1beta1.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
/// MsgClaimVoucher defines a message for claiming an outstanding voucher
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.insurance.v1beta1.MsgClaimVoucher")]
pub struct MsgClaimVoucher {
    /// The sender's Injective address.
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// The token denom of the voucher to claim.
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.insurance.v1beta1.MsgClaimVoucherResponse")]
pub struct MsgClaimVoucherResponse {}
pub struct InsuranceQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> InsuranceQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn insurance_params(&self) -> Result<QueryInsuranceParamsResponse, cosmwasm_std::StdError> {
        QueryInsuranceParamsRequest {}.query(self.querier)
    }
    pub fn insurance_fund(&self, market_id: ::prost::alloc::string::String) -> Result<QueryInsuranceFundResponse, cosmwasm_std::StdError> {
        QueryInsuranceFundRequest { market_id }.query(self.querier)
    }
    pub fn insurance_funds(&self) -> Result<QueryInsuranceFundsResponse, cosmwasm_std::StdError> {
        QueryInsuranceFundsRequest {}.query(self.querier)
    }
    pub fn estimated_redemptions(
        &self,
        market_id: ::prost::alloc::string::String,
        address: ::prost::alloc::string::String,
    ) -> Result<QueryEstimatedRedemptionsResponse, cosmwasm_std::StdError> {
        QueryEstimatedRedemptionsRequest { market_id, address }.query(self.querier)
    }
    pub fn pending_redemptions(
        &self,
        market_id: ::prost::alloc::string::String,
        address: ::prost::alloc::string::String,
    ) -> Result<QueryPendingRedemptionsResponse, cosmwasm_std::StdError> {
        QueryPendingRedemptionsRequest { market_id, address }.query(self.querier)
    }
    pub fn insurance_module_state(&self) -> Result<QueryModuleStateResponse, cosmwasm_std::StdError> {
        QueryModuleStateRequest {}.query(self.querier)
    }
    pub fn failed_redemptions(&self) -> Result<QueryFailedRedemptionsResponse, cosmwasm_std::StdError> {
        QueryFailedRedemptionsRequest {}.query(self.querier)
    }
    pub fn vouchers(&self, denom: ::prost::alloc::string::String) -> Result<QueryVouchersResponse, cosmwasm_std::StdError> {
        QueryVouchersRequest { denom }.query(self.querier)
    }
    pub fn voucher(
        &self,
        denom: ::prost::alloc::string::String,
        address: ::prost::alloc::string::String,
    ) -> Result<QueryVoucherResponse, cosmwasm_std::StdError> {
        QueryVoucherRequest { denom, address }.query(self.querier)
    }
}
