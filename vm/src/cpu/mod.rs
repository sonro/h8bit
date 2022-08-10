use self::register::InvalidRegister;

pub mod operation;
mod register;
#[cfg(test)]
mod tests;

pub type OpResult = Result<(), Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("CPU halted")]
    Halt,
    #[error(transparent)]
    InvalidRegister(#[from] InvalidRegister),
    #[error("no memory")]
    NoMemory,
    #[error("unknown internal error")]
    Unknown(Box<dyn std::error::Error>),
}
