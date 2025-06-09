use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount, Token};
use crate::state::{ProgramState, MerkleTreeState, NullifierState};

#[derive(Accounts)]
#[instruction(tree_height: u32)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,

    #[account(
        init,
        payer = initializer,
        space = ProgramState::SPACE,
        seeds = [b"program_state_seed"],
        bump,
    )]
    pub program_state: Account<'info, ProgramState>,

    #[account(
        init,
        payer = initializer,
        token::mint = token_mint,
        token::authority = program_state,
        seeds = [b"confidential_pool_seed", token_mint.key().as_ref()],
        bump,
    )]
    pub confidential_token_pool: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = initializer,
        space = MerkleTreeState::space(tree_height),
        seeds = [b"merkle_tree_state_seed"],
        bump,
    )]
    pub merkle_tree: Account<'info, MerkleTreeState>,

    pub token_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct DepositPrivate<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,

    #[account(mut, token::authority = depositor)]
    pub from_token_account: Account<'info, TokenAccount>,

    pub token_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = depositor,
        seeds = [b"confidential_pool_seed", token_mint.key().as_ref()],
        bump,
        token::mint = token_mint,
        token::authority = program_state,
    )]
    pub confidential_token_pool: Account<'info, TokenAccount>,

    #[account(mut, seeds = [b"merkle_tree_state_seed"], bump = merkle_tree.merkle_tree_bump)]
    pub merkle_tree: Account<'info, MerkleTreeState>,

    #[account(
        seeds = [b"program_state_seed"],
        bump = program_state.program_state_bump,
    )]
    pub program_state: Account<'info, ProgramState>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(nullifier_seed: [u8; 32])]
pub struct TransferPrivate<'info> {
    #[account(mut)]
    pub fee_payer: Signer<'info>,

    #[account(mut, seeds = [b"merkle_tree_state_seed"], bump = merkle_tree.merkle_tree_bump)]
    pub merkle_tree: Account<'info, MerkleTreeState>,

    #[account(
        init_if_needed,
        payer = fee_payer,
        space = NullifierState::SPACE,
        seeds = [b"nullifier_seed", nullifier_seed.as_ref()],
        bump,
    )]
    pub nullifier_account: Account<'info, NullifierState>,

    #[account(mut)]
    pub confidential_token_pool: Account<'info, TokenAccount>,

    #[account(mut)]
    pub recipient_token_account: Account<'info, TokenAccount>,

    pub token_mint: Account<'info, Mint>,

    #[account(
        seeds = [b"program_state_seed"],
        bump = program_state.program_state_bump,
    )]
    pub program_state: Account<'info, ProgramState>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(nullifier_seed: [u8; 32])]
pub struct WithdrawPrivate<'info> {
    #[account(mut)]
    pub withdrawer: Signer<'info>,

    #[account(mut, token::authority = withdrawer)]
    pub to_token_account: Account<'info, TokenAccount>,

    pub token_mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"confidential_pool_seed", token_mint.key().as_ref()],
        bump = program_state.confidential_token_pool_bump,
        token::mint = token_mint,
    )]
    pub confidential_token_pool: Account<'info, TokenAccount>,

    #[account(mut, seeds = [b"merkle_tree_state_seed"], bump = merkle_tree.merkle_tree_bump)]
    pub merkle_tree: Account<'info, MerkleTreeState>,

    #[account(
        init_if_needed,
        payer = withdrawer,
        space = NullifierState::SPACE,
        seeds = [b"nullifier_seed", nullifier_seed.as_ref()],
        bump,
    )]
    pub nullifier_account: Account<'info, NullifierState>,

    #[account(
        seeds = [b"program_state_seed"],
        bump = program_state.program_state_bump,
    )]
    pub program_state: Account<'info, ProgramState>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}