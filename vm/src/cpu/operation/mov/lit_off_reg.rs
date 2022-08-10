use crate::cpu::{Cpu, OpResult};

use super::mov_mem_reg;

pub const CODE: u8 = 0x18;
pub const NAME: &str = "MOV_LIT_OFF_REG";
pub const SIZE: u8 = 5;

pub(in crate::cpu::operation) fn run(cpu: &mut Cpu) -> OpResult {
    let mut addr = cpu.fetch_wide()?;
    let from = cpu.fetch_register_wide()?;
    let offset = cpu.registers.get_wide(from);
    addr += offset;
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
        let offset = 0;
        let to = Register::H;

        let mut expected = TestCpuState::new();
        expected.reg(to, value);

        op_run_success(&expected, &mut builder(value, offset, to), run);
    }

    #[test]
    fn success_wide() {
        let value = 0xabcd;
        let offset = 0;
        let to = WideRegister::GH;

        let mut expected = TestCpuState::new();
        expected.reg_wide(to, value);

        op_run_success(&expected, &mut builder_wide(value, offset, to), run);
    }
    #[test]
    fn success_offset_std() {
        let value = 0xab;
        let offset = 1;
        let to = Register::H;

        let mut expected = TestCpuState::new();
        expected.reg(to, value);

        op_run_success(&expected, &mut builder(value, offset, to), run);
    }

    #[test]
    fn success_offset_wide() {
        let value = 0xabcd;
        let offset = 1;
        let to = WideRegister::GH;

        let mut expected = TestCpuState::new();
        expected.reg_wide(to, value);

        op_run_success(&expected, &mut builder_wide(value, offset, to), run);
    }

    test_builder_size!(builder(0xab, 0, Register::A), SIZE, std);
    test_builder_size!(builder_wide(0xabcd, 0, WideRegister::CD), SIZE, wide);

    test_invalid_register!(&[0x00, 0x01, 0x00, Register::A.into()], 0x00, first);
    test_invalid_register!(&[0x00, 0x01, WideRegister::GH.into(), 0x00], 0x00, second);

    test_invalid_memory!(builder_invalid_memory(TEST_OP_MEM_SIZE), TEST_OP_MEM_SIZE);

    test_run_no_mem!();

    fn builder(value: u8, offset: u16, to: Register) -> TestCpuState {
        let base_addr = 0x10;
        let from = WideRegister::EF;
        let mut build = builder_opargs(base_addr, offset, from, to.into());
        build.mem_at(base_addr + offset, value);
        build
    }

    fn builder_wide(value: u16, offset: u16, to: WideRegister) -> TestCpuState {
        let base_addr = 0x10;
        let from = WideRegister::EF;
        let mut build = builder_opargs(base_addr, offset, from, to.into());
        build.mem_at_wide(base_addr + offset, value);
        build
    }

    fn builder_invalid_memory(ptr: u16) -> TestCpuState {
        builder_opargs(ptr, 0, WideRegister::GH, Register::F.into())
    }

    fn builder_opargs(
        base_addr: u16,
        offset: u16,
        from: WideRegister,
        to_addr: u8,
    ) -> TestCpuState {
        let (high, low) = high_and_low_value(offset);
        let opargs = [high, low, from.into(), to_addr];
        let mut build = TestCpuState::new_with_program(&opargs);
        build.reg_wide(from, base_addr);
        build
    }
}
