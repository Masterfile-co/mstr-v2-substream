// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MasterfileEvent {
    #[prost(message, optional, tag="101")]
    pub metadata: ::core::option::Option<super::super::common::v1::TransactionMetadata>,
    #[prost(uint64, tag="200")]
    pub ordinal: u64,
    #[prost(oneof="masterfile_event::Event", tags="1, 10")]
    pub event: ::core::option::Option<masterfile_event::Event>,
}
/// Nested message and enum types in `MasterfileEvent`.
pub mod masterfile_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        #[prost(message, tag="1")]
        SafeFactory(super::super::super::safe::v1::SafeFactoryEvent),
        #[prost(message, tag="10")]
        Safe(super::super::super::safe::v1::SafeEvent),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MasterfileEvents {
    #[prost(message, repeated, tag="1")]
    pub events: ::prost::alloc::vec::Vec<MasterfileEvent>,
}
// @@protoc_insertion_point(module)
