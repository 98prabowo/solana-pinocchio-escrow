use num_derive::FromPrimitive;
use pinocchio::program_error::{ProgramError, ToStr};
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum PinocchioError {
    // 0
    #[error("Account not a signer")]
    NotSigner,
    // 1
    #[error("Invalid account owner")]
    InvalidOwner,
    // 2
    #[error("Invalid account data")]
    InvalidAccountData,
    // 3
    #[error("Invalid program address")]
    InvalidAddress,
    // 4
    #[error("Lamport balance below rent-exempt threshold")]
    NoRentExempt,
}

impl From<PinocchioError> for ProgramError {
    fn from(e: PinocchioError) -> Self {
        match e {
            PinocchioError::NotSigner => Self::MissingRequiredSignature,
            PinocchioError::InvalidOwner => Self::InvalidAccountOwner,
            PinocchioError::InvalidAccountData => Self::InvalidAccountData,
            PinocchioError::InvalidAddress => Self::IncorrectProgramId,
            PinocchioError::NoRentExempt => Self::Custom(e as u32),
        }
    }
}

impl TryFrom<u32> for PinocchioError {
    type Error = ProgramError;

    fn try_from(error: u32) -> Result<Self, Self::Error> {
        match error {
            0 => Ok(PinocchioError::NotSigner),
            1 => Ok(PinocchioError::InvalidOwner),
            2 => Ok(PinocchioError::InvalidAccountData),
            3 => Ok(PinocchioError::InvalidAddress),
            4 => Ok(PinocchioError::NoRentExempt),
            _ => Err(ProgramError::InvalidArgument),
        }
    }
}

impl ToStr for PinocchioError {
    fn to_str<E>(&self) -> &'static str
    where
        E: 'static + ToStr + TryFrom<u32>,
    {
        match self {
            PinocchioError::NotSigner => "Error: Account not a signer",
            PinocchioError::InvalidOwner => "Error: Invalid account owner",
            PinocchioError::InvalidAccountData => "Error: Invalid account data",
            PinocchioError::InvalidAddress => "Error: Invalid program address",
            PinocchioError::NoRentExempt => "Error: lamport balance below rent-exempt threshold",
        }
    }
}
