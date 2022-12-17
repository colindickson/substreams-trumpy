pub mod abi;
mod utils;
mod pb;

use substreams::errors::Error;
use substreams::{log, Hex};
use substreams_ethereum::pb::eth::v2::Block;
use crate::pb::trumpy::{Transfer, Transfers};
use crate::utils::FACTORY_ADDRESS;



#[substreams::handlers::map]
pub fn map_transfer_ownership(block: Block) -> Result<pb::trumpy::Transfers, Error> {
    use abi::factory::events::OwnershipTransferred;

    Ok(pb::trumpy::Transfers {
        transfers: block.events::<OwnershipTransferred>(&[&FACTORY_ADDRESS])
            .filter_map(|(event, log)| {
                Some(pb::trumpy::Transfer {
                    previous_owner: Hex(event.previous_owner).to_string(),
                    new_owner: Hex(event.new_owner).to_string(),
                })
            }).collect()
    })
}