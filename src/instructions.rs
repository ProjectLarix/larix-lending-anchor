use crate::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::Instruction;
use anchor_lang::solana_program::program::{invoke, invoke_signed};
use larix_lending::instruction::LendingInstruction;
use larix_lending::state::obligation::OBLIGATION_LEN;

pub fn init_lending_market<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, InitLendingMarket<'info>>,
    quote_currency: [u8; 32],
) -> Result<()> {
    let ix = Instruction {
        program_id: ID,
        accounts: ctx.to_account_metas(Option::from(false)),
        data: LendingInstruction::InitLendingMarket {
            owner: ctx.accounts.authority.key(),
            quote_currency,
        }
        .pack(),
    };
    invoke_signed(&ix, &ctx.to_account_infos(), ctx.signer_seeds).map_err(Into::into)
}

///
/// ctx.remaining_accounts: Other reserve and oracles
///
///
pub fn refresh_reserves<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, RefreshReserves<'info>>,
) -> Result<()> {
    let mut reserves = vec![ctx.accounts.reserve.key()];
    let mut oracles = vec![ctx.accounts.oracle.key()];
    for i in 0..ctx.remaining_accounts.len() / 2 {
        reserves.push(ctx.remaining_accounts[i * 2].key());
        oracles.push(ctx.remaining_accounts[i * 2 + 1].key());
    }
    let ix = larix_lending::instruction::refresh_reserves(ID, reserves, oracles);
    invoke(&ix, &ctx.to_account_infos()).map_err(Into::into)
}

///
///
/// lending_market_authority: Pubkey::find_program_address(
///         &[&lending_market_pubkey.to_bytes()[..PUBKEY_BYTES]],
///         &program_id,
///     ).0;
///
pub fn deposit_reserve_liquidity<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, DepositReserveLiquidity<'info>>,
    liquidity_amount: u64,
) -> Result<()> {
    let ix = larix_lending::instruction::deposit_reserve_liquidity(
        ID,
        liquidity_amount,
        ctx.accounts.source_liquidity.key(),
        ctx.accounts.destination_collateral.key(),
        ctx.accounts.reserve.key(),
        ctx.accounts.reserve_collateral_mint.key(),
        ctx.accounts.reserve_liquidity_supply.key(),
        ctx.accounts.lending_market.key(),
        ctx.accounts.lending_market_authority.key(),
        ctx.accounts.user_transfer_authority.key(),
    );
    invoke_signed(&ix, &ctx.to_account_infos(), ctx.signer_seeds).map_err(Into::into)
}

pub fn redeem_reserve_collateral<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, RedeemReserveCollateral<'info>>,
    liquidity_amount: u64,
) -> Result<()> {
    let ix = larix_lending::instruction::redeem_reserve_collateral(
        ID,
        liquidity_amount,
        ctx.accounts.source_collateral.key(),
        ctx.accounts.destination_liquidity_pubkey.key(),
        ctx.accounts.reserve.key(),
        ctx.accounts.reserve_collateral_mint.key(),
        ctx.accounts.reserve_liquidity_supply.key(),
        ctx.accounts.lending_market.key(),
        ctx.accounts.lending_market_authority.key(),
        ctx.accounts.user_transfer_authority.key(),
    );
    invoke_signed(&ix, &ctx.to_account_infos(), ctx.signer_seeds).map_err(Into::into)
}

pub fn init_obligation<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, InitObligation<'info>>,
) -> Result<()> {
    let ix = larix_lending::instruction::init_obligation(
        ID,
        ctx.accounts.obligation.key(),
        ctx.accounts.lending_market.key(),
        ctx.accounts.obligation_owner.key(),
    );
    invoke_signed(&ix, &ctx.to_account_infos(), ctx.signer_seeds).map_err(Into::into)
}

pub fn init_obligation2<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, InitObligation2<'info>>,
) -> Result<()> {
    let min_balance = Rent::get()?.minimum_balance(OBLIGATION_LEN);
    let ix = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.payer.key(),
        &ctx.accounts.init_obligation.obligation.key(),
        min_balance,
    );
    invoke_signed(
        &ix,
        &[
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.init_obligation.obligation.to_account_info(),
        ],
        ctx.signer_seeds,
    )?;

    let ix = anchor_lang::solana_program::system_instruction::allocate(
        &ctx.accounts.init_obligation.obligation.key(),
        OBLIGATION_LEN as u64,
    );
    invoke_signed(
        &ix,
        &[
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.init_obligation.obligation.to_account_info(),
        ],
        ctx.signer_seeds,
    )?;

    init_obligation(CpiContext::new_with_signer(
        ctx.program,
        ctx.accounts.init_obligation,
        ctx.signer_seeds,
    ))
}

///
/// @param ctx.remaining_accounts: deposit reserves and borrow reserves that should be refreshed
///
pub fn refresh_obligation<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, RefreshObligation<'info>>,
) -> Result<()> {
    let mut reserve_pubkeys = vec![];
    for i in 0..ctx.remaining_accounts.len() {
        reserve_pubkeys.push(*ctx.remaining_accounts[i].key);
    }
    let ix = larix_lending::instruction::refresh_obligation(
        ID,
        ctx.accounts.obligation.key(),
        reserve_pubkeys,
    );
    invoke(&ix, &ctx.to_account_infos()).map_err(Into::into)
}

///
/// @param ctx.remaining_accounts: deposit reserves and borrow reserves
///
pub fn deposit_obligation_collateral<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, DepositObligationCollateral<'info>>,
    collateral_amount: u64,
) -> Result<()> {
    let mut reserve_pubkeys = vec![];
    for i in 0..ctx.remaining_accounts.len() {
        reserve_pubkeys.push(*ctx.remaining_accounts[i].key);
    }
    let ix = larix_lending::instruction::deposit_obligation_collateral(
        ID,
        collateral_amount,
        ctx.accounts.source_collateral.key(),
        ctx.accounts.destination_collateral.key(),
        ctx.accounts.deposit_reserve.key(),
        ctx.accounts.obligation.key(),
        ctx.accounts.lending_market.key(),
        ctx.accounts.lending_market_authority.key(),
        ctx.accounts.obligation_owner.key(),
        ctx.accounts.user_transfer_authority.key(),
        reserve_pubkeys,
    );
    invoke_signed(&ix, &ctx.to_account_infos(), ctx.signer_seeds).map_err(Into::into)
}

pub fn borrow_obligation_liquidity<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, BorrowObligationLiquidity<'info>>,
    liquidity_amount: u64,
) -> Result<()> {
    let ix = larix_lending::instruction::borrow_obligation_liquidity(
        ID,
        liquidity_amount,
        ctx.accounts.source_liquidity.key(),
        ctx.accounts.destination_liquidity.key(),
        ctx.accounts.borrow_reserve.key(),
        ctx.accounts.borrow_reserve_liquidity_fee_receiver.key(),
        ctx.accounts.obligation.key(),
        ctx.accounts.lending_market.key(),
        ctx.accounts.lending_market_authority.key(),
        ctx.accounts.obligation_owner.key(),
        ctx.accounts.larix_oracle_program.key(),
        ctx.accounts.mine_mint.key(),
    );
    invoke_signed(&ix, &ctx.to_account_infos(), ctx.signer_seeds).map_err(Into::into)
}

pub fn repay_obligation_liquidity<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, RepayObligationLiquidity<'info>>,
    liquidity_amount: u64,
) -> Result<()> {
    let ix = larix_lending::instruction::repay_obligation_liquidity(
        ID,
        liquidity_amount,
        ctx.accounts.source_liquidity.key(),
        ctx.accounts.destination_liquidity.key(),
        ctx.accounts.repay_reserve.key(),
        ctx.accounts.obligation.key(),
        ctx.accounts.lending_market.key(),
        ctx.accounts.user_transfer_authority.key(),
    );
    invoke_signed(&ix, &ctx.to_account_infos(), ctx.signer_seeds).map_err(Into::into)
}

pub fn claim_obligation_mine<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, ClaimObligationMine<'info>>,
) -> Result<()> {
    let ix = larix_lending::instruction::claim_obligation_mine(
        ID,
        ctx.accounts.obligation.key(),
        ctx.accounts.mine_supply.key(),
        ctx.accounts.destination_account.key(),
        ctx.accounts.obligation_owner.key(),
        ctx.accounts.lending_market.key(),
        ctx.accounts.lending_market_authority.key(),
    );
    invoke_signed(&ix, &ctx.to_account_infos(), ctx.signer_seeds).map_err(Into::into)
}

pub fn liquidate_obligation<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, LiquidateObligation<'info>>,
    liquidity_amount: u64,
) -> Result<()> {
    liquidate_obligation_2(
        CpiContext::new_with_signer(
            ctx.program,
            ctx.accounts.liquidity_obligation_2,
            ctx.signer_seeds,
        ),
        liquidity_amount,
    )
}

pub fn liquidate_obligation_2<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, LiquidateObligation2<'info>>,
    liquidity_amount: u64,
) -> Result<()> {
    let ix = Instruction {
        program_id: ID,
        accounts: ctx.to_account_metas(Option::from(false)),
        data: LendingInstruction::LiquidateObligation2 { liquidity_amount }.pack(),
    };
    invoke_signed(&ix, &ctx.to_account_infos(), ctx.signer_seeds).map_err(Into::into)
}
