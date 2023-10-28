use substreams_ethereum::{block_view::LogView, Event};

use crate::{
    abi::Registry::events::{
        DeploymentAdded, DeploymentRevoked, FactoryAdded, FactoryRevoked, RoleAdminChanged,
        RoleGranted, RoleRevoked,
    },
    pb::masterfile::registry::v1::{
        contract_type::{self, Drop, Safe},
        registry_event, ContractType,
    },
    utils::{pretty_hex, MASTERFILE_DROP, MASTERFILE_SAFE},
};

pub fn map_contract_type(name: &str) -> Option<ContractType> {
    match name {
        MASTERFILE_SAFE => Some(ContractType {
            contract_type: Some(contract_type::ContractType::Safe(Safe {})),
        }),

        MASTERFILE_DROP => Some(ContractType {
            contract_type: Some(contract_type::ContractType::Drop(Drop {})),
        }),
        _ => None,
    }
}

pub fn extract_registry_event(log: &LogView) -> Option<registry_event::Event> {
    if let Some(event) = DeploymentAdded::match_and_decode(log) {
        return Some(registry_event::Event::DeploymentAdded(
            registry_event::DeploymentAdded {
                deployment: pretty_hex(&event.deployment),
                name: pretty_hex(&event.name),
            },
        ));
    }

    if let Some(event) = DeploymentRevoked::match_and_decode(log) {
        return Some(registry_event::Event::DeploymentRevoked(
            registry_event::DeploymentRevoked {
                deployment: pretty_hex(&event.deployment),
                name: pretty_hex(&event.name),
            },
        ));
    }

    if let Some(event) = FactoryAdded::match_and_decode(log) {
        return Some(registry_event::Event::FactoryAdded(
            registry_event::FactoryAdded {
                factory: pretty_hex(&event.factory),
                name: pretty_hex(&event.name),
                version: event.version.to_u64(),
            },
        ));
    }

    if let Some(event) = FactoryRevoked::match_and_decode(log) {
        return Some(registry_event::Event::FactoryRevoked(
            registry_event::FactoryRevoked {
                factory: pretty_hex(&event.factory),
                name: pretty_hex(&event.name),
                version: event.version.to_u64(),
            },
        ));
    }

    if let Some(event) = RoleAdminChanged::match_and_decode(log) {
        return Some(registry_event::Event::RoleAdminChanged(
            registry_event::RoleAdminChanged {
                role: pretty_hex(&event.role),
                previous_admin_role: pretty_hex(&event.previous_admin_role),
                new_admin_role: pretty_hex(&event.new_admin_role),
            },
        ));
    }

    if let Some(event) = RoleGranted::match_and_decode(log) {
        return Some(registry_event::Event::RoleGranted(
            registry_event::RoleGranted {
                role: pretty_hex(&event.role),
                account: pretty_hex(&event.account),
                sender: pretty_hex(&event.sender),
            },
        ));
    }

    if let Some(event) = RoleRevoked::match_and_decode(log) {
        return Some(registry_event::Event::RoleRevoked(
            registry_event::RoleRevoked {
                role: pretty_hex(&event.role),
                account: pretty_hex(&event.account),
                sender: pretty_hex(&event.sender),
            },
        ));
    }

    None
}
