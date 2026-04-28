use injective_std_derive::CosmwasmExt;
/// Request represents a request to the ABCI application.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.abci.v1.Request")]
pub struct Request {
    /// Sum of all possible messages.
    #[prost(oneof = "request::Value", tags = "1, 2, 3, 5, 6, 8, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20")]
    pub value: ::core::option::Option<request::Value>,
}
/// Nested message and enum types in `Request`.
pub mod request {
    use injective_std_derive::CosmwasmExt;
    /// Sum of all possible messages.
    #[derive(Clone, PartialEq, Eq, ::prost::Oneof, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
    pub enum Value {
        #[prost(message, tag = "1")]
        Echo(super::EchoRequest),
        #[prost(message, tag = "2")]
        Flush(super::FlushRequest),
        #[prost(message, tag = "3")]
        Info(super::InfoRequest),
        #[prost(message, tag = "5")]
        InitChain(super::InitChainRequest),
        #[prost(message, tag = "6")]
        Query(super::QueryRequest),
        #[prost(message, tag = "8")]
        CheckTx(super::CheckTxRequest),
        #[prost(message, tag = "11")]
        Commit(super::CommitRequest),
        #[prost(message, tag = "12")]
        ListSnapshots(super::ListSnapshotsRequest),
        #[prost(message, tag = "13")]
        OfferSnapshot(super::OfferSnapshotRequest),
        #[prost(message, tag = "14")]
        LoadSnapshotChunk(super::LoadSnapshotChunkRequest),
        #[prost(message, tag = "15")]
        ApplySnapshotChunk(super::ApplySnapshotChunkRequest),
        #[prost(message, tag = "16")]
        PrepareProposal(super::PrepareProposalRequest),
        #[prost(message, tag = "17")]
        ProcessProposal(super::ProcessProposalRequest),
        #[prost(message, tag = "18")]
        ExtendVote(super::ExtendVoteRequest),
        #[prost(message, tag = "19")]
        VerifyVoteExtension(super::VerifyVoteExtensionRequest),
        #[prost(message, tag = "20")]
        FinalizeBlock(super::FinalizeBlockRequest),
    }
}
/// EchoRequest is a request to "echo" the given string.
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.abci.v1.EchoRequest")]
pub struct EchoRequest {
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
/// FlushRequest is a request to flush the write buffer.
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.abci.v1.FlushRequest")]
pub struct FlushRequest {}
/// InfoRequest is a request for the ABCI application version.
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.abci.v1.InfoRequest")]
pub struct InfoRequest {
    #[prost(string, tag = "1")]
    pub version: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub block_version: u64,
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub p2p_version: u64,
    #[prost(string, tag = "4")]
    pub abci_version: ::prost::alloc::string::String,
}
/// InitChainRequest is a request to initialize the blockchain.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.abci.v1.InitChainRequest")]
pub struct InitChainRequest {
    #[prost(message, optional, tag = "1")]
    pub time: ::core::option::Option<crate::shim::Timestamp>,
    #[prost(string, tag = "2")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub consensus_params: ::core::option::Option<super::super::types::v1::ConsensusParams>,
    #[prost(message, repeated, tag = "4")]
    pub validators: ::prost::alloc::vec::Vec<ValidatorUpdate>,
    #[prost(bytes = "vec", tag = "5")]
    pub app_state_bytes: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub initial_height: i64,
}
/// QueryRequest is a request to query the application state.
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.abci.v1.QueryRequest")]
pub struct QueryRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: i64,
    #[prost(bool, tag = "4")]
    pub prove: bool,
}
/// CheckTxRequest is a request to check that the transaction is valid.
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.abci.v1.CheckTxRequest")]
pub struct CheckTxRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub tx: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration = "CheckTxType", tag = "3")]
    #[serde(deserialize_with = "crate::serde::enum_i32::deserialize::<CheckTxType, _>")]
    pub r#type: i32,
}
/// CommitRequest is a request to commit the pending application state.
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.abci.v1.CommitRequest")]
pub struct CommitRequest {}
/// Request to list available snapshots.
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.abci.v1.ListSnapshotsRequest")]
pub struct ListSnapshotsRequest {}
/// Request offering a snapshot to the application.
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.abci.v1.OfferSnapshotRequest")]
pub struct OfferSnapshotRequest {
    /// snapshot offered by peers
    #[prost(message, optional, tag = "1")]
    pub snapshot: ::core::option::Option<Snapshot>,
    /// light client-verified app hash for snapshot height
    #[prost(bytes = "vec", tag = "2")]
    pub app_hash: ::prost::alloc::vec::Vec<u8>,
}
/// Request to load a snapshot chunk.
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.abci.v1.LoadSnapshotChunkRequest")]
pub struct LoadSnapshotChunkRequest {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: u64,
    #[prost(uint32, tag = "2")]
    pub format: u32,
    #[prost(uint32, tag = "3")]
    pub chunk: u32,
}
/// Request to apply a snapshot chunk.
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.abci.v1.ApplySnapshotChunkRequest")]
pub struct ApplySnapshotChunkRequest {
    #[prost(uint32, tag = "1")]
    pub index: u32,
    #[prost(bytes = "vec", tag = "2")]
    pub chunk: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "3")]
    pub sender: ::prost::alloc::string::String,
}
/// PrepareProposalRequest is a request for the ABCI application to prepare a new
/// block proposal.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.abci.v1.PrepareProposalRequest")]
pub struct PrepareProposalRequest {
    /// the modified transactions cannot exceed this size.
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub max_tx_bytes: i64,
    /// txs is an array of transactions that will be included in a block,
    /// sent to the app for possible modifications.
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub txs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag = "3")]
    pub local_last_commit: ::core::option::Option<ExtendedCommitInfo>,
    #[prost(message, repeated, tag = "4")]
    pub misbehavior: ::prost::alloc::vec::Vec<Misbehavior>,
    #[prost(int64, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: i64,
    #[prost(message, optional, tag = "6")]
    pub time: ::core::option::Option<crate::shim::Timestamp>,
    #[prost(bytes = "vec", tag = "7")]
    pub next_validators_hash: ::prost::alloc::vec::Vec<u8>,
    /// address of the public key of the validator proposing the block.
    #[prost(bytes = "vec", tag = "8")]
    pub proposer_address: ::prost::alloc::vec::Vec<u8>,
}
/// ProcessProposalRequest is a request for the ABCI application to process a proposal
/// received from another validator.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.abci.v1.ProcessProposalRequest")]
pub struct ProcessProposalRequest {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub txs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag = "2")]
    pub proposed_last_commit: ::core::option::Option<CommitInfo>,
    #[prost(message, repeated, tag = "3")]
    pub misbehavior: ::prost::alloc::vec::Vec<Misbehavior>,
    /// Merkle root hash of the fields of the proposed block.
    #[prost(bytes = "vec", tag = "4")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: i64,
    #[prost(message, optional, tag = "6")]
    pub time: ::core::option::Option<crate::shim::Timestamp>,
    #[prost(bytes = "vec", tag = "7")]
    pub next_validators_hash: ::prost::alloc::vec::Vec<u8>,
    /// address of the public key of the original proposer of the block.
    #[prost(bytes = "vec", tag = "8")]
    pub proposer_address: ::prost::alloc::vec::Vec<u8>,
}
/// ExtendVoteRequest extends a precommit vote with application-injected data.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.abci.v1.ExtendVoteRequest")]
pub struct ExtendVoteRequest {
    /// the hash of the block that this vote may be referring to
    #[prost(bytes = "vec", tag = "1")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    /// the height of the extended vote
    #[prost(int64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: i64,
    /// info of the block that this vote may be referring to
    #[prost(message, optional, tag = "3")]
    pub time: ::core::option::Option<crate::shim::Timestamp>,
    #[prost(bytes = "vec", repeated, tag = "4")]
    pub txs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag = "5")]
    pub proposed_last_commit: ::core::option::Option<CommitInfo>,
    #[prost(message, repeated, tag = "6")]
    pub misbehavior: ::prost::alloc::vec::Vec<Misbehavior>,
    #[prost(bytes = "vec", tag = "7")]
    pub next_validators_hash: ::prost::alloc::vec::Vec<u8>,
    /// address of the public key of the original proposer of the block.
    #[prost(bytes = "vec", tag = "8")]
    pub proposer_address: ::prost::alloc::vec::Vec<u8>,
}
/// VerifyVoteExtensionRequest is a request for the application to verify a vote extension
/// produced by a different validator.
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.abci.v1.VerifyVoteExtensionRequest")]
pub struct VerifyVoteExtensionRequest {
    /// the hash of the block that this received vote corresponds to
    #[prost(bytes = "vec", tag = "1")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    /// the validator that signed the vote extension
    #[prost(bytes = "vec", tag = "2")]
    pub validator_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: i64,
    #[prost(bytes = "vec", tag = "4")]
    pub vote_extension: ::prost::alloc::vec::Vec<u8>,
}
/// FinalizeBlockRequest is a request to finalize the block.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.abci.v1.FinalizeBlockRequest")]
pub struct FinalizeBlockRequest {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub txs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag = "2")]
    pub decided_last_commit: ::core::option::Option<CommitInfo>,
    #[prost(message, repeated, tag = "3")]
    pub misbehavior: ::prost::alloc::vec::Vec<Misbehavior>,
    /// Merkle root hash of the fields of the decided block.
    #[prost(bytes = "vec", tag = "4")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: i64,
    #[prost(message, optional, tag = "6")]
    pub time: ::core::option::Option<crate::shim::Timestamp>,
    #[prost(bytes = "vec", tag = "7")]
    pub next_validators_hash: ::prost::alloc::vec::Vec<u8>,
    /// address of the public key of the original proposer of the block.
    #[prost(bytes = "vec", tag = "8")]
    pub proposer_address: ::prost::alloc::vec::Vec<u8>,
    /// If the node is syncing/replaying blocks - target height. If not, syncing_to == height.
    #[prost(int64, tag = "9")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub syncing_to_height: i64,
}
/// Response represents a response from the ABCI application.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.abci.v1.Response")]
pub struct Response {
    /// Sum of all possible messages.
    #[prost(oneof = "response::Value", tags = "1, 2, 3, 4, 6, 7, 9, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21")]
    pub value: ::core::option::Option<response::Value>,
}
/// Nested message and enum types in `Response`.
pub mod response {
    use injective_std_derive::CosmwasmExt;
    /// Sum of all possible messages.
    #[derive(Clone, PartialEq, Eq, ::prost::Oneof, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
    pub enum Value {
        #[prost(message, tag = "1")]
        Exception(super::ExceptionResponse),
        #[prost(message, tag = "2")]
        Echo(super::EchoResponse),
        #[prost(message, tag = "3")]
        Flush(super::FlushResponse),
        #[prost(message, tag = "4")]
        Info(super::InfoResponse),
        #[prost(message, tag = "6")]
        InitChain(super::InitChainResponse),
        #[prost(message, tag = "7")]
        Query(super::QueryResponse),
        #[prost(message, tag = "9")]
        CheckTx(super::CheckTxResponse),
        #[prost(message, tag = "12")]
        Commit(super::CommitResponse),
        #[prost(message, tag = "13")]
        ListSnapshots(super::ListSnapshotsResponse),
        #[prost(message, tag = "14")]
        OfferSnapshot(super::OfferSnapshotResponse),
        #[prost(message, tag = "15")]
        LoadSnapshotChunk(super::LoadSnapshotChunkResponse),
        #[prost(message, tag = "16")]
        ApplySnapshotChunk(super::ApplySnapshotChunkResponse),
        #[prost(message, tag = "17")]
        PrepareProposal(super::PrepareProposalResponse),
        #[prost(message, tag = "18")]
        ProcessProposal(super::ProcessProposalResponse),
        #[prost(message, tag = "19")]
        ExtendVote(super::ExtendVoteResponse),
        #[prost(message, tag = "20")]
        VerifyVoteExtension(super::VerifyVoteExtensionResponse),
        #[prost(message, tag = "21")]
        FinalizeBlock(super::FinalizeBlockResponse),
    }
}
/// nondeterministic
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.abci.v1.ExceptionResponse")]
pub struct ExceptionResponse {
    #[prost(string, tag = "1")]
    pub error: ::prost::alloc::string::String,
}
/// EchoResponse indicates that the connection is still alive.
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.abci.v1.EchoResponse")]
pub struct EchoResponse {
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
/// FlushResponse indicates that the write buffer was flushed.
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.abci.v1.FlushResponse")]
pub struct FlushResponse {}
/// InfoResponse contains the ABCI application version information.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.abci.v1.InfoResponse")]
pub struct InfoResponse {
    #[prost(string, tag = "1")]
    pub data: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub app_version: u64,
    #[prost(int64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub last_block_height: i64,
    #[prost(bytes = "vec", tag = "5")]
    pub last_block_app_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(map = "string, uint32", tag = "6")]
    pub lane_priorities: ::std::collections::HashMap<::prost::alloc::string::String, u32>,
    #[prost(string, tag = "7")]
    pub default_lane: ::prost::alloc::string::String,
}
/// InitChainResponse contains the ABCI application's hash and updates to the
/// validator set and/or the consensus params, if any.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.abci.v1.InitChainResponse")]
pub struct InitChainResponse {
    #[prost(message, optional, tag = "1")]
    pub consensus_params: ::core::option::Option<super::super::types::v1::ConsensusParams>,
    #[prost(message, repeated, tag = "2")]
    pub validators: ::prost::alloc::vec::Vec<ValidatorUpdate>,
    #[prost(bytes = "vec", tag = "3")]
    pub app_hash: ::prost::alloc::vec::Vec<u8>,
}
/// QueryResponse contains the ABCI application data along with a proof.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.abci.v1.QueryResponse")]
pub struct QueryResponse {
    #[prost(uint32, tag = "1")]
    pub code: u32,
    /// bytes data = 2; // use "value" instead.
    ///
    /// nondeterministic
    #[prost(string, tag = "3")]
    pub log: ::prost::alloc::string::String,
    /// nondeterministic
    #[prost(string, tag = "4")]
    pub info: ::prost::alloc::string::String,
    #[prost(int64, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub index: i64,
    #[prost(bytes = "vec", tag = "6")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "7")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "8")]
    pub proof_ops: ::core::option::Option<super::super::crypto::v1::ProofOps>,
    #[prost(int64, tag = "9")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: i64,
    #[prost(string, tag = "10")]
    pub codespace: ::prost::alloc::string::String,
}
/// CheckTxResponse shows if the transaction was deemed valid by the ABCI
/// application.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.abci.v1.CheckTxResponse")]
pub struct CheckTxResponse {
    #[prost(uint32, tag = "1")]
    pub code: u32,
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// nondeterministic
    #[prost(string, tag = "3")]
    pub log: ::prost::alloc::string::String,
    /// nondeterministic
    #[prost(string, tag = "4")]
    pub info: ::prost::alloc::string::String,
    #[prost(int64, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub gas_wanted: i64,
    #[prost(int64, tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub gas_used: i64,
    /// nondeterministic
    #[prost(message, repeated, tag = "7")]
    pub events: ::prost::alloc::vec::Vec<Event>,
    #[prost(string, tag = "8")]
    pub codespace: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    #[serde(alias = "laneID")]
    pub lane_id: ::prost::alloc::string::String,
}
/// CommitResponse indicates how much blocks should CometBFT retain.
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.abci.v1.CommitResponse")]
pub struct CommitResponse {
    #[prost(int64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub retain_height: i64,
}
/// ListSnapshotsResponse contains the list of snapshots.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.abci.v1.ListSnapshotsResponse")]
pub struct ListSnapshotsResponse {
    #[prost(message, repeated, tag = "1")]
    pub snapshots: ::prost::alloc::vec::Vec<Snapshot>,
}
/// OfferSnapshotResponse indicates the ABCI application decision whenever to
/// provide a snapshot to the requester or not.
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.abci.v1.OfferSnapshotResponse")]
pub struct OfferSnapshotResponse {
    #[prost(enumeration = "OfferSnapshotResult", tag = "1")]
    #[serde(deserialize_with = "crate::serde::enum_i32::deserialize::<OfferSnapshotResult, _>")]
    pub result: i32,
}
/// LoadSnapshotChunkResponse returns a snapshot's chunk.
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.abci.v1.LoadSnapshotChunkResponse")]
pub struct LoadSnapshotChunkResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub chunk: ::prost::alloc::vec::Vec<u8>,
}
/// ApplySnapshotChunkResponse returns a result of applying the specified chunk.
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.abci.v1.ApplySnapshotChunkResponse")]
pub struct ApplySnapshotChunkResponse {
    #[prost(enumeration = "ApplySnapshotChunkResult", tag = "1")]
    #[serde(deserialize_with = "crate::serde::enum_i32::deserialize::<ApplySnapshotChunkResult, _>")]
    pub result: i32,
    /// Chunks to refetch and reapply
    #[prost(uint32, repeated, tag = "2")]
    pub refetch_chunks: ::prost::alloc::vec::Vec<u32>,
    /// Chunk senders to reject and ban
    #[prost(string, repeated, tag = "3")]
    pub reject_senders: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// PrepareProposalResponse contains a list of transactions, which will form a block.
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.abci.v1.PrepareProposalResponse")]
pub struct PrepareProposalResponse {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub txs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// ProcessProposalResponse indicates the ABCI application's decision whenever
/// the given proposal should be accepted or not.
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.abci.v1.ProcessProposalResponse")]
pub struct ProcessProposalResponse {
    #[prost(enumeration = "ProcessProposalStatus", tag = "1")]
    #[serde(deserialize_with = "crate::serde::enum_i32::deserialize::<ProcessProposalStatus, _>")]
    pub status: i32,
}
/// ExtendVoteResponse contains the vote extension that the application would like to
/// attach to its next precommit vote.
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.abci.v1.ExtendVoteResponse")]
pub struct ExtendVoteResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub vote_extension: ::prost::alloc::vec::Vec<u8>,
}
/// VerifyVoteExtensionResponse indicates the ABCI application's decision
/// whenever the vote extension should be accepted or not.
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.abci.v1.VerifyVoteExtensionResponse")]
pub struct VerifyVoteExtensionResponse {
    #[prost(enumeration = "VerifyVoteExtensionStatus", tag = "1")]
    #[serde(deserialize_with = "crate::serde::enum_i32::deserialize::<VerifyVoteExtensionStatus, _>")]
    pub status: i32,
}
/// FinalizeBlockResponse contains the result of executing the block.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.abci.v1.FinalizeBlockResponse")]
pub struct FinalizeBlockResponse {
    /// set of block events emitted as part of executing the block
    ///
    /// nondeterministic
    #[prost(message, repeated, tag = "1")]
    pub events: ::prost::alloc::vec::Vec<Event>,
    /// the result of executing each transaction including the events
    /// the particular transaction emitted. This should match the order
    /// of the transactions delivered in the block itself
    #[prost(message, repeated, tag = "2")]
    pub tx_results: ::prost::alloc::vec::Vec<ExecTxResult>,
    /// a list of updates to the validator set. These will reflect the validator set at current height + 2.
    #[prost(message, repeated, tag = "3")]
    pub validator_updates: ::prost::alloc::vec::Vec<ValidatorUpdate>,
    /// updates to the consensus params, if any.
    #[prost(message, optional, tag = "4")]
    pub consensus_param_updates: ::core::option::Option<super::super::types::v1::ConsensusParams>,
    /// app_hash is the hash of the applications' state which is used to confirm
    /// that execution of the transactions was deterministic.
    /// It is up to the application to decide which algorithm to use.
    #[prost(bytes = "vec", tag = "5")]
    pub app_hash: ::prost::alloc::vec::Vec<u8>,
}
/// CommitInfo contains votes for the particular round.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.abci.v1.CommitInfo")]
pub struct CommitInfo {
    #[prost(int32, tag = "1")]
    pub round: i32,
    #[prost(message, repeated, tag = "2")]
    pub votes: ::prost::alloc::vec::Vec<VoteInfo>,
}
/// ExtendedCommitInfo is similar to CommitInfo except that it is only used in
/// the PrepareProposal request such that Tendermint can provide vote extensions
/// to the application.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.abci.v1.ExtendedCommitInfo")]
pub struct ExtendedCommitInfo {
    /// The round at which the block proposer decided in the previous height.
    #[prost(int32, tag = "1")]
    pub round: i32,
    /// List of validators' addresses in the last validator set with their voting
    /// information, including vote extensions.
    #[prost(message, repeated, tag = "2")]
    pub votes: ::prost::alloc::vec::Vec<ExtendedVoteInfo>,
}
/// Event allows application developers to attach additional information to
/// ResponseFinalizeBlock and ResponseCheckTx.
/// Up to 0.37, this could also be used in ResponseBeginBlock, ResponseEndBlock,
/// and ResponseDeliverTx.
/// Later, transactions may be queried using these events.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.abci.v1.Event")]
pub struct Event {
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub attributes: ::prost::alloc::vec::Vec<EventAttribute>,
}
/// EventAttribute is a single key-value pair, associated with an event.
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.abci.v1.EventAttribute")]
pub struct EventAttribute {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
    /// nondeterministic
    #[prost(bool, tag = "3")]
    pub index: bool,
}
/// ExecTxResult contains results of executing one individual transaction.
///
/// * Its structure is equivalent to #ResponseDeliverTx which will be deprecated/deleted
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.abci.v1.ExecTxResult")]
pub struct ExecTxResult {
    #[prost(uint32, tag = "1")]
    pub code: u32,
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// nondeterministic
    #[prost(string, tag = "3")]
    pub log: ::prost::alloc::string::String,
    /// nondeterministic
    #[prost(string, tag = "4")]
    pub info: ::prost::alloc::string::String,
    #[prost(int64, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub gas_wanted: i64,
    #[prost(int64, tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub gas_used: i64,
    /// nondeterministic
    #[prost(message, repeated, tag = "7")]
    pub events: ::prost::alloc::vec::Vec<Event>,
    #[prost(string, tag = "8")]
    pub codespace: ::prost::alloc::string::String,
}
/// TxResult contains results of executing the transaction.
///
/// One usage is indexing transaction results.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.abci.v1.TxResult")]
pub struct TxResult {
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: i64,
    #[prost(uint32, tag = "2")]
    pub index: u32,
    #[prost(bytes = "vec", tag = "3")]
    pub tx: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "4")]
    pub result: ::core::option::Option<ExecTxResult>,
}
/// Validator in the validator set.
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.abci.v1.Validator")]
pub struct Validator {
    /// The first 20 bytes of SHA256(public key)
    #[prost(bytes = "vec", tag = "1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    /// PubKey pub_key = 2 \[(gogoproto.nullable)=false\];
    ///
    /// The voting power
    #[prost(int64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub power: i64,
}
/// ValidatorUpdate is a singular update to a validator set.
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.abci.v1.ValidatorUpdate")]
pub struct ValidatorUpdate {
    #[prost(int64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub power: i64,
    #[prost(bytes = "vec", tag = "3")]
    pub pub_key_bytes: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "4")]
    pub pub_key_type: ::prost::alloc::string::String,
}
/// VoteInfo contains the information about the vote.
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.abci.v1.VoteInfo")]
pub struct VoteInfo {
    #[prost(message, optional, tag = "1")]
    pub validator: ::core::option::Option<Validator>,
    #[prost(enumeration = "super::super::types::v1::BlockIdFlag", tag = "3")]
    #[serde(alias = "blockID_flag")]
    #[serde(deserialize_with = "crate::serde::enum_i32::deserialize::<super::super::types::v1::BlockIdFlag, _>")]
    pub block_id_flag: i32,
}
/// ExtendedVoteInfo extends VoteInfo with the vote extensions (non-deterministic).
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.abci.v1.ExtendedVoteInfo")]
pub struct ExtendedVoteInfo {
    /// The validator that sent the vote.
    #[prost(message, optional, tag = "1")]
    pub validator: ::core::option::Option<Validator>,
    /// Non-deterministic extension provided by the sending validator's application.
    #[prost(bytes = "vec", tag = "3")]
    pub vote_extension: ::prost::alloc::vec::Vec<u8>,
    /// Vote extension signature created by CometBFT
    #[prost(bytes = "vec", tag = "4")]
    pub extension_signature: ::prost::alloc::vec::Vec<u8>,
    /// block_id_flag indicates whether the validator voted for a block, nil, or did not vote at all
    #[prost(enumeration = "super::super::types::v1::BlockIdFlag", tag = "5")]
    #[serde(alias = "blockID_flag")]
    #[serde(deserialize_with = "crate::serde::enum_i32::deserialize::<super::super::types::v1::BlockIdFlag, _>")]
    pub block_id_flag: i32,
}
/// Misbehavior is a type of misbehavior committed by a validator.
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.abci.v1.Misbehavior")]
pub struct Misbehavior {
    #[prost(enumeration = "MisbehaviorType", tag = "1")]
    #[serde(deserialize_with = "crate::serde::enum_i32::deserialize::<MisbehaviorType, _>")]
    pub r#type: i32,
    /// The offending validator
    #[prost(message, optional, tag = "2")]
    pub validator: ::core::option::Option<Validator>,
    /// The height when the offense occurred
    #[prost(int64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: i64,
    /// The corresponding time where the offense occurred
    #[prost(message, optional, tag = "4")]
    pub time: ::core::option::Option<crate::shim::Timestamp>,
    /// Total voting power of the validator set in case the ABCI application does
    /// not store historical validators.
    /// <https://github.com/tendermint/tendermint/issues/4581>
    #[prost(int64, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub total_voting_power: i64,
}
/// Snapshot of the ABCI application state.
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cometbft.abci.v1.Snapshot")]
pub struct Snapshot {
    /// The height at which the snapshot was taken
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: u64,
    /// The application-specific snapshot format
    #[prost(uint32, tag = "2")]
    pub format: u32,
    /// Number of chunks in the snapshot
    #[prost(uint32, tag = "3")]
    pub chunks: u32,
    /// Arbitrary snapshot hash, equal only if identical
    #[prost(bytes = "vec", tag = "4")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    /// Arbitrary application metadata
    #[prost(bytes = "vec", tag = "5")]
    pub metadata: ::prost::alloc::vec::Vec<u8>,
}
/// Type of the transaction check request.
///
/// This enumeration is incompatible with the CheckTxType definition in
/// cometbft.abci.v1beta1 and therefore shall not be used in encoding with the same
/// field number.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum CheckTxType {
    /// Unknown
    Unknown = 0,
    /// Recheck (2nd, 3rd, etc.)
    Recheck = 1,
    /// Check (1st time)
    Check = 2,
}
impl CheckTxType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unknown => "CHECK_TX_TYPE_UNKNOWN",
            Self::Recheck => "CHECK_TX_TYPE_RECHECK",
            Self::Check => "CHECK_TX_TYPE_CHECK",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CHECK_TX_TYPE_UNKNOWN" => Some(Self::Unknown),
            "CHECK_TX_TYPE_RECHECK" => Some(Self::Recheck),
            "CHECK_TX_TYPE_CHECK" => Some(Self::Check),
            _ => None,
        }
    }
}
/// The result of offering a snapshot.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum OfferSnapshotResult {
    /// Unknown result, abort all snapshot restoration
    Unknown = 0,
    /// Snapshot accepted, apply chunks
    Accept = 1,
    /// Abort all snapshot restoration
    Abort = 2,
    /// Reject this specific snapshot, try others
    Reject = 3,
    /// Reject all snapshots of this format, try others
    RejectFormat = 4,
    /// Reject all snapshots from the sender(s), try others
    RejectSender = 5,
}
impl OfferSnapshotResult {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unknown => "OFFER_SNAPSHOT_RESULT_UNKNOWN",
            Self::Accept => "OFFER_SNAPSHOT_RESULT_ACCEPT",
            Self::Abort => "OFFER_SNAPSHOT_RESULT_ABORT",
            Self::Reject => "OFFER_SNAPSHOT_RESULT_REJECT",
            Self::RejectFormat => "OFFER_SNAPSHOT_RESULT_REJECT_FORMAT",
            Self::RejectSender => "OFFER_SNAPSHOT_RESULT_REJECT_SENDER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OFFER_SNAPSHOT_RESULT_UNKNOWN" => Some(Self::Unknown),
            "OFFER_SNAPSHOT_RESULT_ACCEPT" => Some(Self::Accept),
            "OFFER_SNAPSHOT_RESULT_ABORT" => Some(Self::Abort),
            "OFFER_SNAPSHOT_RESULT_REJECT" => Some(Self::Reject),
            "OFFER_SNAPSHOT_RESULT_REJECT_FORMAT" => Some(Self::RejectFormat),
            "OFFER_SNAPSHOT_RESULT_REJECT_SENDER" => Some(Self::RejectSender),
            _ => None,
        }
    }
}
/// The result of applying a snapshot chunk.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum ApplySnapshotChunkResult {
    /// Unknown result, abort all snapshot restoration
    Unknown = 0,
    /// Chunk successfully accepted
    Accept = 1,
    /// Abort all snapshot restoration
    Abort = 2,
    /// Retry chunk (combine with refetch and reject)
    Retry = 3,
    /// Retry snapshot (combine with refetch and reject)
    RetrySnapshot = 4,
    /// Reject this snapshot, try others
    RejectSnapshot = 5,
}
impl ApplySnapshotChunkResult {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unknown => "APPLY_SNAPSHOT_CHUNK_RESULT_UNKNOWN",
            Self::Accept => "APPLY_SNAPSHOT_CHUNK_RESULT_ACCEPT",
            Self::Abort => "APPLY_SNAPSHOT_CHUNK_RESULT_ABORT",
            Self::Retry => "APPLY_SNAPSHOT_CHUNK_RESULT_RETRY",
            Self::RetrySnapshot => "APPLY_SNAPSHOT_CHUNK_RESULT_RETRY_SNAPSHOT",
            Self::RejectSnapshot => "APPLY_SNAPSHOT_CHUNK_RESULT_REJECT_SNAPSHOT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "APPLY_SNAPSHOT_CHUNK_RESULT_UNKNOWN" => Some(Self::Unknown),
            "APPLY_SNAPSHOT_CHUNK_RESULT_ACCEPT" => Some(Self::Accept),
            "APPLY_SNAPSHOT_CHUNK_RESULT_ABORT" => Some(Self::Abort),
            "APPLY_SNAPSHOT_CHUNK_RESULT_RETRY" => Some(Self::Retry),
            "APPLY_SNAPSHOT_CHUNK_RESULT_RETRY_SNAPSHOT" => Some(Self::RetrySnapshot),
            "APPLY_SNAPSHOT_CHUNK_RESULT_REJECT_SNAPSHOT" => Some(Self::RejectSnapshot),
            _ => None,
        }
    }
}
/// ProcessProposalStatus is the status of the proposal processing.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum ProcessProposalStatus {
    /// Unknown
    Unknown = 0,
    /// Accepted
    Accept = 1,
    /// Rejected
    Reject = 2,
}
impl ProcessProposalStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unknown => "PROCESS_PROPOSAL_STATUS_UNKNOWN",
            Self::Accept => "PROCESS_PROPOSAL_STATUS_ACCEPT",
            Self::Reject => "PROCESS_PROPOSAL_STATUS_REJECT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PROCESS_PROPOSAL_STATUS_UNKNOWN" => Some(Self::Unknown),
            "PROCESS_PROPOSAL_STATUS_ACCEPT" => Some(Self::Accept),
            "PROCESS_PROPOSAL_STATUS_REJECT" => Some(Self::Reject),
            _ => None,
        }
    }
}
/// VerifyVoteExtensionStatus is the status of the vote extension verification.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum VerifyVoteExtensionStatus {
    /// Unknown
    Unknown = 0,
    /// Accepted
    Accept = 1,
    /// Rejecting the vote extension will reject the entire precommit by the sender.
    /// Incorrectly implementing this thus has liveness implications as it may affect
    /// CometBFT's ability to receive 2/3+ valid votes to finalize the block.
    /// Honest nodes should never be rejected.
    Reject = 2,
}
impl VerifyVoteExtensionStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unknown => "VERIFY_VOTE_EXTENSION_STATUS_UNKNOWN",
            Self::Accept => "VERIFY_VOTE_EXTENSION_STATUS_ACCEPT",
            Self::Reject => "VERIFY_VOTE_EXTENSION_STATUS_REJECT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "VERIFY_VOTE_EXTENSION_STATUS_UNKNOWN" => Some(Self::Unknown),
            "VERIFY_VOTE_EXTENSION_STATUS_ACCEPT" => Some(Self::Accept),
            "VERIFY_VOTE_EXTENSION_STATUS_REJECT" => Some(Self::Reject),
            _ => None,
        }
    }
}
/// The type of misbehavior committed by a validator.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum MisbehaviorType {
    /// Unknown
    Unknown = 0,
    /// Duplicate vote
    DuplicateVote = 1,
    /// Light client attack
    LightClientAttack = 2,
}
impl MisbehaviorType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unknown => "MISBEHAVIOR_TYPE_UNKNOWN",
            Self::DuplicateVote => "MISBEHAVIOR_TYPE_DUPLICATE_VOTE",
            Self::LightClientAttack => "MISBEHAVIOR_TYPE_LIGHT_CLIENT_ATTACK",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MISBEHAVIOR_TYPE_UNKNOWN" => Some(Self::Unknown),
            "MISBEHAVIOR_TYPE_DUPLICATE_VOTE" => Some(Self::DuplicateVote),
            "MISBEHAVIOR_TYPE_LIGHT_CLIENT_ATTACK" => Some(Self::LightClientAttack),
            _ => None,
        }
    }
}
