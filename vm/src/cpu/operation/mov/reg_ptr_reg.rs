use super::mov_mem_reg;
use crate::cpu::{Cpu, OpResult};

pub const CODE: u8 = 0x17;
pub const NAME: &str = "MOV_REG_PTR_REG";
pub const SIZE: u8 = 3;

pub(in crate::cpu::operation) fn run(cpu: &mut Cpu) -> OpResult {
    let from = cpu.fetch_register_wide()?;
    let addr = cpu.registers.get_wide(from);
    mov_mem_reg(cpu, addr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::{
        operation::tests::{
            op_run_success, test_builder_size, test_invalid_memory, test_invalid_register,
            test_run_no_mem, TEST_OP_MEM_SIZE,
        },
        tests::TestCpuState,
        Register, WideRegister,
    };

    #[test]
    fn success_std() {
        let value = 0xab;
        let ptr = 0x10;
        let from = WideRegister::AB;
        let to = Register::C;

        let mut expected = TestCpuState::new();
        expected.reg(to, value);

        op_run_success(&expected, &mut builder(value, ptr, from, to), run);
    }

    #[test]
    fn success_wide() {
        let value = 0xabcd;
        let ptr = 0x10;
        let from = WideRegister::AB;
        let to = WideRegister::CD;

        let mut expected = TestCpuState::new();
        expected.reg_wide(to, value);

        op_run_success(&expected, &mut builder_wide(value, ptr, from, to), run);
    }

    test_builder_size!(
        builder(0xab, 0x10, WideRegister::AB, Register::C),
        SIZE,
        std
    );
    test_builder_size!(
        builder_wide(0xabcd, 0x10, WideRegister::AB, WideRegister::CD),
        SIZE,
        wide
    );

    test_invalid_register!(&[0x00, Register::A.into()], 0x00, first);
    test_invalid_register!(&[WideRegister::AB.into(), 0x00], 0x00, second);

    test_invalid_memory!(builder_invalid_memory(TEST_OP_MEM_SIZE), TEST_OP_MEM_SIZE);

    test_run_no_mem!();

    fn builder(value: u8, ptr: u16, from: WideRegister, to: Register) -> TestCpuState {
        let mut build = builder_opargs(ptr, from, to.into());
        build.mem_at(ptr, value);
        build
    }

    fn builder_wide(value: u16, ptr: u16, from: WideRegister, to: WideRegister) -> TestCpuState {
        let mut build = builder_opargs(ptr, from, to.into());
        build.mem_at_wide(ptr, value);
        build
    }

    fn builder_invalid_memory(ptr: u16) -> TestCpuState {
        builder_opargs(ptr, WideRegister::AB, Register::A.into())
    }

    fn builder_opargs(ptr: u16, from: WideRegister, to_addr: u8) -> TestCpuState {
        let opargs = [from.into(), to_addr];
        let mut build = TestCpuState::new_with_program(&opargs);
        build.reg_wide(from, ptr);
        build
    }
}
