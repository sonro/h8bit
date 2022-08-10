use super::mov_mem_reg;
use crate::cpu::{Cpu, OpResult};

pub const CODE: u8 = 0x14;
pub const NAME: &str = "MOV_MEM_REG";
pub const SIZE: u8 = 4;

pub(in crate::cpu::operation) fn run(cpu: &mut Cpu) -> OpResult {
    let addr = cpu.fetch_wide()?;
    mov_mem_reg(cpu, addr)
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

        let mut expected = TestCpuState::new();
        expected.reg(reg, value);

        op_run_success(&expected, &mut builder(value, addr, reg), run);
    }

    #[test]
    fn success_wide() {
        let value = 0xabcd;
        let addr = 0xf1;
        let reg = WideRegister::CD;

        let mut expected = TestCpuState::new();
        expected.reg_wide(reg, value);

        op_run_success(&expected, &mut builder_wide(value, addr, reg), run);
    }

    test_builder_size!(builder(0xAB, 0x10, Register::C), SIZE, std);
    test_builder_size!(builder_wide(0xABCD, 0x10, WideRegister::CD), SIZE, wide);

    test_invalid_memory!(builder_invalid_memory(TEST_OP_MEM_SIZE), TEST_OP_MEM_SIZE);

    test_invalid_register!(&[0x00, 0x10, 0x00], 0x00);

    test_run_no_mem!();

    fn builder(value: u8, addr: u16, reg: Register) -> TestCpuState {
        let mut build = builder_opargs(reg.into(), addr);
        build.mem_at(addr, value);
        build
    }

    fn builder_wide(value: u16, addr: u16, reg: WideRegister) -> TestCpuState {
        let mut build = builder_opargs(reg.into(), addr);
        build.mem_at_wide(addr, value);
        build
    }

    fn builder_invalid_memory(addr: u16) -> TestCpuState {
        let mut build = builder_opargs(Register::C.into(), addr);
        build.mem(TEST_OP_MEM_SIZE);
        build
    }

    fn builder_opargs(reg_addr: u8, addr: u16) -> TestCpuState {
        let (a_high, a_low) = high_and_low_value(addr);
        let opargs = [a_high, a_low, reg_addr];
        TestCpuState::new_with_program(&opargs)
    }
}
