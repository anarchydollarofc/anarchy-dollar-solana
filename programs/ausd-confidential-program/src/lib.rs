use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount, Token};

declare_id!("A6TAha2ysPWN6vFHpZGMf8Wnqb5TyCuvzoQKpmLVjre");

pub mod program_accounts;
pub use program_accounts::*;

pub mod instructions;
pub mod processor;
pub mod state;
pub mod proof;

#[program]
pub mod ausd_confidential_program {
    use super::*;
    use crate::program_accounts::{Initialize, DepositPrivate, TransferPrivate, WithdrawPrivate};

    pub fn initialize(ctx: Context<Initialize>, tree_height: u32) -> Result<()> {
        processor::initialize(ctx, tree_height)
    }

    pub fn deposit_private(ctx: Context<DepositPrivate>, amount: u64, commitment: [u8; 32]) -> Result<()> {
        processor::deposit_private(ctx, amount, commitment)
    }

    pub fn transfer_private(
        ctx: Context<TransferPrivate>,
        nullifier_seed: [u8; 32],
        proof: Vec<u8>,
        nullifier: [u8; 32],
    ) -> Result<()> {
        processor::transfer_private(ctx, nullifier_seed, proof, nullifier)
    }

    pub fn withdraw_private(
        ctx: Context<WithdrawPrivate>,
        nullifier_seed: [u8; 32],
        amount: u64,
        proof: Vec<u8>,
        nullifier: [u8; 32],
    ) -> Result<()> {
        processor::withdraw_private(ctx, nullifier_seed, amount, proof, nullifier)
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Merkle Tree is full.")]
    MerkleTreeFull,
    #[msg("Merkle Tree is too large.  Reduce the tree height.")]
    MerkleTreeTooLarge,
    #[msg("Invalid Merkle Proof.")]
    InvalidMerkleProof,
    #[msg("Invalid Account State.")]
    InvalidAccountState,
    #[msg("Invalid Fee Version.")]
    InvalidFeeVersion,
    #[msg("Insufficient Fee.")]
    InsufficientFee,
    #[msg("Invalid Recipient Address.")]
    InvalidRecipient,
    #[msg("Invalid Amount.")]
    InvalidAmount,
    #[msg("Computation Not Finished.")]
    ComputationIsNotYetFinished,
    #[msg("Computation Already Finished.")]
    ComputationIsAlreadyFinished,
    #[msg("Inputs Mismatch Expected Values.")]
    InputsMismatch,
    #[msg("Invalid Merkle Root.")]
    InvalidMerkleRoot,
    #[msg("Invalid Recent Commitment Index.")]
    InvalidRecentCommitmentIndex,
    #[msg("Could Not Insert Nullifier (Duplicate).")]
    CouldNotInsertNullifier,
    #[msg("Queue Is Full.")]
    QueueIsFull,
    #[msg("Invalid Other Instruction.")]
    InvalidOtherInstruction,
    #[msg("Feature Not Available.")]
    FeatureNotAvailable,
    #[msg("ZK Proof Verification Failed.")]
    ZKProofVerificationFailed,
    #[msg("Bump seed not found")]
    BumpSeedNotFound,
    #[msg("Nullifier Already Exists")]
    NullifierAlreadyExists,
}

impl From<ErrorCode> for ProgramError {
    fn from(e: ErrorCode) -> Self {
        ProgramError::Custom(e as u32)
    }
}