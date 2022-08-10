use crate::cpu::{Cpu, OpResult};

pub const CODE: u8 = 0x10;
pub const NAME: &str = "MOV_LIT_REG";
pub const SIZE: u8 = 3;

pub(in crate::cpu::operation) fn run(cpu: &mut Cpu) -> OpResult {
    let literal = cpu.fetch()?;
    let reg = cpu.fetch_register()?;
    cpu.registers.set(reg, literal);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::{
        operation::tests::{
            op_run_success, test_builder_size, test_invalid_register, test_run_no_mem,
        },
        tests::TestCpuState,
        Register,
    };

    #[test]
    fn success() {
        let literal = 0xAB;
        let reg = Register::C;

        let mut expected = TestCpuState::new();
        expected.reg(reg, literal);

        op_run_success(&expected, &mut builder(literal, reg), run);
    }

    test_builder_size!(builder(0xAB, Register::C), SIZE);

    test_invalid_register!(&[0xAB, 0x00], 0x00);

    test_run_no_mem!();

    fn builder(literal: u8, reg: Register) -> TestCpuState {
        let opargs = [literal, reg.into()];
        TestCpuState::new_with_program(&opargs)
    }
}
