use crate::cpu::{Cpu, OpResult};

pub const CODE: u8 = 0x11;
pub const NAME: &str = "MOV_LIT_REG_WIDE";
pub const SIZE: u8 = 4;

pub(in crate::cpu::operation) fn run(cpu: &mut Cpu) -> OpResult {
    let literal = cpu.fetch_wide()?;
    let reg = cpu.fetch_register_wide()?;
    cpu.registers.set_wide(reg, literal);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        cpu::{
            operation::tests::{
                op_run_success, test_builder_size, test_invalid_register, test_run_no_mem,
            },
            tests::TestCpuState,
            WideRegister,
        },
        util::high_and_low_value,
    };

    #[test]
    fn success() {
        let literal = 0xABCD;
        let reg = WideRegister::CD;

        let mut expected = TestCpuState::new();
        expected.reg_wide(reg, literal);

        op_run_success(&expected, &mut builder(literal, reg), run);
    }

    test_builder_size!(builder(0xABCD, WideRegister::AB), SIZE);

    test_invalid_register!(&[0xAB, 0x00], 0x00);

    test_run_no_mem!();

    fn builder(literal: u16, reg: WideRegister) -> TestCpuState {
        let (high, low) = high_and_low_value(literal);
        let opargs = [high, low, reg.into()];
        TestCpuState::new_with_program(&opargs)
    }
}
