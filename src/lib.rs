pub mod abi;
mod utils;
mod pb;

use substreams::errors::Error;
use substreams::{log, Hex};
use substreams_ethereum::Function;
use substreams_ethereum::pb::eth::v2::Block;
use crate::pb::trumpy::{Mints, Transfer, Transfers};
use crate::utils::FACTORY_ADDRESS;



#[substreams::handlers::map]
pub fn map_transfer_ownership(block: Block) -> Result<pb::trumpy::Transfers, Error> {
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

#[substreams::handlers::map]
pub fn map_mint_several(block: Block) -> Result<pb::trumpy::Mints, Error> {
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
                            number_of_tokens: mint.number_of_tokens.to_u64() as i64,
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