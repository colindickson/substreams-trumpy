pub mod abi;
mod utils;
mod pb;
mod db_out;

use std::fmt::format;
use substreams::store::StoreNew;
use substreams::scalar::{BigDecimal, BigInt};
use substreams::errors::Error;
use substreams::{log, Hex, store};
use substreams_ethereum::Function;
use substreams_ethereum::pb::eth::v2::Block;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams::store::{DeltaArray, DeltaBigDecimal, DeltaBigInt, DeltaProto, StoreAdd, StoreAddBigDecimal, StoreAddBigInt, StoreAppend, StoreGetBigDecimal, StoreGetBigInt, StoreGetProto, StoreGetRaw, StoreSet, StoreSetBigDecimal, StoreSetBigInt, StoreSetProto};
use substreams_database_change::pb::database::table_change::Operation;
use crate::db_out::{mints_to_database_changes, push_create_mint, transfers_to_database_changes};
use crate::pb::trumpy::{Mint, Mints, Transfer, Transfers};
use crate::utils::{FACTORY_ADDRESS};


#[substreams::handlers::map]
pub fn map_transfers(block: Block) -> Result<pb::trumpy::Transfers, Error> {
    use abi::factory::events::Transfer;

    Ok(pb::trumpy::Transfers {
        transfers: block.events::<Transfer>(&[&FACTORY_ADDRESS])
            .filter_map(|(event, log)| {
                Some(pb::trumpy::Transfer {
                    from: Hex(event.from).to_string(),
                    to: Hex(event.to).to_string(),
                    token_id:event.token_id.to_string(),
                })

            }).collect()
    })
}

#[substreams::handlers::store]
pub fn store_transfers(transfers: Transfers, store: StoreSetProto<Transfer>) {
    for transfer in transfers.transfers {
        store.set(0, format!("transfer::from:{}:to:{}:token_id:{}", transfer.from, transfer.to, transfer.token_id), &transfer);
    }
}

#[substreams::handlers::map]
pub fn map_mints(block: Block) -> Result<pb::trumpy::Mints, Error> {
    use abi::factory::functions::MintSeveral;

    let mut mints = vec![];
    for trx in block.transaction_traces {
        if trx.status != 1 {
            continue;
        }

        for call in trx.calls {
            if call.address != FACTORY_ADDRESS {
                continue;
            }
            if MintSeveral::match_call(&call) {
                match MintSeveral::decode(&call) {
                    Ok(mint) => {
                        mints.push(pb::trumpy::Mint {
                            minter: Hex(mint.minter).to_string(),
                            number_of_tokens: mint.number_of_tokens.to_u64(),
                        });
                    }
                    Err(_) => {}
                }
            }
        }
    }

    Ok(Mints {
        mints,
    })
}

#[substreams::handlers::store]
pub fn store_mints(mints: Mints, store: StoreSetProto<Mint>) {
    for mint in mints.mints {
        store.set(0, format!("mint::minter:{}:tokens:{}", mint.minter, mint.number_of_tokens.to_string()), &mint);
    }
}

#[substreams::handlers::map]
pub fn db_out(mint_deltas: store::Deltas<DeltaProto<Mint>>, transfers_deltas: store::Deltas<DeltaProto<Transfer>>) -> Result<DatabaseChanges, Error> {
    let mut database_changes: DatabaseChanges = Default::default();
    mints_to_database_changes(&mut database_changes, mint_deltas);
    transfers_to_database_changes(&mut database_changes, transfers_deltas);
    Ok(database_changes)
}
