use anchor_lang::prelude::*;

#[account]
pub struct ProgramState {
    pub root: [u8; 32],
    pub program_state_bump: u8,
    pub confidential_token_pool_bump: u8,
    pub merkle_tree: Pubkey,
    pub tree_height: u32,
}

impl ProgramState {
    pub const SPACE: usize = 8 + 32 + 1 + 1 + 32 + 4;

    pub fn initialize(
        &mut self,
        tree_height: u32,
        program_state_bump: u8,
        confidential_token_pool_bump: u8,
        merkle_tree: Pubkey,
    ) -> Result<()> {
        self.tree_height = tree_height;
        self.program_state_bump = program_state_bump;
        self.confidential_token_pool_bump = confidential_token_pool_bump;
        self.merkle_tree = merkle_tree;
        self.root = [0u8; 32];
        Ok(())
    }
}

#[account]
pub struct MerkleTreeState {
    pub leaves: Vec<[u8; 32]>,
    pub root: [u8; 32],
    pub merkle_tree_bump: u8,
    pub program_state: Pubkey,
    pub tree_height: u32,
    pub leaf_count: u32,
}

impl MerkleTreeState {
    pub fn space(tree_height: u32) -> usize {
        let max_leaves: usize = 1 << tree_height;
        8 + 4 + (max_leaves * 32) + 32 + 1 + 32 + 4 + 4
    }

    pub fn initialize(&mut self, tree_height: u32, bump: u8, program_state: Pubkey) -> Result<()> {
        if tree_height > 8 {
            return err!(crate::ErrorCode::MerkleTreeTooLarge);
        }
        self.tree_height = tree_height;
        self.merkle_tree_bump = bump;
        self.program_state = program_state;
        self.root = [0u8; 32];
        self.leaves = Vec::new();
        self.leaf_count = 0;
        Ok(())
    }

    pub fn add_leaf(&mut self, commitment: [u8; 32], _program_state: &Account<ProgramState>) -> Result<()> {
        if self.leaves.len() >= (1 << self.tree_height) {
            return err!(crate::ErrorCode::MerkleTreeFull);
        }
        self.leaves.push(commitment);
        self.leaf_count += 1;
        self.root = commitment; // Placeholder root update.
        Ok(())
    }
}

#[account]
pub struct NullifierState {
    pub nullifier_hash: [u8; 32],
    pub nullifier_bump: u8,
}

impl NullifierState {
    pub const SPACE: usize = 8 + 32 + 1;

    pub fn record_nullifier(&mut self, nullifier: [u8; 32], bump: u8) -> Result<()> {
        if self.nullifier_hash != [0u8;32] {
            return err!(crate::ErrorCode::NullifierAlreadyExists);
        }
        self.nullifier_hash = nullifier;
        self.nullifier_bump = bump;
        Ok(())
    }
}