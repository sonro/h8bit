use crate::cpu::{AnyRegister, Cpu, OpResult};

pub const CODE: u8 = 0x12;
pub const NAME: &str = "MOV_REG_REG";
pub const SIZE: u8 = 3;

pub(in crate::cpu::operation) fn run(cpu: &mut Cpu) -> OpResult {
    match cpu.fetch_any_register()? {
        AnyRegister::Std(from) => {
            let to = cpu.fetch_register()?;
            let value = cpu.registers.get(from);
            cpu.registers.set(to, value);
        }
        AnyRegister::Wide(from) => {
            let to = cpu.fetch_register_wide()?;
            let value = cpu.registers.get_wide(from);
            cpu.registers.set_wide(to, value);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::cpu::{
        operation::tests::{
            op_run_success, test_builder_size, test_invalid_register, test_run_no_mem,
        },
        tests::TestCpuState,
        Register, WideRegister,
    };

    use super::*;

    #[test]
    fn success() {
        let value = 0xAB;
        let from = Register::C;
        let to = Register::D;

        let mut expected = TestCpuState::new();
        expected.reg(to, value);

        op_run_success(&expected, &mut builder(value, from, to), run);
    }

    #[test]
    fn wide_success() {
        let value = 0xABCD;
        let from = WideRegister::CD;
        let to = WideRegister::EF;

        let mut expected = TestCpuState::new();
        expected.reg_wide(to, value);

        op_run_success(&expected, &mut builder_wide(value, from, to), run);
    }

    test_builder_size!(builder(0xAB, Register::A, Register::B), SIZE);

    test_invalid_register!(&[0x00, Register::A.into()], 0x00, first);
    test_invalid_register!(&[Register::A.into(), 0x00], 0x00, second);
    test_invalid_register!(&[WideRegister::AB.into(), 0x00], 0x00, wide_second);
    test_invalid_register!(
        &[Register::A.into(), WideRegister::CD.into()],
        WideRegister::CD.into(),
        second_wide
    );
    test_invalid_register!(
        &[WideRegister::AB.into(), Register::C.into()],
        Register::C.into(),
        second_std
    );

    test_run_no_mem!();

    fn builder(value: u8, from: Register, to: Register) -> TestCpuState {
        let opargs = [from.into(), to.into()];
        let mut build = TestCpuState::new_with_program(&opargs);
        build.reg(from, value);
        build
    }

    fn builder_wide(value: u16, from: WideRegister, to: WideRegister) -> TestCpuState {
        let opargs = [from.into(), to.into()];
        let mut build = TestCpuState::new_with_program(&opargs);
        build.reg_wide(from, value);
        build
    }
}
