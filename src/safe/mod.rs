use ethabi::{Address, Token, Uint};
use hex_literal::hex;
use substreams_ethereum::block_view::LogView;
use substreams_ethereum::Event;

use crate::{
    abi::SafeL2::events::{
        AddedOwner, ApproveHash, ChangedFallbackHandler, ChangedGuard, ChangedThreshold,
        DisabledModule, EnabledModule, ExecutionFailure, ExecutionSuccess, RemovedOwner,
        SafeModuleTransaction, SafeMultiSigTransaction, SafeReceived, SafeSetup,
    },
    pb::masterfile::{
        events::v1::{masterfile_event, MasterfileEvent},
        safe::v1::safe_event::{self, safe_multi_sig_transaction, SafeOwner},
    },
    utils::{keccak256, pretty_hex},
};

pub fn extract_safe_event(log: &LogView, chain_id: &u64) -> Option<safe_event::Event> {
    if let Some(event) = AddedOwner::match_and_decode(log) {
        return Some(safe_event::Event::AddedOwner(safe_event::AddedOwner {
            owner: pretty_hex(&event.owner),
        }));
    }
    if let Some(event) = RemovedOwner::match_and_decode(log) {
        return Some(safe_event::Event::RemovedOwner(safe_event::RemovedOwner {
            owner: pretty_hex(&event.owner),
        }));
    }
    if let Some(event) = ApproveHash::match_and_decode(log) {
        return Some(safe_event::Event::ApproveHash(safe_event::ApproveHash {
            owner: pretty_hex(&event.owner),
            hash: pretty_hex(&event.approved_hash),
        }));
    }
    if let Some(event) = ChangedFallbackHandler::match_and_decode(log) {
        return Some(safe_event::Event::ChangedFallbackHandler(
            safe_event::ChangedFallbackHandler {
                handler: pretty_hex(&event.handler),
            },
        ));
    }
    if let Some(event) = ChangedGuard::match_and_decode(log) {
        return Some(safe_event::Event::ChangedGuard(safe_event::ChangedGuard {
            guard: pretty_hex(&event.guard),
        }));
    }
    if let Some(event) = ChangedThreshold::match_and_decode(log) {
        return Some(safe_event::Event::ChangedThreshold(
            safe_event::ChangedThreshold {
                threshold: event.threshold.to_u64(),
            },
        ));
    }
    if let Some(event) = DisabledModule::match_and_decode(log) {
        return Some(safe_event::Event::DisabledModule(
            safe_event::DisabledModule {
                module: pretty_hex(&event.module),
            },
        ));
    }
    if let Some(event) = EnabledModule::match_and_decode(log) {
        return Some(safe_event::Event::EnabledModule(
            safe_event::EnabledModule {
                module: pretty_hex(&event.module),
            },
        ));
    }
    if let Some(event) = SafeModuleTransaction::match_and_decode(log) {
        return Some(safe_event::Event::SafeModuleTransaction(
            safe_event::SafeModuleTransaction {
                module: pretty_hex(&event.module),
                to: pretty_hex(&event.to),
                value: event.value.to_string(),
                data: pretty_hex(&event.data),
                operation: event.operation.to_u64(),
                result: None,
            },
        ));
    }
    if let Some(event) = SafeMultiSigTransaction::match_and_decode(log) {
        let mut tx_event = safe_event::SafeMultiSigTransaction {
            to: pretty_hex(&event.to),
            value: event.value.to_string(),
            data: pretty_hex(&event.data),
            operation: event.operation.to_u64(),
            safe_tx_gas: event.safe_tx_gas.to_string(),
            base_gas: event.base_gas.to_string(),
            gas_price: event.gas_price.to_string(),
            gas_token: pretty_hex(&event.gas_token),
            refund_receiver: pretty_hex(&event.refund_receiver),
            signatures: pretty_hex(&event.signatures),
            additional_info: pretty_hex(&event.additional_info),
            safe_tx_hash: "".to_string(),
            nonce: 0,
            result: None,
        };

        let (safe_tx_hash, nonce) = calculate_safe_tx_hash(event, &log.address(), chain_id);

        tx_event.safe_tx_hash = pretty_hex(&safe_tx_hash);
        tx_event.nonce = nonce;

        return Some(safe_event::Event::SafeMultisigTransaction(tx_event));
    }
    if let Some(event) = SafeReceived::match_and_decode(log) {
        return Some(safe_event::Event::SafeReceived(safe_event::SafeReceived {
            sender: pretty_hex(&event.sender),
            value: event.value.to_string(),
        }));
    }
    if let Some(event) = SafeSetup::match_and_decode(log) {
        return Some(safe_event::Event::SafeSetup(safe_event::SafeSetup {
            initiator: pretty_hex(&event.initiator),
            threshold: event.threshold.to_u64(),
            initializer: pretty_hex(&event.initializer),
            fallback_handler: pretty_hex(&event.fallback_handler),
            owners: event
                .owners
                .iter()
                .map(|o| SafeOwner {
                    address: pretty_hex(o),
                })
                .collect(),
        }));
    }
    None
}

pub fn hydrate_tx_results(log: &LogView, events: &mut Vec<MasterfileEvent>) {
    if let Some(execution_success) = ExecutionSuccess::match_and_decode(log) {
        events.iter_mut().for_each(|event| {
            if let Some(masterfile_event::Event::Safe(ref mut event)) = &mut event.event {
                if let Some(safe_event::Event::SafeMultisigTransaction(ref mut tx_event)) =
                    &mut event.event
                {
                    if tx_event.safe_tx_hash == pretty_hex(&execution_success.tx_hash) {
                        tx_event.r#result =
                            Some(safe_multi_sig_transaction::Result::ExecutionSuccess({
                                safe_event::ExecutionSuccess {
                                    safe_tx_hash: pretty_hex(&execution_success.tx_hash),
                                    payment: execution_success.payment.to_string(),
                                }
                            }));
                        return;
                    }
                }
            }
        });
    }
    if let Some(execution_failure) = ExecutionFailure::match_and_decode(log) {
        events.iter_mut().for_each(|event| {
            if let Some(masterfile_event::Event::Safe(ref mut event)) = &mut event.event {
                if let Some(safe_event::Event::SafeMultisigTransaction(ref mut tx_event)) =
                    &mut event.event
                {
                    if tx_event.safe_tx_hash == pretty_hex(&execution_failure.tx_hash) {
                        tx_event.r#result =
                            Some(safe_multi_sig_transaction::Result::ExecutionFailure({
                                safe_event::ExecutionFailure {
                                    safe_tx_hash: pretty_hex(&execution_failure.tx_hash),
                                    payment: execution_failure.payment.to_string(),
                                }
                            }));
                        return;
                    }
                }
            }
        });
    }
    // TODO: ExecutionFromModuleSuccess/Failure
}

const SAFE_TX_TYPEHASH: [u8; 32] =
    hex!("bb8310d486368db6bd6f849402fdd73ad53d316b5a4b2644ad6efe0f941286d8");
pub const DOMAIN_SEPARATOR_TYPEHASH: [u8; 32] =
    hex!("47e79534a245952e8b16893a336b85a3d9ea9fa8c573f3d803afb92a79469218");

pub const ERC191_BYTE: &'static str = "19";
pub const ERC191_VERSION: &'static str = "01";

/// https://github.com/safe-global/safe-client-gateway/blob/0957d8d66cc40b1c4c6556b33735e613caf50bce/src/utils/transactions.rs
fn calculate_safe_tx_hash(
    event: SafeMultiSigTransaction,
    address: &[u8],
    chain_id: &u64,
) -> ([u8; 32], u64) {
    let safe_type_hash: Uint = Uint::from(SAFE_TX_TYPEHASH);
    let to: Address = Address::from_slice(&event.to);
    let value: Uint = Uint::from_big_endian(&event.value.to_signed_bytes_be());
    let data: Uint = Uint::from(keccak256(&event.data.to_vec()));
    let operation: Uint = Uint::from(event.operation.to_u64());
    let safe_tx_gas: Uint = Uint::from_big_endian(&event.safe_tx_gas.to_signed_bytes_be());
    let base_gas: Uint = Uint::from_big_endian(&event.base_gas.to_signed_bytes_be());
    let gas_price: Uint = Uint::from_big_endian(&event.gas_price.to_signed_bytes_be());
    let gas_token: Address = Address::from_slice(&event.gas_token);
    let refund_receiver: Address = Address::from_slice(&event.refund_receiver);
    let nonce: Uint = Uint::from_big_endian(&event.additional_info[0..32]);

    let hash = keccak256(ethabi::encode(&[
        Token::Uint(safe_type_hash),
        Token::Address(to),              // to
        Token::Uint(value),              // value
        Token::Uint(data),               // data
        Token::Uint(operation),          // operation
        Token::Uint(safe_tx_gas),        // safe_tx_gas
        Token::Uint(base_gas),           // base_gas
        Token::Uint(gas_price),          // gas_price
        Token::Address(gas_token),       // gas_token
        Token::Address(refund_receiver), // refund_receiver
        Token::Uint(nonce),              // nonce
    ]));

    let domain_hash = keccak256(ethabi::encode(&[
        Token::Uint(Uint::from(DOMAIN_SEPARATOR_TYPEHASH)),
        Token::Uint(Uint::from(chain_id.clone())),
        Token::Address(Address::from_slice(address)),
    ]));

    let mut encoded = ethabi::encode(&[
        ethabi::Token::Uint(Uint::from(domain_hash)),
        ethabi::Token::Uint(Uint::from(hash)),
    ]);
    let erc_191_byte = u8::from_str_radix(ERC191_BYTE, 16).unwrap();
    let erc_191_version = u8::from_str_radix(ERC191_VERSION, 16).unwrap();

    encoded.insert(0, erc_191_version);
    encoded.insert(0, erc_191_byte);
    (keccak256(encoded), nonce.as_u64())
}
