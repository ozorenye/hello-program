/// Define the type of state stored in accounts
use solana_program::{
account_info::{next_account_info, AccountInfo},
entrypoint::ProgramResult,
msg,
program::{invoke, invoke_signed},
program_error::ProgramError,
program_pack::Pack,
pubkey::Pubkey,
system_instruction,
};
use borsh::{BorshSerialize,BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    /// number of greetings
    pub counter: u32,
}
// Increment and store the number of times the account has been greeted
