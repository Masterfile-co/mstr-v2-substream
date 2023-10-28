use abi::Registry;
use pb::masterfile::drop::v1::{DropEvent, DropFactoryEvent};
use pb::masterfile::events::v1::{masterfile_event, MasterfileEvent, MasterfileEvents};

use pb::masterfile::registry::v1::{
    contract_type, deployment_type, Deployment, DeploymentType, Deployments, RegistryEvent,
};
use pb::masterfile::safe::v1::{SafeEvent, SafeFactoryEvent};
use serde::{Deserialize, Serialize};
use serde_qs;
use substreams::errors::Error;
use substreams::prelude::*;
use substreams_ethereum::pb::eth::v2::Block;
use substreams_ethereum::Event;
use utils::{extract_metadata, pretty_hex};

pub mod abi;
pub mod drop;
pub mod drop_factory;
pub mod pb;
pub mod registry;
pub mod safe;
pub mod safe_factory;
pub mod utils;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct MapRegistryParams {
    registry: String,
}

#[substreams::handlers::map]
pub fn map_registry(param: String, block: Block) -> Result<Deployments, Error> {
    let mut deployments = vec![];

    let params: MapRegistryParams = serde_qs::from_str(&param).unwrap();

    for log in block.logs() {
        if pretty_hex(&log.address()) == params.registry {
            ////////////////////////////////////////////////////////////////////////////
            //                             	  Factories                               //
            ////////////////////////////////////////////////////////////////////////////
            if let Some(event) = Registry::events::FactoryAdded::match_and_decode(log) {
                if let Some(contract_type) = registry::map_contract_type(&pretty_hex(&event.name)) {
                    deployments.push(Deployment {
                        address: pretty_hex(&event.factory),
                        contract_type: Some(contract_type),
                        deployment_type: Some(DeploymentType {
                            deployment_type: Some(deployment_type::DeploymentType::Factory(
                                deployment_type::Factory {},
                            )),
                        }),
                        ordinal: log.block_index() as u64,
                    })
                }
            }
            ////////////////////////////////////////////////////////////////////////////
            //                             	 Deployments                              //
            ////////////////////////////////////////////////////////////////////////////
            if let Some(event) = Registry::events::DeploymentAdded::match_and_decode(log) {
                if let Some(contract_type) = registry::map_contract_type(&pretty_hex(&event.name)) {
                    deployments.push(Deployment {
                        address: pretty_hex(&event.deployment),
                        contract_type: Some(contract_type),
                        deployment_type: Some(DeploymentType {
                            deployment_type: Some(deployment_type::DeploymentType::Contract(
                                deployment_type::Contract {},
                            )),
                        }),
                        ordinal: log.block_index() as u64,
                    })
                }
            }
        }
    }

    Ok(Deployments { deployments })
}

#[substreams::handlers::store]
fn store_deployments(deployments: Deployments, store: StoreSetProto<Deployment>) {
    for deployment in deployments.deployments {
        store.set(deployment.ordinal, &deployment.address, &deployment);
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct MapEventsParams {
    registry_address: String,
    chain_id: u64,
}

#[substreams::handlers::map]
fn map_events(
    param: String,
    deployments: StoreGetProto<Deployment>,
    block: Block,
) -> Result<MasterfileEvents, Error> {
    let mut events = vec![];

    let params: MapEventsParams = serde_qs::from_str(&param).unwrap();

    for log in block.logs() {
        let address = pretty_hex(&log.address());
        let metadata = extract_metadata(&log, &block);
        let ordinal = log.block_index() as u64;

        ////////////////////////////////////////////////////////////////////////////
        //                             	  Registry                                //
        ////////////////////////////////////////////////////////////////////////////

        if address == params.registry_address {
            events.push(MasterfileEvent {
                metadata,
                ordinal,
                event: Some(masterfile_event::Event::Registry(RegistryEvent {
                    event: registry::extract_registry_event(&log),
                })),
            })
        } else if let Some(deployment) = deployments.get_last(&address) {
            match deployment.deployment_type.unwrap().deployment_type.unwrap() {
                ////////////////////////////////////////////////////////////////////////////
                //                             	  Factories                               //
                ////////////////////////////////////////////////////////////////////////////
                deployment_type::DeploymentType::Factory(_) => {
                    match deployment.contract_type.unwrap().contract_type.unwrap() {
                        ////////////////////////////////////////////////////////////////////////////
                        //                             	 Safe Factory                             //
                        ////////////////////////////////////////////////////////////////////////////
                        contract_type::ContractType::Safe(_) => {
                            if let Some(event) = safe_factory::extract_safe_factory_event(&log) {
                                events.push(MasterfileEvent {
                                    event: Some(masterfile_event::Event::SafeFactory(
                                        SafeFactoryEvent {
                                            event: Some(event),
                                            factory_address: address.clone(),
                                        },
                                    )),
                                    ordinal,
                                    metadata,
                                })
                            }
                        }
                        ////////////////////////////////////////////////////////////////////////////
                        //                             	 Drop Factory                             //
                        ////////////////////////////////////////////////////////////////////////////
                        contract_type::ContractType::Drop(_) => {
                            if let Some(event) = drop_factory::extract_drop_factory_event(&log) {
                                events.push(MasterfileEvent {
                                    event: Some(masterfile_event::Event::DropFactory(
                                        DropFactoryEvent {
                                            event: Some(event),
                                            factory_address: address.clone(),
                                        },
                                    )),
                                    ordinal,
                                    metadata,
                                })
                            }
                        }
                    }
                }

                ////////////////////////////////////////////////////////////////////////////
                //                             	 Deployments                              //
                ////////////////////////////////////////////////////////////////////////////
                deployment_type::DeploymentType::Contract(_) => {
                    match deployment.contract_type.unwrap().contract_type.unwrap() {
                        ////////////////////////////////////////////////////////////////////////////
                        //                             	 Safe                                    //
                        ////////////////////////////////////////////////////////////////////////////
                        contract_type::ContractType::Safe(_) => {
                            if let Some(event) = safe::extract_safe_event(&log, &params.chain_id) {
                                events.push(MasterfileEvent {
                                    event: Some(masterfile_event::Event::Safe(SafeEvent {
                                        event: Some(event),
                                        safe_address: address.clone(),
                                    })),
                                    ordinal,
                                    metadata,
                                })
                            }
                            // Add results for each safe transaction
                            safe::hydrate_tx_results(&log, &mut events);
                        }
                        ////////////////////////////////////////////////////////////////////////////
                        //                             	 Drop                                    //
                        ////////////////////////////////////////////////////////////////////////////
                        contract_type::ContractType::Drop(_) => {
                            if let Some(event) = drop::extract_drop_event(&log) {
                                events.push(MasterfileEvent {
                                    event: Some(masterfile_event::Event::Drop(DropEvent {
                                        event: Some(event),
                                        drop_address: address.clone(),
                                    })),
                                    ordinal,
                                    metadata,
                                })
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(MasterfileEvents { events })
}
