use base64::{engine::general_purpose, Engine as _};
use substreams_ethereum::{block_view::LogView, Event};

use crate::{
    abi::MasterfileDrop::events::{
        ApprovalForAll, BundlePurchased, BundleSet, DetailsSet, EditionProbabilitySet,
        EditionPurchased, EditionRedeemed, EditionSet, MetaTransactionExecuted,
        MysteryBoxPurchased, MysteryBoxRevealRequested, MysteryBoxRevealed, MysteryBoxSet,
        OwnershipTransferred, TransferBatch, TransferSingle,
    },
    pb::masterfile::drop::v1::drop_event,
    utils::pretty_hex,
};

pub fn extract_drop_event(log: &LogView) -> Option<drop_event::Event> {
    ////////////////////////////////////////////////////////////////////////////
    //                             	    Drop                                  //
    ////////////////////////////////////////////////////////////////////////////

    if let Some(event) = BundlePurchased::match_and_decode(log) {
        return Some(drop_event::Event::BundlePurchased(
            drop_event::BundlePurchased {
                funder: pretty_hex(&event.funder),
                recipient: pretty_hex(&event.recipient),
                quantity: event.quantity.to_string(),
                total_price: event.total_price.to_string(),
            },
        ));
    }
    if let Some(event) = BundleSet::match_and_decode(log) {
        return Some(drop_event::Event::BundleSet(drop_event::BundleSet {
            quantity: event.quantity.to_string(),
            price: event.price.to_string(),
            token_ids: event
                .token_ids
                .iter()
                .map(|token_id| token_id.to_u64())
                .collect(),
        }));
    }
    if let Some(event) = DetailsSet::match_and_decode(log) {
        return Some(drop_event::Event::DetailsSet(drop_event::DetailsSet {
            usdc: pretty_hex(&event.usdc),
            revenue_recipient: pretty_hex(&event.revenue_recipient),
            randomness_provider: pretty_hex(&event.randomness_provider),
            start_date: event.start_date.to_u64(),
            end_date: event.end_date.to_u64(),
            num_editions: event.num_editions.to_u64(),
        }));
    }
    if let Some(event) = EditionProbabilitySet::match_and_decode(log) {
        return Some(drop_event::Event::EditionProbabilitySet(
            drop_event::EditionProbabilitySet {
                token_id: event.token_id.to_u64(),
                probability: event.probability.to_u64(),
            },
        ));
    }
    if let Some(event) = EditionPurchased::match_and_decode(log) {
        return Some(drop_event::Event::EditionPurchased(
            drop_event::EditionPurchased {
                funder: pretty_hex(&event.funder),
                recipient: pretty_hex(&event.recipient),
                token_id: event.token_id.to_u64(),
                quantity: event.quantity.to_string(),
                total_price: event.total_price.to_string(),
            },
        ));
    }
    if let Some(event) = EditionRedeemed::match_and_decode(log) {
        return Some(drop_event::Event::EditionRedeemed(
            drop_event::EditionRedeemed {
                request_id: event.request_id.to_string(),
                redeemer: pretty_hex(&event.redeemer),
                token_id: event.token_id.to_u64(),
            },
        ));
    }
    if let Some(event) = EditionSet::match_and_decode(log) {
        return Some(drop_event::Event::EditionSet(drop_event::EditionSet {
            token_id: event.token_id.to_u64(),
            price: event.edition.0.to_string(),
            quantity: event.edition.1.to_string(),
            arweave_hash: general_purpose::STANDARD.encode(&event.edition.2), // Encode as base64
            arweave_cid: pretty_hex(&event.edition.2),
        }));
    }
    if let Some(event) = MysteryBoxPurchased::match_and_decode(log) {
        return Some(drop_event::Event::MysteryBoxPurchased(
            drop_event::MysteryBoxPurchased {
                funder: pretty_hex(&event.funder),
                recipient: pretty_hex(&event.recipient),
                quantity: event.quantity.to_string(),
                total_price: event.total_price.to_string(),
            },
        ));
    }
    if let Some(event) = MysteryBoxRevealRequested::match_and_decode(log) {
        return Some(drop_event::Event::MysteryBoxRevealRequested(
            drop_event::MysteryBoxRevealRequested {
                request_id: event.request_id.to_string(),
                recipient: pretty_hex(&event.recipient),
                nonce: event.nonce.to_string(),
            },
        ));
    }
    if let Some(event) = MysteryBoxRevealed::match_and_decode(log) {
        return Some(drop_event::Event::MysteryBoxRevealed(
            drop_event::MysteryBoxRevealed {
                request_id: event.request_id.to_string(),
                recipient: pretty_hex(&event.recipient),
                randomness: event.randomness.to_string(),
            },
        ));
    }
    if let Some(event) = MysteryBoxSet::match_and_decode(log) {
        return Some(drop_event::Event::MysteryBoxSet(
            drop_event::MysteryBoxSet {
                quantity: event.quantity.to_string(),
                price: event.price.to_string(),
                redemptions: event.redemptions.to_u64(),
            },
        ));
    }

    ////////////////////////////////////////////////////////////////////////////
    //                             	  ERC-1155                                //
    ////////////////////////////////////////////////////////////////////////////

    if let Some(event) = ApprovalForAll::match_and_decode(log) {
        return Some(drop_event::Event::ApprovalForAll(
            drop_event::ApprovalForAll {
                account: pretty_hex(&event.account),
                operator: pretty_hex(&event.operator),
                approved: event.approved,
            },
        ));
    }

    if let Some(event) = TransferBatch::match_and_decode(log) {
        return Some(drop_event::Event::TransferBatch(
            drop_event::TransferBatch {
                operator: pretty_hex(&event.operator),
                from: pretty_hex(&event.from),
                to: pretty_hex(&event.to),
                ids: event.ids.iter().map(|id| id.to_u64()).collect(),
                values: event.values.iter().map(|value| value.to_string()).collect(),
            },
        ));
    }

    if let Some(event) = TransferSingle::match_and_decode(log) {
        return Some(drop_event::Event::TransferSingle(
            drop_event::TransferSingle {
                operator: pretty_hex(&event.operator),
                from: pretty_hex(&event.from),
                to: pretty_hex(&event.to),
                id: event.id.to_u64(),
                value: event.value.to_string(),
            },
        ));
    }

    ////////////////////////////////////////////////////////////////////////////
    //                        Native Meta Transaction                         //
    ////////////////////////////////////////////////////////////////////////////

    if let Some(event) = MetaTransactionExecuted::match_and_decode(log) {
        return Some(drop_event::Event::MetaTransactionExecuted(
            drop_event::MetaTransactionExecuted {
                user_address: pretty_hex(&event.user_address),
                relayer_address: pretty_hex(&event.relayer_address),
                function_signature: pretty_hex(&event.function_signature),
            },
        ));
    }

    ////////////////////////////////////////////////////////////////////////////
    //                                Ownable                                 //
    ////////////////////////////////////////////////////////////////////////////

    if let Some(event) = OwnershipTransferred::match_and_decode(log) {
        return Some(drop_event::Event::OwnershipTransferred(
            drop_event::OwnershipTransferred {
                previous_owner: pretty_hex(&event.previous_owner),
                new_owner: pretty_hex(&event.new_owner),
            },
        ));
    }

    None
}
