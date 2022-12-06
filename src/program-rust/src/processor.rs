use {
    crate::{
        instruction::NameInstruction
    },
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        program_error::ProgramError,
        pubkey::Pubkey,
    },
};
use crate::state::GreetingAccount;
//use crate::error::NameError;
pub struct Processor {}



impl Processor {
    
    pub fn process_greeting(program_id: &Pubkey,
                            accounts: &[AccountInfo],// Public key of the account the hello world program was loaded into
    ) -> ProgramResult {
        println!("process_greeting");
        // Iterating accounts is safer than indexing
        let accounts_iter = &mut accounts.iter();
        let (pda,_)= Pubkey::find_program_address(&[b"admin_account"], program_id);
        let account = next_account_info(accounts_iter)?;


        if account.key != &pda {
            return  Err(ProgramError::MissingRequiredSignature);
        }
        let mut greeting_account = GreetingAccount::try_from_slice(&account.data.borrow())?;
        greeting_account.counter += 1;
        greeting_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

        msg!("Greeted {} time(s)!", greeting_account.counter);


        Ok(())
    }
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        _instruction_data: &[u8],
    ) -> ProgramResult {
        msg!("Beginning processing");
        let instruction = NameInstruction::Greeting;
            // .map_err(|_| ProgramError::InvalidInstructionData)?;
        msg!("Instruction unpacked");

        match instruction {
            NameInstruction::Greeting {} => { msg!("Instruction: Greeting");
                Processor::process_greeting(program_id, accounts)?;
            }

        }
        Ok(())
    }
}