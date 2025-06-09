use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount, Token, Transfer};
use crate::program_accounts::{Initialize, DepositPrivate, TransferPrivate, WithdrawPrivate};
use crate::state::ProgramState;
use crate::proof;

pub fn initialize(ctx: Context<Initialize>, tree_height: u32) -> Result<()> {
    msg!("aUSD Confidential Program Initialized!");

    ctx.accounts.program_state.initialize(
        tree_height,
        ctx.bumps.program_state,
        ctx.bumps.confidential_token_pool,
        ctx.accounts.merkle_tree.key(),
    )?;
    ctx.accounts.merkle_tree.initialize(
        tree_height,
        ctx.bumps.merkle_tree,
        ctx.accounts.program_state.key(),
    )?;
    Ok(())
}

pub fn deposit_private(ctx: Context<DepositPrivate>, amount: u64, commitment: [u8; 32]) -> Result<()> {
    msg!("Confidential deposit of {} aUSD with commitment {:?}", amount, commitment);

    transfer_tokens(
        &TransferTokensParams {
            source_account: ctx.accounts.from_token_account.to_account_info(),
            destination_account: ctx.accounts.confidential_token_pool.to_account_info(),
            authority: ctx.accounts.depositor.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            amount,
        },
    )?;

    ctx.accounts.merkle_tree.add_leaf(commitment, &ctx.accounts.program_state)?;

    Ok(())
}

pub fn transfer_private(ctx: Context<TransferPrivate>, _nullifier_seed: [u8; 32], proof: Vec<u8>, nullifier: [u8; 32]) -> Result<()> {
    msg!("Confidential transfer with nullifier {:?}", nullifier);

    proof::verify_zk_proof(&proof)?;

    ctx.accounts.nullifier_account.record_nullifier(
        nullifier,
        ctx.bumps.nullifier_account,
    )?;

    msg!("Tokens conceptually transferred privately.");
    Ok(())
}

pub fn withdraw_private(ctx: Context<WithdrawPrivate>, _nullifier_seed: [u8; 32], amount: u64, proof: Vec<u8>, nullifier: [u8; 32]) -> Result<()> {
    msg!("Confidential withdrawal of {} aUSD with nullifier {:?}", amount, nullifier);

    proof::verify_zk_proof(&proof)?;

    ctx.accounts.nullifier_account.record_nullifier(
        nullifier,
        ctx.bumps.nullifier_account,
    )?;

    withdraw_tokens(
        &WithdrawTokensParams {
            destination_account: ctx.accounts.to_token_account.to_account_info(),
            confidential_token_pool: ctx.accounts.confidential_token_pool.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            token_mint: ctx.accounts.token_mint.to_account_info(),
            program_state: ctx.accounts.program_state.to_account_info(),
            confidential_token_pool_bump: ctx.accounts.program_state.confidential_token_pool_bump,
            amount,
        },
    )?;

    Ok(())
}

// CPI helpers
struct TransferTokensParams<'info> {
    source_account: AccountInfo<'info>,
    destination_account: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    token_program: AccountInfo<'info>,
    amount: u64,
}

fn transfer_tokens(params: &TransferTokensParams<'_>) -> Result<()> {
    let accounts = Transfer {
        from: params.source_account.clone(),
        to: params.destination_account.clone(),
        authority: params.authority.clone(),
    };
    let cpi_context = CpiContext::new(
        params.token_program.clone(),
        accounts
    );
    anchor_spl::token::transfer(cpi_context, params.amount)?;
    Ok(())
}

struct WithdrawTokensParams<'info> {
    destination_account: AccountInfo<'info>,
    confidential_token_pool: AccountInfo<'info>,
    token_program: AccountInfo<'info>,
    token_mint: AccountInfo<'info>,
    program_state: AccountInfo<'info>,
    confidential_token_pool_bump: u8,
    amount: u64,
}

fn withdraw_tokens(params: &WithdrawTokensParams<'_>) -> Result<()> {
    let accounts = Transfer {
        from: params.confidential_token_pool.clone(),
        to: params.destination_account.clone(),
        authority: params.confidential_token_pool.clone(),
    };

    let token_mint_key = params.token_mint.key();

    let confidential_token_pool_seed = &[
        b"confidential_pool_seed",
        token_mint_key.as_ref(),
        &[params.confidential_token_pool_bump],
    ];
    let signer_seeds: &[&[&[u8]]] = &[confidential_token_pool_seed];

    let cpi_context = CpiContext::new_with_signer(
        params.token_program.clone(),
        accounts,
        signer_seeds
    );

    anchor_spl::token::transfer(cpi_context, params.amount)?;
    Ok(())
}