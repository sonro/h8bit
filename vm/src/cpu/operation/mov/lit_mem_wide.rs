use crate::cpu::{Cpu, Device, OpResult};

pub const CODE: u8 = 0x16;
pub const NAME: &str = "MOV_LIT_MEM_WIDE";
pub const SIZE: u8 = 5;

pub(in crate::cpu::operation) fn run(cpu: &mut Cpu) -> OpResult {
    let value = cpu.fetch_wide()?;
    let addr = cpu.fetch_wide()?;
    cpu.memory.set_wide(addr, value)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        cpu::{
            operation::tests::{
                op_run_success, test_builder_size, test_invalid_memory, test_run_no_mem,
                TEST_OP_MEM_SIZE,
            },
            tests::TestCpuState,
        },
        util::high_and_low_value,
    };

    #[test]
    fn success() {
        let literal = 0xabcd;
        let addr = 0xf1;
        let mut build = builder(literal, addr);

        let mut expected = TestCpuState::new_with_program(build.get_program());
        expected.mem_at_wide(addr, literal);

        op_run_success(&expected, &mut build, run);
    }

    test_builder_size!(builder(0xab, 0x10), SIZE);

    test_invalid_memory!(builder(0xab, TEST_OP_MEM_SIZE), TEST_OP_MEM_SIZE);

    test_run_no_mem!();

    fn builder(literal: u16, addr: u16) -> TestCpuState {
        let (a_high, a_low) = high_and_low_value(addr);
        let (l_high, l_low) = high_and_low_value(literal);
        let opargs = [l_high, l_low, a_high, a_low];
        TestCpuState::new_with_program(&opargs)
    }
}
