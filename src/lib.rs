pub mod accounts;
pub mod instructions;
pub mod state;

pub use {accounts::*, instructions::*, state::*};

use anchor_lang::prelude::*;
use larix_lending::ID;

#[derive(Clone)]
pub struct Larix;

impl anchor_lang::AccountDeserialize for Larix {
    fn try_deserialize(buf: &mut &[u8]) -> Result<Self> {
        Larix::try_deserialize_unchecked(buf)
    }

    fn try_deserialize_unchecked(_buf: &mut &[u8]) -> Result<Self> {
        Ok(Larix)
    }
}

impl anchor_lang::Id for Larix {
    fn id() -> Pubkey {
        ID
    }
}
