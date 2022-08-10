use crate::cpu::{Cpu, Error, OpResult};

pub const CODE: u8 = 0xff;
pub const NAME: &str = "HLT";
pub const SIZE: u8 = 1;

pub(in crate::cpu::operation) fn run(_cpu: &mut Cpu) -> OpResult {
    Err(Error::Halt)
}
