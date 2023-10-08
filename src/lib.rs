use pb::masterfile::events::v1::{masterfile_event, MasterfileEvent, MasterfileEvents};
use pb::masterfile::safe::v1::{Safe, SafeEvent, Safes};
use serde::{Deserialize, Serialize};
use serde_qs;
use substreams::prelude::*;
use substreams::{errors::Error, Hex};
use substreams_ethereum::pb::eth::v2::Block;
use utils::{extract_metadata, pretty_hex};

pub mod abi;
pub mod pb;
pub mod safe;
pub mod safe_factory;
pub mod utils;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct MapSafesParams {
    factory: String,
}

#[substreams::handlers::map]
pub fn map_safes(param: String, block: Block) -> Result<Safes, Error> {
    let params: MapSafesParams = serde_qs::from_str(&param).unwrap();

    let factory_address = Hex::decode(params.factory).unwrap();

    Ok(Safes {
        safes: block
            .events::<abi::SafeFactory::events::SafeDeployed>(&[&factory_address])
            .filter_map(|(event, log)| {
                Some(Safe {
                    address: pretty_hex(&event.safe),
                    ordinal: log.block_index() as u64,
                })
            })
            .collect(),
    })
}

#[substreams::handlers::store]
fn store_safes(safes: Safes, store: StoreSetProto<Safe>) {
    for safe in safes.safes {
        store.set(safe.ordinal, &safe.address, &safe);
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct MapEventsParams {
    chain_id: u64,
}

#[substreams::handlers::map]
fn map_events(
    param: String,
    safes: StoreGetProto<Safe>,
    block: Block,
) -> Result<MasterfileEvents, Error> {
    let params: MapEventsParams = serde_qs::from_str(&param).unwrap();

    let mut events = vec![];

    for log in block.logs() {
        let address = pretty_hex(&log.address());
        let metadata = extract_metadata(&log, &block);
        let ordinal = log.block_index() as u64;

        if let Some(safe) = safes.get_last(&address) {
            if let Some(event) = safe::extract_safe_event(&log, &params.chain_id) {
                events.push(MasterfileEvent {
                    ordinal,
                    metadata: metadata.clone(),
                    event: Some(masterfile_event::Event::Safe(SafeEvent {
                        safe_address: safe.address.clone(),
                        event: Some(event),
                    })),
                });
            }
        }
    }

    Ok(MasterfileEvents { events })
}
