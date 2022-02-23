use anchor_lang::prelude::*;
use anchor_lang::Accounts;
#[derive(Accounts)]
pub struct InitLendingMarket<'info> {
    pub authority: Signer<'info>,
    #[account(mut)]
    pub lending_market: AccountInfo<'info>,
    pub rent: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub pyth_orcale_program: AccountInfo<'info>,
    pub larix_orcale_program: AccountInfo<'info>,
    pub larix_orcale_id: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct RefreshReserves<'info> {
    #[account(mut)]
    pub reserve:AccountInfo<'info>,
    pub oracle:AccountInfo<'info>,
}

/// lending_market_authority: Pubkey::find_program_address(
///         &[&lending_market_pubkey.to_bytes()[..PUBKEY_BYTES]],
///         &program_id,
///     ).0;
///
#[derive(Accounts)]
pub struct DepositReserveLiquidity<'info> {
    #[account(mut)]
    pub source_liquidity: AccountInfo<'info>,
    #[account(mut)]
    pub destination_collateral: AccountInfo<'info>,
    #[account(mut)]
    pub reserve: AccountInfo<'info>,
    #[account(mut)]
    pub reserve_collateral_mint: AccountInfo<'info>,
    #[account(mut)]
    pub reserve_liquidity_supply: AccountInfo<'info>,
    pub lending_market: AccountInfo<'info>,
    pub lending_market_authority:AccountInfo<'info>,
    pub user_transfer_authority: Signer<'info>,
    pub token_program:AccountInfo<'info>,

}
#[derive(Accounts)]
pub struct RedeemReserveCollateral<'info> {
    #[account(mut)]
    pub source_collateral: AccountInfo<'info>,
    #[account(mut)]
    pub reserve: AccountInfo<'info>,
    #[account(mut)]
    pub reserve_collateral_mint: AccountInfo<'info>,
    #[account(mut)]
    pub reserve_liquidity_supply: AccountInfo<'info>,
    pub lending_market: AccountInfo<'info>,
    pub lending_market_authority:AccountInfo<'info>,
    pub user_transfer_authority: Signer<'info>,
    pub token_program:AccountInfo<'info>,
    #[account(mut)]
    pub destination_liquidity_pubkey: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct InitObligation<'info>{
    #[account(mut)]
    pub obligation: AccountInfo<'info>,
    pub lending_market: AccountInfo<'info>,
    pub obligation_owner: Signer<'info>,
}
#[derive(Accounts)]
pub struct InitObligation2<'info>{
    pub init_obligation:InitObligation<'info>,
    #[account(mut)]
    pub payer:Signer<'info>,
    pub system_program:AccountInfo<'info>
}
///
/// @param ctx.remaining_accounts: deposit reserves and borrow reserves that should be refreshed
///
#[derive(Accounts)]
pub struct RefreshObligation<'info>{
    #[account(mut)]
    pub obligation:AccountInfo<'info>,
}
///
/// @param ctx.remaining_accounts: deposit reserves and borrow reserves
///
#[derive(Accounts)]
pub struct DepositObligationCollateral<'info>{
    #[account(mut)]
    pub source_collateral: AccountInfo<'info>,
    #[account(mut)]
    pub destination_collateral: AccountInfo<'info>,
    #[account(mut)]
    pub deposit_reserve: AccountInfo<'info>,
    #[account(mut)]
    pub obligation: AccountInfo<'info>,
    pub lending_market: AccountInfo<'info>,
    pub lending_market_authority:AccountInfo<'info>,
    pub obligation_owner: Signer<'info>,
    pub user_transfer_authority: Signer<'info>,
    pub token_program:AccountInfo<'info>
}
#[derive(Accounts)]
pub struct DepositObligationCollateral2<'info>{
    #[account(mut)]
    pub source_collateral: AccountInfo<'info>,
    #[account(mut)]
    pub destination_collateral: AccountInfo<'info>,
    #[account(mut)]
    pub deposit_reserve: AccountInfo<'info>,
    #[account(mut)]
    pub obligation: AccountInfo<'info>,
    pub lending_market: AccountInfo<'info>,
    pub obligation_owner: Signer<'info>,
    pub user_transfer_authority: Signer<'info>,
    pub token_program:AccountInfo<'info>
}
#[derive(Accounts)]
pub struct WithdrawObligationCollateral<'info>{
    #[account(mut)]
    pub source_collateral: AccountInfo<'info>,
    #[account(mut)]
    pub destination_collateral: AccountInfo<'info>,
    ///
    /// refreshed
    ///
    #[account(mut)]
    pub withdraw_reserve: AccountInfo<'info>,
    ///
    /// refreshed when borrows is not empty.
    ///
    pub obligation: AccountInfo<'info>,
    pub lending_market: AccountInfo<'info>,
    pub obligation_owner: Signer<'info>,
}
#[derive(Accounts)]
pub struct BorrowObligationLiquidity<'info>{
    #[account(mut)]
    pub source_liquidity: AccountInfo<'info>,
    #[account(mut)]
    pub destination_liquidity: AccountInfo<'info>,
    #[account(mut)]
    pub borrow_reserve: AccountInfo<'info>,
    #[account(mut)]
    pub obligation: AccountInfo<'info>,
    pub lending_market: AccountInfo<'info>,
    pub lending_market_authority:AccountInfo<'info>,
    pub obligation_owner: Signer<'info>,
    pub token_program:AccountInfo<'info>,
    #[account(mut)]
    pub borrow_reserve_liquidity_fee_receiver: AccountInfo<'info>,
    pub larix_oracle_program:AccountInfo<'info>,
    pub mine_mint:AccountInfo<'info>
}
#[derive(Accounts)]
pub struct RepayObligationLiquidity<'info>{
    #[account(mut)]
    pub source_liquidity: AccountInfo<'info>,
    #[account(mut)]
    pub destination_liquidity: AccountInfo<'info>,
    #[account(mut)]
    pub repay_reserve: AccountInfo<'info>,
    #[account(mut)]
    pub obligation: AccountInfo<'info>,
    pub lending_market: AccountInfo<'info>,
    pub user_transfer_authority: Signer<'info>,
    pub token_program:AccountInfo<'info>
}
#[derive(Accounts)]
pub struct ClaimObligationMine<'info>{
    #[account(mut)]
    pub obligation:AccountInfo<'info>,
    #[account(mut)]
    pub mine_supply:AccountInfo<'info>,
    #[account(mut)]
    pub destination_account:AccountInfo<'info>,
    pub obligation_owner:Signer<'info>,
    pub lending_market:AccountInfo<'info>,
    pub lending_market_authority:AccountInfo<'info>
}
#[derive(Accounts)]
pub struct LiquidateObligation<'info>{
    pub liquidity_obligation_2:LiquidateObligation2<'info>,
    pub clock:AccountInfo<'info>
}
#[derive(Accounts)]
pub struct LiquidateObligation2<'info>{
    #[account(mut)]
    pub source_liquidity:AccountInfo<'info>,
    #[account(mut)]
    pub destination_collateral:AccountInfo<'info>,
    #[account(mut)]
    pub repay_reserve:AccountInfo<'info>,
    #[account(mut)]
    pub repay_reserve_liquidity_supply:AccountInfo<'info>,
    #[account(mut)]
    pub withdraw_reserve:AccountInfo<'info>,
    #[account(mut)]
    pub withdraw_reserve_collateral_supply:AccountInfo<'info>,
    #[account(mut)]
    pub obligation:AccountInfo<'info>,
    pub lending_market_:AccountInfo<'info>,
    pub user_transfer_authority:Signer<'info>,
    pub token_program:AccountInfo<'info>
}