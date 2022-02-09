use anchor_lang::context::CpiContext;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::Instruction;
use anchor_lang::solana_program::program::{invoke, invoke_signed};
use larix_lending::id as larix_lending_id;
use larix_lending::instruction::LendingInstruction;
use crate::accounts::{InitLendingMarket, RefreshReserve};

pub fn init_lending_market<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, InitLendingMarket<'info>>,
    quote_currency: [u8; 32],
) -> ProgramResult {
    let ix = Instruction {
        program_id: larix_lending_id(),
        accounts: vec![
            AccountMeta::new(ctx.accounts.authority.key(), true),
            AccountMeta::new(ctx.accounts.lending_market.key(), false),
            AccountMeta::new(ctx.accounts.rent.key(), false),
            AccountMeta::new(ctx.accounts.token_program.key(), false),
            AccountMeta::new(ctx.accounts.pyth_orcale_program.key(), false),
            AccountMeta::new(ctx.accounts.larix_orcale_program.key(), false),
            AccountMeta::new(ctx.accounts.larix_orcale_id.key(), false),
        ],
        data: LendingInstruction::InitLendingMarket {
            owner: ctx.accounts.authority.key(),
            quote_currency,
        }
            .pack(),
    };

    invoke_signed(
        &ix,
        &[
            ctx.accounts.authority,
            ctx.accounts.lending_market,
            ctx.accounts.rent,
            ctx.accounts.token_program,
            ctx.accounts.pyth_orcale_program,
            ctx.accounts.larix_orcale_program,
            ctx.accounts.larix_orcale_id,
            ctx.program,
        ],
        ctx.signer_seeds,
    )
}

pub fn refresh_reserve<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, RefreshReserve<'info>>
) -> ProgramResult{
    let ix = larix_lending::instruction::refresh_reserves(
        larix_lending_id(),
        vec![ctx.accounts.reserve.key()],
        vec![ctx.accounts.oracle.key()],
    );
    invoke(&ix, &ctx.to_account_infos())
}
