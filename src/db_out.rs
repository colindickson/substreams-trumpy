use substreams::{Hex, store};
use substreams::store::DeltaProto;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_database_change::pb::database::table_change::Operation;
use crate::pb::trumpy::{Mint, Transfer};

pub fn mints_to_database_changes(
    changes: &mut DatabaseChanges,
    deltas: store::Deltas<DeltaProto<Mint>>,
) {
    use substreams::pb::substreams::store_delta::Operation;

    for delta in deltas.deltas {
        match delta.operation {
            Operation::Create => push_create_mint(changes, &delta.key, delta.ordinal, delta.new_value),
            Operation::Update => push_update_mint(changes, &delta.key, delta.ordinal, delta.old_value, delta.new_value),
            Operation::Delete => todo!(),
            _ => panic!("unsupported opeation {:?}", delta.operation),
        }
    }
}

pub fn transfers_to_database_changes(
    changes: &mut DatabaseChanges,
    deltas: store::Deltas<DeltaProto<Transfer>>,
) {
    use substreams::pb::substreams::store_delta::Operation;

    for delta in deltas.deltas {
        match delta.operation {
            Operation::Create => push_create_transfer(changes, &delta.key, delta.ordinal, delta.new_value),
            Operation::Update => push_update_transfer(changes, &delta.key, delta.ordinal, delta.old_value, delta.new_value),
            Operation::Delete => todo!(),
            _ => panic!("unsupported opeation {:?}", delta.operation),
        }
    }
}

pub fn push_create_mint(
    changes: &mut DatabaseChanges,
    key: &str,
    ordinal: u64,
    value: Mint,
) {
    changes
        .push_change("mint", key, ordinal, Operation::Create)
        .change("minter", (None, Hex(value.minter)))
        .change("number_of_tokens", (None, value.number_of_tokens));
}

pub fn push_update_mint(
    changes: &mut DatabaseChanges,
    key: &str,
    ordinal: u64,
    old_value: Mint,
    new_value: Mint,
) {
    changes
        .push_change("mint", key, ordinal, Operation::Update)
        .change("minter", (Hex(old_value.minter), Hex(new_value.minter)))
        .change("number_of_tokens", (old_value.number_of_tokens, new_value.number_of_tokens));
}

pub fn push_create_transfer(
    changes: &mut DatabaseChanges,
    key: &str,
    ordinal: u64,
    value: Transfer,
) {
    changes
        .push_change("transfer", key, ordinal, Operation::Create)
        .change("from", (None, Hex(value.from)))
        .change("to", (None, Hex(value.to)))
        .change("token_id", (None, value.token_id));
}

pub fn push_update_transfer(
    changes: &mut DatabaseChanges,
    key: &str,
    ordinal: u64,
    old_value: Transfer,
    new_value: Transfer,
) {
    changes
        .push_change("transfer", key, ordinal, Operation::Update)
        .change("from", (Hex(old_value.from), Hex(new_value.from)))
        .change("to", (Hex(old_value.to), Hex(new_value.to)))
        .change("token_id", (old_value.token_id, new_value.token_id));
}