// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MasterfileEvent {
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::common::v1::TransactionMetadata>,
    #[prost(uint64, tag="2")]
    pub ordinal: u64,
    #[prost(oneof="masterfile_event::Event", tags="3, 10, 20, 100, 101")]
    pub event: ::core::option::Option<masterfile_event::Event>,
}
/// Nested message and enum types in `MasterfileEvent`.
pub mod masterfile_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        #[prost(message, tag="3")]
        Registry(super::super::super::registry::v1::RegistryEvent),
        #[prost(message, tag="10")]
        SafeFactory(super::super::super::safe::v1::SafeFactoryEvent),
        #[prost(message, tag="20")]
        DropFactory(super::super::super::drop::v1::DropFactoryEvent),
        #[prost(message, tag="100")]
        Safe(super::super::super::safe::v1::SafeEvent),
        #[prost(message, tag="101")]
        Drop(super::super::super::drop::v1::DropEvent),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MasterfileEvents {
    #[prost(message, repeated, tag="1")]
    pub events: ::prost::alloc::vec::Vec<MasterfileEvent>,
}
// @@protoc_insertion_point(module)
