use solana_program::entrypoint::ProgramResult;
use {
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        //instruction::{AccountMeta, Instruction},
        account_info::{next_account_info, AccountInfo},
        program_error::ProgramError,
        pubkey::Pubkey,
        msg,
        //system_program,
    },
};
use crate::state::GreetingAccount;

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum NameInstruction {
    Greeting,
}

pub fn greeting ( program_id: &Pubkey,
                  accounts: &[AccountInfo],// Public key of the account the hello world program was loaded into
                  ) -> ProgramResult {
    // Iterating accounts is safer than indexing
    let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to
    let account = next_account_info(accounts_iter)?;
    //AccountMeta::new(account, true);
    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("Greeted account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }
    let mut greeting_account = GreetingAccount::try_from_slice(&account.data.borrow())?;
    greeting_account.counter += 1;
    greeting_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    msg!("Greeted {} time(s)!", greeting_account.counter);


    Ok(())
}
