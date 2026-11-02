use anchor_lang::prelude::*;

declare_id!("8f6ZoJv8UeBsY7khEKX7ddhcjt2JLhKbHQdRnCBZ3mYa");

#[account]
pub struct Pool {
    pub authority: Pubkey,
    pub reward_rate: u64,
    pub total_staked: u64,
    pub created_at: i64,
}

#[account]
pub struct Stake {
    pub pool: Pubkey,
    pub user: Pubkey,
    pub amount: u64,
    pub staked_at: i64,
    pub rewards_owed: u64,
}

impl Pool {
    pub const SIZE: usize = 8 + 32 + 8 + 8 + 8;
}

impl Stake {
    pub const SIZE: usize = 8 + 32 + 32 + 8 + 8 + 8;
}

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(init, payer = authority, space = Pool::SIZE)]
    pub pool: Account<'info, Pool>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub pool: Account<'info, Pool>,
    #[account(init, payer = user, space = Stake::SIZE)]
    pub stake: Account<'info, Stake>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub pool: Account<'info, Pool>,
    #[account(mut)]
    pub stake: Account<'info, Stake>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub pool: Account<'info, Pool>,
    #[account(mut, close = user)]
    pub stake: Account<'info, Stake>,
    pub system_program: Program<'info, System>,
}

#[program]
pub mod staking_pool {
    use super::*;
    use anchor_lang::system_program::{transfer, Transfer};

    pub fn initialize_pool(ctx: Context<InitializePool>, reward_rate: u64) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        pool.authority = ctx.accounts.authority.key();
        pool.reward_rate = reward_rate;
        pool.total_staked = 0;
        pool.created_at = Clock::get()?.unix_timestamp;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let stake = &mut ctx.accounts.stake;
        let pool = &mut ctx.accounts.pool;
        stake.pool = pool.key();
        stake.user = ctx.accounts.user.key();
        stake.staked_at = Clock::get()?.unix_timestamp;
        stake.rewards_owed = 0;
        stake.amount = amount;
        pool.total_staked += amount;
        transfer(
            CpiContext::new(
                ctx.accounts.system_program.key(),
                Transfer {
                    from: ctx.accounts.user.to_account_info(),
                    to: ctx.accounts.pool.to_account_info(),
                },
            ),
            amount,
        )?;
        Ok(())
    }

    pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
        let stake = &mut ctx.accounts.stake;
        let pool = &ctx.accounts.pool;
        let now = Clock::get()?.unix_timestamp;
        let elapsed = (now - stake.staked_at).max(0) as u64;
        let rewards = (stake.amount as u128 * pool.reward_rate as u128 * elapsed as u128) / 1_000_000_000;
        stake.rewards_owed += rewards as u64;
        stake.staked_at = now;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        let stake = &ctx.accounts.stake;
        let pool = &mut ctx.accounts.pool;
        let total = stake.amount + stake.rewards_owed;
        pool.total_staked = pool.total_staked.saturating_sub(stake.amount);
        transfer(
            CpiContext::new(
                ctx.accounts.system_program.key(),
                Transfer {
                    from: ctx.accounts.pool.to_account_info(),
                    to: ctx.accounts.user.to_account_info(),
                },
            ),
            total,
        )?;
        Ok(())
    }
}
