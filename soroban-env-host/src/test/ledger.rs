use soroban_env_common::{xdr::ScBytes, Env};

use crate::{
    budget::Budget,
    ledger_info::{LedgerInfo, DEFAULT_MAX_ENTRY_TTL},
    storage::{Footprint, Storage, StorageMap},
    Host, HostError,
};

#[test]
fn ledger_network_id() -> Result<(), HostError> {
    let budget = Budget::default();
    let storage =
        Storage::with_enforcing_footprint_and_map(Footprint::default(), StorageMap::new());

    let host = Host::with_storage_and_budget(storage, budget);
    host.set_test_ledger_info_with_current_test_protocol();
    host.with_mut_ledger_info(|li| {
        li.network_id = [7; 32];
    })?;
    let obj = host.get_ledger_network_id()?;
    let np = host.visit_obj(obj, |np: &ScBytes| Ok(np.to_vec()))?;
    assert_eq!(np, vec![7; 32],);
    Ok(())
}

#[test]
fn test_default_max_entry_ttl_fallback() -> Result<(), HostError> {
    // 1. Direct LedgerInfo check with max_entry_ttl == 0
    let li = LedgerInfo {
        sequence_number: 705663,
        max_entry_ttl: 0,
        ..Default::default()
    };
    assert_eq!(
        li.max_live_until_ledger_checked(),
        Some(705663 + DEFAULT_MAX_ENTRY_TTL - 1)
    );

    // 2. Direct LedgerInfo check with custom non-zero max_entry_ttl
    let li_custom = LedgerInfo {
        sequence_number: 705663,
        max_entry_ttl: 5000,
        ..Default::default()
    };
    assert_eq!(
        li_custom.max_live_until_ledger_checked(),
        Some(705663 + 5000 - 1)
    );

    // 3. Host::set_ledger_info with max_entry_ttl == 0 (such as from snapshot without archival config)
    let host = Host::test_host();
    let info = LedgerInfo {
        protocol_version: Host::current_test_protocol(),
        sequence_number: 705663,
        max_entry_ttl: 0,
        ..Default::default()
    };
    host.set_ledger_info(info)?;
    assert_eq!(
        host.with_ledger_info(|li| Ok(li.max_entry_ttl))?,
        DEFAULT_MAX_ENTRY_TTL
    );
    assert_eq!(
        host.max_live_until_ledger()?,
        705663 + DEFAULT_MAX_ENTRY_TTL - 1
    );

    // 4. Host::set_ledger_info preserves explicit non-zero max_entry_ttl
    let info_custom = LedgerInfo {
        protocol_version: Host::current_test_protocol(),
        sequence_number: 705663,
        max_entry_ttl: 12345,
        ..Default::default()
    };
    host.set_ledger_info(info_custom)?;
    assert_eq!(host.with_ledger_info(|li| Ok(li.max_entry_ttl))?, 12345);
    assert_eq!(host.max_live_until_ledger()?, 705663 + 12345 - 1);

    Ok(())
}
