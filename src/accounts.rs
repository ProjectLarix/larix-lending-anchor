use anchor_lang::prelude::*;
#[derive(Accounts)]
pub struct InitLendingMarket<'info> {
    // signer
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub lending_market: AccountInfo<'info>,
    pub rent: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub pyth_orcale_program: AccountInfo<'info>,
    pub larix_orcale_program: AccountInfo<'info>,
    pub larix_orcale_id: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct RefreshReserve<'info> {
    #[account(mut)]
    pub reserve:AccountInfo<'info>,
    pub oracle:AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct DepositReserveLiquidity<'info> {
    #[account(mut)]
    pub source_liquidity_pubkey: AccountInfo<'info>,
    #[account(mut)]
    pub destination_collateral_pubkey: AccountInfo<'info>,
    #[account(mut)]
    pub reserve_pubkey: AccountInfo<'info>,
    #[account(mut)]
    pub reserve_collateral_mint_pubkey: AccountInfo<'info>,
    #[account(mut)]
    pub reserve_liquidity_supply_pubkey: AccountInfo<'info>,
    pub lending_market_pubkey: AccountInfo<'info>,
    pub user_transfer_authority_pubkey: Signer<'info>,
}
#[derive(Accounts)]
pub struct RedeemReserveCollateral<'info> {
    #[account(mut)]
    pub source_collateral_pubkey: AccountInfo<'info>,
    #[account(mut)]
    pub destination_liquidity_pubkey: AccountInfo<'info>,
    #[account(mut)]
    pub reserve_pubkey: AccountInfo<'info>,
    #[account(mut)]
    pub reserve_collateral_mint_pubkey: AccountInfo<'info>,
    #[account(mut)]
    pub reserve_liquidity_supply_pubkey: AccountInfo<'info>,
    pub lending_market_pubkey: AccountInfo<'info>,
    pub user_transfer_authority_pubkey: Signer<'info>,
}
#[derive(Accounts)]
pub struct InitObligation<'info>{
    #[account(mut)]
    pub obligation_pubkey: AccountInfo<'info>,
    pub lending_market_pubkey: AccountInfo<'info>,
    pub obligation_owner_pubkey: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct InitObligation2<'info>{
    #[account(mut)]
    pub obligation_pubkey: AccountInfo<'info>,
    pub lending_market_pubkey: AccountInfo<'info>,
    pub obligation_owner_pubkey: AccountInfo<'info>,
    #[account(mut)]
    pub payer:Signer<'info>,
    pub system_program:AccountInfo<'info>
}
#[derive(Accounts)]
pub struct RefreshObligation<'info>{
    #[account(mut)]
    pub obligation:AccountInfo<'info>,
    pub reserves:AccountInfo<'info>
}