//! Instruction types

use crate::utils::program::{get_address_pair, EthereumAddress};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_program, sysvar,
};

/// Eth address
#[derive(Clone, BorshDeserialize, BorshSerialize, PartialEq, Debug)]
pub struct CreateTokenAccount {
    /// Ethereum address
    pub eth_address: EthereumAddress,
}

/// Eth address
#[derive(Clone, BorshDeserialize, BorshSerialize, PartialEq, Debug)]
pub struct Claim {
    /// Ethereum address
    pub eth_address: EthereumAddress,
    /// The amount of claiming tokens. If set 0 claim all tokens
    /// otherwise claim specified value
    pub amount: u64,
}

/// Instruction definition
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub enum ClaimableProgramInstruction {
    /// CreateTokenAccount
    ///
    ///   0. `[sw]` Account to pay for creating token acc
    ///   1. `[r]` Mint account
    ///   2. `[r]` Base acc used in PDA token acc (need because of create_with_seed instruction)
    ///   3. `[w]` PDA token account to create
    ///   4. `[r]` Rent id
    ///   5. `[r]` SPL token account id
    ///   6. `[r]` System program id
    CreateTokenAccount(CreateTokenAccount),

    /// Claim
    ///
    ///   0. `[w]` Token acc from which tokens will be send (bank account)
    ///   1. `[w]` Receiver token acc
    ///   2. `[r]` Banks token account authority
    ///   3. `[r]` Sysvar instruction id
    ///   4. `[r]` SPL token account id
    Claim(Claim),
}

/// Create `CreateTokenAccount` instruction
pub fn init(
    program_id: &Pubkey,
    fee_payer: &Pubkey,
    mint: &Pubkey,
    ethereum_address: CreateTokenAccount,
) -> Result<Instruction, ProgramError> {
    let pair = get_address_pair(mint, ethereum_address.eth_address)?;

    let data = ClaimableProgramInstruction::CreateTokenAccount(ethereum_address).try_to_vec()?;
    let accounts = vec![
        AccountMeta::new(*fee_payer, true),
        AccountMeta::new_readonly(*mint, false),
        AccountMeta::new_readonly(pair.base.address, false),
        AccountMeta::new(pair.derive.address, false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

/// Create `Claim` instruction
///
/// NOTE: Instruction must followed after `new_secp256k1_instruction`
/// with params: ethereum private key and user token account public key.
/// Otherwise error message `Secp256 instruction losing` will be issued
pub fn claim(
    program_id: &Pubkey,
    banks_token_acc: &Pubkey,
    users_token_acc: &Pubkey,
    authority: &Pubkey,
    eth_address: Claim,
) -> Result<Instruction, ProgramError> {
    let data = ClaimableProgramInstruction::Claim(eth_address).try_to_vec()?;
    let accounts = vec![
        AccountMeta::new(*banks_token_acc, false),
        AccountMeta::new(*users_token_acc, false),
        AccountMeta::new_readonly(*authority, false),
        AccountMeta::new_readonly(sysvar::instructions::id(), false),
        AccountMeta::new_readonly(spl_token::id(), false),
    ];
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}
