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
