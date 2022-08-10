use crate::cpu::{Cpu, OpResult};

pub const CODE: u8 = 0x00;
pub const NAME: &str = "NOP";
pub const SIZE: u8 = 1;

pub(in crate::cpu::operation) fn run(_cpu: &mut Cpu) -> OpResult {
    Ok(())
}
