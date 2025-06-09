//! Define enums de instruções customizadas (opcional para Anchor).

use anchor_lang::prelude::*;

/// Enum opcional para instruções customizadas.
/// Anchor normalmente usa funções no #[program], mas você pode usar enums para integração com clients low-level.
#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum AusdInstruction {
    Initialize { tree_height: u32 },
    DepositPrivate { amount: u64, commitment: [u8; 32] },
    TransferPrivate { nullifier_seed: [u8; 32], proof: Vec<u8>, nullifier: [u8; 32] },
    WithdrawPrivate { nullifier_seed: [u8; 32], amount: u64, proof: Vec<u8>, nullifier: [u8; 32] },
}
