//! Instruction types

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
};

/// Instruction definition
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub enum ClaimableProgramInstruction {
    /// Example.
    ///
    ///   0. `[w]` Example account.
    ExampleInstruction,
}

/// Create `Example` instruction
pub fn init(
    program_id: &Pubkey,
    example_account: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let init_data = ClaimableProgramInstruction::ExampleInstruction;
    let data = init_data
        .try_to_vec()
        .or(Err(ProgramError::InvalidArgument))?;
    let accounts = vec![AccountMeta::new(*example_account, false)];
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}
