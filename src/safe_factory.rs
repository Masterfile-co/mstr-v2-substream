use substreams_ethereum::{block_view::LogView, Event};

use crate::{
    abi::SafeFactory::events::SafeDeployed, pb::masterfile::safe::v1::safe_factory_event,
    utils::pretty_hex,
};

pub fn extract_safe_factory_event(log: &LogView) -> Option<safe_factory_event::Event> {
    if let Some(event) = SafeDeployed::match_and_decode(log) {
        return Some(safe_factory_event::Event::SafeDeployed(
            safe_factory_event::SafeDeployed {
                safe: pretty_hex(&event.safe),
            },
        ));
    }

    None
}
