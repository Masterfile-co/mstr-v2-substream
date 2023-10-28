use substreams_ethereum::{block_view::LogView, Event};

use crate::{
    abi::MasterfileDropFactory::events::DropDeployed, pb::masterfile::drop::v1::drop_factory_event,
    utils::pretty_hex,
};

pub fn extract_drop_factory_event(log: &LogView) -> Option<drop_factory_event::Event> {
    if let Some(event) = DropDeployed::match_and_decode(log) {
        return Some(drop_factory_event::Event::DropDeployed(
            drop_factory_event::DropDeployed {
                drop: pretty_hex(&event.drop),
                safe: pretty_hex(&event.safe),
            },
        ));
    }

    None
}
