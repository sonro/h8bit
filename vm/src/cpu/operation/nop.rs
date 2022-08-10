use crate::cpu::{Cpu, OpResult};

pub const CODE: u8 = 0x00;
pub const NAME: &str = "NOP";
pub const SIZE: u8 = 1;

pub(in crate::cpu::operation) fn run(_cpu: &mut Cpu) -> OpResult {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::{
        operation::tests::{op_run_success, test_builder_size},
        tests::TestCpuState,
    };

    #[test]
    fn success() {
        op_run_success(&TestCpuState::new(), &mut TestCpuState::new(), run)
    }

    test_builder_size!(TestCpuState::new(), SIZE);
}
