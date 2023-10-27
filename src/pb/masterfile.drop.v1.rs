// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DropEvent {
    #[prost(string, tag="1")]
    pub drop_address: ::prost::alloc::string::String,
    #[prost(oneof="drop_event::Event", tags="2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18")]
    pub event: ::core::option::Option<drop_event::Event>,
}
/// Nested message and enum types in `DropEvent`.
pub mod drop_event {
    // //////////////////////////////////////////////////////////////////////////
    // DROP
    // //////////////////////////////////////////////////////////////////////////

    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BundlePurchased {
        #[prost(string, tag="1")]
        pub funder: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub recipient: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub quantity: ::prost::alloc::string::String,
        #[prost(string, tag="4")]
        pub total_price: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BundleSet {
        #[prost(string, tag="1")]
        pub quantity: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub price: ::prost::alloc::string::String,
        #[prost(uint64, repeated, tag="3")]
        pub token_ids: ::prost::alloc::vec::Vec<u64>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DetailsSet {
        #[prost(string, tag="1")]
        pub usdc: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub revenue_recipient: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub randomness_provider: ::prost::alloc::string::String,
        #[prost(uint64, tag="4")]
        pub start_date: u64,
        #[prost(uint64, tag="5")]
        pub end_date: u64,
        #[prost(uint64, tag="6")]
        pub num_editions: u64,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EditionProbabilitySet {
        #[prost(uint64, tag="1")]
        pub token_id: u64,
        #[prost(uint64, tag="2")]
        pub probability: u64,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EditionPurchased {
        #[prost(string, tag="1")]
        pub funder: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub recipient: ::prost::alloc::string::String,
        #[prost(uint64, tag="3")]
        pub token_id: u64,
        #[prost(string, tag="4")]
        pub quantity: ::prost::alloc::string::String,
        #[prost(string, tag="5")]
        pub total_price: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EditionRedeemed {
        #[prost(string, tag="1")]
        pub request_id: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub redeemer: ::prost::alloc::string::String,
        #[prost(uint64, tag="3")]
        pub token_id: u64,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EditionSet {
        #[prost(uint64, tag="1")]
        pub token_id: u64,
        #[prost(string, tag="2")]
        pub price: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub quantity: ::prost::alloc::string::String,
        #[prost(string, tag="4")]
        pub arweave_hash: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MysteryBoxPurchased {
        #[prost(string, tag="1")]
        pub funder: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub recipient: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub quantity: ::prost::alloc::string::String,
        #[prost(string, tag="4")]
        pub total_price: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MysteryBoxRevealRequested {
        #[prost(string, tag="1")]
        pub request_id: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub requester: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub nonce: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MysteryBoxRevealed {
        #[prost(string, tag="1")]
        pub request_id: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub recipient: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub randomness: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MysteryBoxSet {
        #[prost(string, tag="1")]
        pub quantity: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub price: ::prost::alloc::string::String,
        #[prost(uint64, tag="3")]
        pub redemptions: u64,
    }
    // //////////////////////////////////////////////////////////////////////////
    // ERC-1155
    // //////////////////////////////////////////////////////////////////////////

    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ApprovalForAll {
        #[prost(string, tag="1")]
        pub account: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub operator: ::prost::alloc::string::String,
        #[prost(bool, tag="3")]
        pub approved: bool,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TransferBatch {
        #[prost(string, tag="1")]
        pub operator: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub from: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub to: ::prost::alloc::string::String,
        #[prost(uint64, repeated, tag="4")]
        pub ids: ::prost::alloc::vec::Vec<u64>,
        #[prost(uint64, repeated, tag="5")]
        pub values: ::prost::alloc::vec::Vec<u64>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TransferSingle {
        #[prost(string, tag="1")]
        pub operator: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub from: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub to: ::prost::alloc::string::String,
        #[prost(uint64, tag="4")]
        pub id: u64,
        #[prost(uint64, tag="5")]
        pub value: u64,
    }
    // //////////////////////////////////////////////////////////////////////////
    // Native Meta Transaction
    // //////////////////////////////////////////////////////////////////////////

    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MetaTransactionExecuted {
        #[prost(string, tag="1")]
        pub user_address: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub relayer_address: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub function_signature: ::prost::alloc::string::String,
    }
    // //////////////////////////////////////////////////////////////////////////
    // Initializable
    // //////////////////////////////////////////////////////////////////////////

    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Initialized {
    }
    // //////////////////////////////////////////////////////////////////////////
    // Ownable
    // //////////////////////////////////////////////////////////////////////////

    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OwnershipTransferred {
        #[prost(string, tag="1")]
        pub previous_owner: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub new_owner: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        /// //////////////////////////////////////////////////////////////////////////
        /// DROP
        /// //////////////////////////////////////////////////////////////////////////
        #[prost(message, tag="2")]
        BundlePurchased(BundlePurchased),
        #[prost(message, tag="3")]
        BundleSet(BundleSet),
        #[prost(message, tag="4")]
        DetailsSet(DetailsSet),
        #[prost(message, tag="5")]
        EditionProbabilitySet(EditionProbabilitySet),
        #[prost(message, tag="6")]
        EditionPurchased(EditionPurchased),
        #[prost(message, tag="7")]
        EditionRedeemed(EditionRedeemed),
        #[prost(message, tag="8")]
        EditionSet(EditionSet),
        #[prost(message, tag="9")]
        MysteryBoxPurchased(MysteryBoxPurchased),
        #[prost(message, tag="10")]
        MysteryBoxRevealRequested(MysteryBoxRevealRequested),
        #[prost(message, tag="11")]
        MysteryBoxRevealed(MysteryBoxRevealed),
        #[prost(message, tag="12")]
        MysteryBoxSet(MysteryBoxSet),
        /// //////////////////////////////////////////////////////////////////////////
        /// ERC-1155
        /// //////////////////////////////////////////////////////////////////////////
        #[prost(message, tag="13")]
        ApprovalForAll(ApprovalForAll),
        #[prost(message, tag="14")]
        TransferBatch(TransferBatch),
        #[prost(message, tag="15")]
        TransferSingle(TransferSingle),
        /// //////////////////////////////////////////////////////////////////////////
        /// Native Meta Transaction
        /// //////////////////////////////////////////////////////////////////////////
        #[prost(message, tag="16")]
        MetaTransactionExecuted(MetaTransactionExecuted),
        /// //////////////////////////////////////////////////////////////////////////
        /// Initializable
        /// //////////////////////////////////////////////////////////////////////////
        #[prost(message, tag="17")]
        Initialized(Initialized),
        /// //////////////////////////////////////////////////////////////////////////
        /// Ownable
        /// //////////////////////////////////////////////////////////////////////////
        #[prost(message, tag="18")]
        OwnershipTransferred(OwnershipTransferred),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DropFactoryEvent {
    #[prost(string, tag="1")]
    pub factory_address: ::prost::alloc::string::String,
    #[prost(oneof="drop_factory_event::Event", tags="2")]
    pub event: ::core::option::Option<drop_factory_event::Event>,
}
/// Nested message and enum types in `DropFactoryEvent`.
pub mod drop_factory_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DropDeployed {
        #[prost(string, tag="1")]
        pub drop: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub safe: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        #[prost(message, tag="2")]
        DropDeployed(DropDeployed),
    }
}
// @@protoc_insertion_point(module)
