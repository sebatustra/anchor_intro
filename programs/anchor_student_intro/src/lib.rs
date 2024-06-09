use anchor_lang::prelude::*;

declare_id!("GztZcTBZTcv5DUwY6dRWgzFL5WhcNZ8Wx8zRSoLniXZM");

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

        let intro = &mut ctx.accounts.intro;
        intro.initializer = ctx.accounts.initializer.key();
        intro.name = name;
        intro.message = message;

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

        let intro = &mut ctx.accounts.intro;
        intro.message = new_message;

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
    pub system_program: Program<'info, System>
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

#[account]
pub struct IntroState {
    pub initializer: Pubkey,
    pub name: String,
    pub message: String
}