use crate::xdr::{ContractDataDurability, LedgerKey};

/// Default maximum entry TTL (roughly 1 year of ledgers at 5s/ledger)
/// used as a fallback when `max_entry_ttl` is not configured (0).
pub const DEFAULT_MAX_ENTRY_TTL: u32 = 6_312_000;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LedgerInfo {
    pub protocol_version: u32,
    pub sequence_number: u32,
    pub timestamp: u64,
    pub network_id: [u8; 32],
    pub base_reserve: u32,
    pub min_temp_entry_ttl: u32,
    pub min_persistent_entry_ttl: u32,
    pub max_entry_ttl: u32,
}

impl LedgerInfo {
    pub const DEFAULT_MAX_ENTRY_TTL: u32 = DEFAULT_MAX_ENTRY_TTL;

    pub fn min_live_until_ledger_checked(&self, durability: ContractDataDurability) -> Option<u32> {
        let min_live_until = match durability {
            ContractDataDurability::Temporary => self.min_temp_entry_ttl,
            ContractDataDurability::Persistent => self.min_persistent_entry_ttl,
        };
        self.sequence_number
            .checked_add(min_live_until.saturating_sub(1))
    }

    pub fn max_live_until_ledger_checked(&self) -> Option<u32> {
        let max_entry_ttl = if self.max_entry_ttl == 0 {
            DEFAULT_MAX_ENTRY_TTL
        } else {
            self.max_entry_ttl
        };
        self.sequence_number
            // Entry can live for at most max_entry_live_until ledgers from
            // now, counting the current one.
            .checked_add(max_entry_ttl.saturating_sub(1))
    }
}

pub fn get_key_durability(key: &LedgerKey) -> Option<ContractDataDurability> {
    match &key {
        LedgerKey::ContractData(d) => Some(d.durability),
        LedgerKey::ContractCode(_) => Some(ContractDataDurability::Persistent),
        _ => None,
    }
}
