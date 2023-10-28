// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeploymentType {
    #[prost(oneof="deployment_type::DeploymentType", tags="1, 2")]
    pub deployment_type: ::core::option::Option<deployment_type::DeploymentType>,
}
/// Nested message and enum types in `DeploymentType`.
pub mod deployment_type {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Factory {
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Contract {
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DeploymentType {
        #[prost(message, tag="1")]
        Factory(Factory),
        #[prost(message, tag="2")]
        Contract(Contract),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractType {
    #[prost(oneof="contract_type::ContractType", tags="1, 2")]
    pub contract_type: ::core::option::Option<contract_type::ContractType>,
}
/// Nested message and enum types in `ContractType`.
pub mod contract_type {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Safe {
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Drop {
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ContractType {
        #[prost(message, tag="1")]
        Safe(Safe),
        #[prost(message, tag="2")]
        Drop(Drop),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Deployment {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub ordinal: u64,
    #[prost(message, optional, tag="3")]
    pub deployment_type: ::core::option::Option<DeploymentType>,
    #[prost(message, optional, tag="4")]
    pub contract_type: ::core::option::Option<ContractType>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Deployments {
    #[prost(message, repeated, tag="1")]
    pub deployments: ::prost::alloc::vec::Vec<Deployment>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegistryEvent {
    #[prost(oneof="registry_event::Event", tags="1, 2, 3, 4, 5, 6, 7")]
    pub event: ::core::option::Option<registry_event::Event>,
}
/// Nested message and enum types in `RegistryEvent`.
pub mod registry_event {
    // //////////////////////////////////////////////////////////////////////////
    // Registry
    // //////////////////////////////////////////////////////////////////////////

    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DeploymentAdded {
        #[prost(string, tag="1")]
        pub name: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub deployment: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DeploymentRevoked {
        #[prost(string, tag="1")]
        pub name: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub deployment: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FactoryAdded {
        #[prost(string, tag="1")]
        pub name: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub factory: ::prost::alloc::string::String,
        #[prost(uint64, tag="3")]
        pub version: u64,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FactoryRevoked {
        #[prost(string, tag="1")]
        pub name: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub factory: ::prost::alloc::string::String,
        #[prost(uint64, tag="3")]
        pub version: u64,
    }
    // //////////////////////////////////////////////////////////////////////////
    // Access Control
    // //////////////////////////////////////////////////////////////////////////

    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RoleAdminChanged {
        #[prost(string, tag="1")]
        pub role: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub previous_admin_role: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub new_admin_role: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RoleGranted {
        #[prost(string, tag="1")]
        pub role: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub account: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub sender: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RoleRevoked {
        #[prost(string, tag="1")]
        pub role: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub account: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub sender: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        /// //////////////////////////////////////////////////////////////////////////
        /// Registry
        /// //////////////////////////////////////////////////////////////////////////
        #[prost(message, tag="1")]
        DeploymentAdded(DeploymentAdded),
        #[prost(message, tag="2")]
        DeploymentRevoked(DeploymentRevoked),
        #[prost(message, tag="3")]
        FactoryAdded(FactoryAdded),
        #[prost(message, tag="4")]
        FactoryRevoked(FactoryRevoked),
        /// //////////////////////////////////////////////////////////////////////////
        /// Access Control
        /// //////////////////////////////////////////////////////////////////////////
        #[prost(message, tag="5")]
        RoleAdminChanged(RoleAdminChanged),
        #[prost(message, tag="6")]
        RoleGranted(RoleGranted),
        #[prost(message, tag="7")]
        RoleRevoked(RoleRevoked),
    }
}
// @@protoc_insertion_point(module)
