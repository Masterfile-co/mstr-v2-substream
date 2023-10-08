// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SafeEvent {
    #[prost(string, tag="14")]
    pub safe_address: ::prost::alloc::string::String,
    #[prost(oneof="safe_event::Event", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13")]
    pub event: ::core::option::Option<safe_event::Event>,
}
/// Nested message and enum types in `SafeEvent`.
pub mod safe_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AddedOwner {
        #[prost(string, tag="1")]
        pub owner: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RemovedOwner {
        #[prost(string, tag="1")]
        pub owner: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ApproveHash {
        #[prost(string, tag="1")]
        pub owner: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub hash: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ChangedFallbackHandler {
        #[prost(string, tag="1")]
        pub handler: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ChangedGuard {
        #[prost(string, tag="1")]
        pub guard: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ChangedThreshold {
        #[prost(uint64, tag="1")]
        pub threshold: u64,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DisabledModule {
        #[prost(string, tag="1")]
        pub module: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EnabledModule {
        #[prost(string, tag="1")]
        pub module: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExecutionFailure {
        #[prost(string, tag="1")]
        pub safe_tx_hash: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub payment: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExecutionFromModuleFailure {
        #[prost(string, tag="1")]
        pub module: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExecutionFromModuleSuccess {
        #[prost(string, tag="1")]
        pub module: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExecutionSuccess {
        #[prost(string, tag="1")]
        pub safe_tx_hash: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub payment: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SafeModuleTransaction {
        #[prost(string, tag="1")]
        pub module: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub to: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub value: ::prost::alloc::string::String,
        #[prost(string, tag="4")]
        pub data: ::prost::alloc::string::String,
        /// Maybe decode this into enum
        #[prost(uint64, tag="5")]
        pub operation: u64,
        #[prost(oneof="safe_module_transaction::Result", tags="6, 7")]
        pub result: ::core::option::Option<safe_module_transaction::Result>,
    }
    /// Nested message and enum types in `SafeModuleTransaction`.
    pub mod safe_module_transaction {
        #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Result {
            #[prost(message, tag="6")]
            ExecutionFailure(super::ExecutionFromModuleFailure),
            #[prost(message, tag="7")]
            ExecutionSuccess(super::ExecutionFromModuleSuccess),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SafeMultiSigTransaction {
        #[prost(string, tag="1")]
        pub to: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub value: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub data: ::prost::alloc::string::String,
        /// Maybe decode this into enum
        #[prost(uint64, tag="4")]
        pub operation: u64,
        #[prost(string, tag="5")]
        pub safe_tx_gas: ::prost::alloc::string::String,
        #[prost(string, tag="6")]
        pub base_gas: ::prost::alloc::string::String,
        #[prost(string, tag="7")]
        pub gas_price: ::prost::alloc::string::String,
        #[prost(string, tag="8")]
        pub gas_token: ::prost::alloc::string::String,
        #[prost(string, tag="9")]
        pub refund_receiver: ::prost::alloc::string::String,
        #[prost(string, tag="10")]
        pub signatures: ::prost::alloc::string::String,
        #[prost(string, tag="11")]
        pub additional_info: ::prost::alloc::string::String,
        #[prost(string, tag="12")]
        pub safe_tx_hash: ::prost::alloc::string::String,
        #[prost(uint64, tag="13")]
        pub nonce: u64,
        #[prost(oneof="safe_multi_sig_transaction::Result", tags="14, 15")]
        pub result: ::core::option::Option<safe_multi_sig_transaction::Result>,
    }
    /// Nested message and enum types in `SafeMultiSigTransaction`.
    pub mod safe_multi_sig_transaction {
        #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Result {
            #[prost(message, tag="14")]
            ExecutionFailure(super::ExecutionFailure),
            #[prost(message, tag="15")]
            ExecutionSuccess(super::ExecutionSuccess),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SafeReceived {
        #[prost(string, tag="1")]
        pub sender: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub value: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SafeSetup {
        #[prost(string, tag="1")]
        pub initiator: ::prost::alloc::string::String,
        #[prost(uint64, tag="2")]
        pub threshold: u64,
        #[prost(string, tag="3")]
        pub initializer: ::prost::alloc::string::String,
        #[prost(string, tag="4")]
        pub fallback_handler: ::prost::alloc::string::String,
        #[prost(message, repeated, tag="5")]
        pub owners: ::prost::alloc::vec::Vec<SafeOwner>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SafeOwner {
        #[prost(string, tag="1")]
        pub address: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SafeMsg {
        #[prost(string, tag="1")]
        pub msg_hash: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        #[prost(message, tag="1")]
        AddedOwner(AddedOwner),
        #[prost(message, tag="2")]
        RemovedOwner(RemovedOwner),
        #[prost(message, tag="3")]
        ApproveHash(ApproveHash),
        #[prost(message, tag="4")]
        ChangedFallbackHandler(ChangedFallbackHandler),
        #[prost(message, tag="5")]
        ChangedGuard(ChangedGuard),
        #[prost(message, tag="6")]
        ChangedThreshold(ChangedThreshold),
        #[prost(message, tag="7")]
        DisabledModule(DisabledModule),
        #[prost(message, tag="8")]
        EnabledModule(EnabledModule),
        #[prost(message, tag="9")]
        SafeModuleTransaction(SafeModuleTransaction),
        #[prost(message, tag="10")]
        SafeMultisigTransaction(SafeMultiSigTransaction),
        #[prost(message, tag="11")]
        SafeReceived(SafeReceived),
        #[prost(message, tag="12")]
        SafeSetup(SafeSetup),
        #[prost(message, tag="13")]
        SafeMsg(SafeMsg),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SafeFactoryEvent {
    #[prost(string, tag="100")]
    pub factory_address: ::prost::alloc::string::String,
    #[prost(oneof="safe_factory_event::Event", tags="1")]
    pub event: ::core::option::Option<safe_factory_event::Event>,
}
/// Nested message and enum types in `SafeFactoryEvent`.
pub mod safe_factory_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SafeDeployed {
        #[prost(string, tag="1")]
        pub safe: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        #[prost(message, tag="1")]
        SafeDeployed(SafeDeployed),
    }
}
// @@protoc_insertion_point(module)
