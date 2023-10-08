use crate::pb::masterfile::common::v1::TransactionMetadata;
use substreams::Hex;
use substreams_ethereum::{block_view::LogView, pb::eth::v2 as eth};
use tiny_keccak::{Hasher, Keccak};

pub fn pretty_hex<T: std::convert::AsRef<[u8]>>(addr: &T) -> String {
    format!("0x{}", &Hex(addr).to_string())
}

pub fn extract_metadata(log: &LogView, block: &eth::Block) -> Option<TransactionMetadata> {
    Some(TransactionMetadata {
        tx_hash: pretty_hex(&log.receipt.transaction.hash),
        block_number: block.number,
        block_timestamp: block.timestamp_seconds(),
        to: pretty_hex(&log.receipt.transaction.to),
        from: pretty_hex(&log.receipt.transaction.from),
        log_index: log.log.index,
        block_index: log.log.block_index,
        address: pretty_hex(&log.log.address),
    })
}

pub fn keccak256<S>(bytes: S) -> [u8; 32]
where
    S: AsRef<[u8]>,
{
    let mut output = [0u8; 32];
    let mut hasher = Keccak::v256();
    hasher.update(bytes.as_ref());
    hasher.finalize(&mut output);
    output
}
