use super::{Cpu, OpResult};
use std::fmt;
use strum_macros::{FromRepr, IntoStaticStr};

pub mod hlt;
pub mod nop;

#[derive(Debug, PartialEq, Copy, Clone, FromRepr, IntoStaticStr)]
#[repr(u8)]
pub enum Operation {
    /// No operation
    Nop = nop::CODE,

    /// Halt
    Hlt = hlt::CODE,
}

impl Operation {
    pub fn execute(&self, cpu: &mut Cpu) -> OpResult {
        match self {
            Operation::Nop => nop::run(cpu),
            Operation::Hlt => hlt::run(cpu),
        }
    }

    pub fn as_str(&self) -> &'static str {
        self.into()
    }

    pub fn addr(&self) -> u8 {
        (*self).into()
    }
}

impl From<u8> for Operation {
    fn from(code: u8) -> Self {
        if let Some(op) = Self::from_repr(code) {
            op
        } else {
            Self::Nop
        }
    }
}

impl From<Operation> for u8 {
    fn from(op: Operation) -> Self {
        op as Self
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name: &str = self.into();
        write!(f, "{}", name)
    }
}
