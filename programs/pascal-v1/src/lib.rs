use anchor_lang::prelude::*;
use anchor_spl::{
    token::{
        Mint,
        MintTo,
        mint_to,
        TokenAccount,
        Token
    }, 
    associated_token::AssociatedToken
};

declare_id!("E7nxGLpLSjgNaeMmLXQT9s4T7naheQoBiaxxjf7YwQiT");

#[program]
pub mod pascal_v1 {
    use super::*;

    pub fn place_order(
        ctx: Context<PlaceOrder>, 
        market_id: String, 
        description: String, 
        contracts: u8
    ) -> Result<()> {
        msg!("Movie review account created");
        msg!("Market ID: {}", market_id);
        msg!("Description: {}", description);
        msg!("Contracts: {}", contracts);

        require!(contracts > 0, OrderError::InvalidContracts);

        let movie_review = &mut ctx.accounts.movie_review;
        movie_review.trader = ctx.accounts.initializer.key();
        movie_review.market_id = market_id;
        movie_review.contracts = contracts;

        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(), 
                MintTo {
                    authority: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                }, 
                &[&[
                    b"mint",
                    &[*ctx.bumps.get("mint").unwrap()]
                ]]
            ),
            10*10^6
        )?;

        msg!("minted tokens");

        Ok(())
    }

    pub fn update_movie_review(
        ctx: Context<UpdateMoviewReview>,
        market_id: String, 
        description: String, 
        contracts: u8
    ) -> Result<()> {
        msg!("Movie review account updated");
        msg!("Market ID: {}", market_id);
        msg!("Description: {}", description);
        msg!("Contracts: {}", contracts);

        require!(contracts > 0, OrderError::InvalidContracts);

        let movie_review = &mut ctx.accounts.movie_review;
        movie_review.description = description;
        movie_review.contracts = contracts;

        Ok(())
    }

    pub fn delete_movie_review(
        _ctx: Context<DeleteMoviewReview>,
        market_id: String
    ) -> Result<()> {
        msg!("Movie review for {} deleted", market_id);
        Ok(())
    }

    pub fn initialize_token_mint(
        _ctx: Context<InitializeMint>,

    ) -> Result<()> {
        msg!("Token mint initialized");
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(market_id: String, description: String)]
pub struct PlaceOrder<'info> {
    #[account(
        init,
        seeds=[market_id.as_bytes(), initializer.key().as_ref()],
        bump,
        payer = initializer,
        space = 8 + 32 + 1 + 4 + market_id.len() + 4 + description.len()
    )]

    pub movie_review: Account<'info, OrderAccountState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    #[account(
        seeds = ["mint".as_bytes().as_ref()],
        bump,
        mut
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = initializer,
        associated_token::mint = mint,
        associated_token::authority = initializer
    )]
    pub token_account: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>
}

#[derive(Accounts)]
#[instruction(market_id: String, description: String)]
pub struct UpdateMoviewReview<'info> {
    #[account(
        mut,
        seeds=[market_id.as_bytes(), initializer.key().as_ref()],
        bump,
        realloc = 8 + 32 + 1 + 4 + market_id.len() + 4 + description.len(),
        realloc::payer = initializer,
        realloc::zero = true
    )]
    pub movie_review: Account<'info, OrderAccountState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(market_id: String)]
pub struct DeleteMoviewReview<'info> {
    #[account(
        mut,
        seeds=[market_id.as_bytes(), trader.key().as_ref()],
        bump,
        close=trader,
        has_one=trader
    )]
    pub movie_review: Account<'info, OrderAccountState>,
    #[account(mut)]
    pub trader: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct InitializeMint<'info> {
    #[account(
        init,
        seeds = ["mint".as_bytes().as_ref()],
        bump,
        payer = user,
        mint::decimals = 6,
        mint::authority = mint
    )]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>
}

#[account]
pub struct OrderAccountState {
    pub trader: Pubkey,
    pub contracts: u8,
    pub market_id: String,
    pub description: String
}

#[error_code]
enum OrderError {
    #[msg("Contracts must be more than 0")]
    InvalidContracts
}