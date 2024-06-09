use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount, mint_to, MintTo}
};

declare_id!("DQRdzv6NdrUEGWvMfbiqXDvFQSQ9kEnW76w2cpk84BmV");

#[program]
pub mod anchor_student_intro {
    use super::*;

    pub fn add_student_intro(
        ctx: Context<AddStudentIntro>,
        name: String,
        message: String
    ) -> Result<()> {
        msg!("adding student intro");
        msg!("name: {}", name);
        msg!("message: {}", message);

        require!(name.chars().count() >= 1 && name.chars().count() <= 20, IntroError::InvalidName);
        require!(message.chars().count() >= 1 && message.chars().count() <= 50, IntroError::InvalidMessage);

        let intro = &mut ctx.accounts.intro;
        intro.initializer = ctx.accounts.initializer.key();
        intro.name = name;
        intro.message = message;

        let counter = &mut ctx.accounts.comment_counter;
        counter.intro_account = ctx.accounts.intro.key();
        counter.count = 0;

        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(), 
                MintTo {
                    authority: ctx.accounts.reward_mint.to_account_info(),
                    mint: ctx.accounts.reward_mint.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info()
                }, 
                &[&[
                    "mint".as_bytes(),
                    &[ctx.bumps.reward_mint]
                ]]
            ),
            10
        )?;

        msg!("Minted tokens");

        Ok(())
    }

    pub fn update_student_intro(
        ctx: Context<UpdateStudentIntro>,
        name: String,
        new_message: String
    ) -> Result<()> {
        msg!("updating student intro");
        msg!("name: {}", name);
        msg!("new message: {}", new_message);

        require!(new_message.chars().count() >= 1 && new_message.chars().count() <= 50, IntroError::InvalidMessage);

        let intro = &mut ctx.accounts.intro;
        intro.message = new_message;

        Ok(())
    }

    pub fn add_comment_to_intro(
        ctx: Context<AddCommentToIntro>,
        comment: String
    ) -> Result<()> {
        msg!("adding comment to student intro");
        msg!("comment: {}", comment);

        let comment_account = &mut ctx.accounts.comment_account;
        comment_account.comment = comment;
        comment_account.commenter = ctx.accounts.commenter.key();
        comment_account.intro = ctx.accounts.intro.key();

        let comment_counter = &mut ctx.accounts.comment_counter;
        comment_counter.count = comment_counter.count.checked_add(1).unwrap();

        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(), 
                MintTo {
                    mint: ctx.accounts.reward_mint.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    authority: ctx.accounts.reward_mint.to_account_info()
                }, 
                &[&[
                    "mint".as_bytes(),
                    &[ctx.bumps.reward_mint]
                ]]
            ), 
            2
        )?;

        msg!("Minted tokens");

        Ok(())
    }

    pub fn close_student_intro(
        _ctx: Context<CloseStudentIntro>,
        name: String
    ) -> Result<()> {
        msg!("closing student intro");
        msg!("name: {}", name);

        Ok(())
    }

    pub fn initialize_mint(
        _ctx: Context<InitializeMint>
    ) -> Result<()> {
        msg!("initialized mint");

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(name: String, message: String)]
pub struct AddStudentIntro<'info> {
    #[account(
        init,
        seeds = [name.as_bytes(), initializer.key().as_ref()],
        bump,
        payer = initializer,
        space = 8 + 32 + 4 + name.len() + 4 + message.len()
    )]
    pub intro: Account<'info, IntroState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    #[account(
        mut,
        seeds = ["mint".as_bytes()],
        bump,
    )]
    pub reward_mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = initializer,
        associated_token::mint = reward_mint,
        associated_token::authority = initializer
    )]
    pub token_account: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
    #[account(
        init,
        seeds = ["counter".as_bytes(), intro.key().as_ref()],
        bump,
        payer = initializer,
        space = 8 + 32 + 4
    )]
    pub comment_counter: Account<'info, CommentCounterState>
}

#[derive(Accounts)]
#[instruction(name: String, new_message: String)]
pub struct UpdateStudentIntro<'info> {
    #[account(
        mut,
        seeds = [name.as_bytes(), initializer.key().as_ref()],
        bump,
        realloc = 8 + 32 + 4 + name.len() + 4 + new_message.len(),
        realloc::payer = initializer,
        realloc::zero = true
    )]
    pub intro: Account<'info, IntroState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(comment: String)]
pub struct AddCommentToIntro<'info> {
    #[account(
        init,
        seeds = [commenter.key().as_ref(), intro.key().as_ref()],
        bump,
        payer = commenter,
        space = 8 + 32 + 32 + 4 + comment.len()
    )]
    pub comment_account: Account<'info, IntroCommentState>,
    #[account(
        seeds = [intro.name.as_bytes(), intro.initializer.key().as_ref()],
        bump
    )]
    pub intro: Account<'info, IntroState>,
    #[account(mut)]
    pub commenter: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(
        mut,
        seeds = ["counter".as_bytes(), intro.key().as_ref()],
        bump
    )]
    pub comment_counter: Account<'info, CommentCounterState>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
    #[account(
        mut,
        seeds = ["mint".as_bytes()],
        bump
    )]
    pub reward_mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = commenter,
        associated_token::mint = reward_mint,
        associated_token::authority = commenter
    )]
    pub token_account: Account<'info, TokenAccount>
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CloseStudentIntro<'info> {
    #[account(
        mut,
        seeds = [name.as_bytes(), initializer.key().as_ref()],
        bump,
        close = initializer
    )]
    pub intro: Account<'info, IntroState>,
    #[account(mut)]
    pub initializer: Signer<'info>
}

#[derive(Accounts)]
pub struct InitializeMint<'info> {
    #[account(
        init,
        seeds = ["mint".as_bytes()],
        bump,
        payer = initializer,
        mint::decimals = 0,
        mint::authority = reward_mint
    )]
    pub reward_mint: Account<'info, Mint>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>
}

#[account]
pub struct IntroState {
    pub initializer: Pubkey,
    pub name: String,
    pub message: String
}

#[account]
pub struct IntroCommentState {
    pub intro: Pubkey,
    pub commenter: Pubkey,
    pub comment: String,
}

#[account]
pub struct CommentCounterState {
    pub intro_account: Pubkey,
    pub count: u32
}

#[error_code]
pub enum IntroError {
    #[msg("Name must be between 1 and 20 characters")]
    InvalidName,
    #[msg("Message must be between 1 and 50 characters")]
    InvalidMessage
}