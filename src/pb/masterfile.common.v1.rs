// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionMetadata {
    #[prost(string, tag="1")]
    pub tx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub block_number: u64,
    #[prost(uint64, tag="3")]
    pub block_timestamp: u64,
    #[prost(string, tag="4")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub from: ::prost::alloc::string::String,
    #[prost(uint32, tag="6")]
    pub log_index: u32,
    #[prost(uint32, tag="7")]
    pub block_index: u32,
    #[prost(string, tag="8")]
    pub address: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
