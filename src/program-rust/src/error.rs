use {
    num_derive::FromPrimitive,
    solana_program::{decode_error::DecodeError, program_error::ProgramError},
    thiserror::Error,
};

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum NameError {
    #[error("Greeted account does not have the correct program id")]
    IncorrectProgramId,
}

pub type NameServiceResult = Result<(), NameError>;

impl From<NameError> for ProgramError {
    fn from(e: NameError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for NameError {
    fn type_of() -> &'static str {
        "NameError"
    }
}