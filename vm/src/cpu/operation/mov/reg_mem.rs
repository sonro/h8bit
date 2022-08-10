use crate::cpu::{AnyRegister, Cpu, OpResult};
use crate::memory::Device;

pub const CODE: u8 = 0x13;
pub const NAME: &str = "MOV_REG_MEM";
pub const SIZE: u8 = 4;

pub(in crate::cpu::operation) fn run(cpu: &mut Cpu) -> OpResult {
    match cpu.fetch_any_register()? {
        AnyRegister::Std(reg) => {
            let addr = cpu.fetch_wide()?;
            let value = cpu.registers.get(reg);
            cpu.memory.set(addr, value)?;
        }
        AnyRegister::Wide(reg) => {
            let addr = cpu.fetch_wide()?;
            let value = cpu.registers.get_wide(reg);
            cpu.memory.set_wide(addr, value)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        cpu::{
            operation::tests::{
                op_run_success, test_builder_size, test_invalid_memory, test_invalid_register,
                test_run_no_mem, TEST_OP_MEM_SIZE,
            },
            tests::TestCpuState,
            Register, WideRegister,
        },
        util::high_and_low_value,
    };

    #[test]
    fn success_std() {
        let value = 0xab;
        let addr = 0xf1;
        let reg = Register::A;
        let mut build = builder(value, reg, addr);

        let mut expected = TestCpuState::new_with_program(build.get_program());
        expected.mem_at(addr, value);

        op_run_success(&expected, &mut build, run);
    }

    #[test]
    fn success_wide() {
        let value = 0xabcd;
        let addr = 0xf1;
        let reg = WideRegister::CD;
        let mut build = builder_wide(value, reg, addr);

        let mut expected = TestCpuState::new_with_program(build.get_program());
        expected.mem_at_wide(addr, value);

        op_run_success(&expected, &mut build, run);
    }

    test_builder_size!(builder(0xAB, Register::C, 0x10), SIZE, std);
    test_builder_size!(builder_wide(0xABCD, WideRegister::CD, 0x10), SIZE, wide);

    test_invalid_register!(&[0x00, 0x00, 0x10], 0x00);

    test_invalid_memory!(
        builder(0xAB, Register::C, TEST_OP_MEM_SIZE),
        TEST_OP_MEM_SIZE
    );

    test_run_no_mem!();

    fn builder(value: u8, reg: Register, addr: u16) -> TestCpuState {
        let mut build = builder_opargs(reg.into(), addr);
        build.reg(reg, value);
        build
    }

    fn builder_wide(value: u16, reg: WideRegister, addr: u16) -> TestCpuState {
        let mut build = builder_opargs(reg.into(), addr);
        build.reg_wide(reg, value);
        build
    }

    fn builder_opargs(reg_addr: u8, addr: u16) -> TestCpuState {
        let (a_high, a_low) = high_and_low_value(addr);
        let opargs = [reg_addr, a_high, a_low];
        let mut build = TestCpuState::new_with_program(&opargs);
        build.mem(TEST_OP_MEM_SIZE);
        build
    }
}
