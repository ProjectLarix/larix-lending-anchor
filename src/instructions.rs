use anchor_lang::context::CpiContext;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::Instruction;
use anchor_lang::solana_program::program::{invoke, invoke_signed};
use larix_lending::id as larix_lending_id;
use larix_lending::instruction::LendingInstruction;
use larix_lending::state::obligation::OBLIGATION_LEN;
use crate::accounts::{DepositReserveLiquidity, InitLendingMarket, InitObligation, InitObligation2, RedeemReserveCollateral, RefreshReserve};

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
pub fn deposit_reserve_liquidity<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, DepositReserveLiquidity<'info>>,
    liquidity_amount:u64,
) -> ProgramResult{
    let ix = larix_lending::instruction::deposit_reserve_liquidity(
        larix_lending_id(),
        liquidity_amount,
        ctx.accounts.source_liquidity_pubkey.key(),
        ctx.accounts.destination_collateral_pubkey.key(),
        ctx.accounts.reserve_pubkey.key(),
        ctx.accounts.reserve_collateral_mint_pubkey.key(),
        ctx.accounts.reserve_liquidity_supply_pubkey.key(),
        ctx.accounts.lending_market_pubkey.key(),
        ctx.accounts.user_transfer_authority_pubkey.key()
    );
    invoke_signed(&ix, &ctx.to_account_infos(),ctx.signer_seeds)
}
pub fn redeem_reserve_collateral<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, RedeemReserveCollateral<'info>>,
    liquidity_amount:u64,
) -> ProgramResult{
    let ix = larix_lending::instruction::redeem_reserve_collateral(
        larix_lending_id(),
        liquidity_amount,
        ctx.accounts.source_collateral_pubkey.key(),
        ctx.accounts.destination_liquidity_pubkey.key(),
        ctx.accounts.reserve_pubkey.key(),
        ctx.accounts.reserve_collateral_mint_pubkey.key(),
        ctx.accounts.reserve_liquidity_supply_pubkey.key(),
        ctx.accounts.lending_market_pubkey.key(),
        ctx.accounts.user_transfer_authority_pubkey.key()
    );
    invoke_signed(&ix, &ctx.to_account_infos(),ctx.signer_seeds)
}

pub fn init_obligation<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, InitObligation<'info>>,
) -> ProgramResult {
    let ix = larix_lending::instruction::init_obligation(
        larix_lending_id(),
        ctx.accounts.obligation_pubkey.key(),
        ctx.accounts.lending_market_pubkey.key(),
        ctx.accounts.obligation_owner_pubkey.key()
    );
    invoke_signed(
        &ix,
        &[
            ctx.accounts.obligation_pubkey.to_account_info(),
            ctx.accounts.lending_market_pubkey.to_account_info(),
            ctx.accounts.obligation_owner_pubkey.to_account_info(),
        ],
        ctx.signer_seeds
    )
}
pub fn init_obligation2<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, InitObligation2<'info>>,
) -> ProgramResult {
    let min_balance = Rent::get()?.minimum_balance(OBLIGATION_LEN);
    let ix = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.payer.key(),
        &ctx.accounts.obligation_pubkey.key(),
        min_balance
    );
    invoke_signed(
        &ix,
        &[
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.obligation_pubkey.to_account_info(),
                  ],
        ctx.signer_seeds)?;
    let ix = larix_lending::instruction::init_obligation(
        larix_lending_id(),
        ctx.accounts.obligation_pubkey.key(),
        ctx.accounts.lending_market_pubkey.key(),
        ctx.accounts.obligation_owner_pubkey.key()
    );
    invoke_signed(
        &ix,
        &[
            ctx.accounts.obligation_pubkey.to_account_info(),
            ctx.accounts.lending_market_pubkey.to_account_info(),
            ctx.accounts.obligation_owner_pubkey.to_account_info(),
        ],
        ctx.signer_seeds
    )
}