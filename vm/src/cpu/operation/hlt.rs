use crate::cpu::{Cpu, Error, OpResult};

pub const CODE: u8 = 0xff;
pub const NAME: &str = "HLT";
pub const SIZE: u8 = 1;

pub(in crate::cpu::operation) fn run(_cpu: &mut Cpu) -> OpResult {
    Err(Error::Halt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::{
        operation::tests::{op_run_error, op_run_error_expected_state},
        tests::TestCpuState,
    };

    #[test]
    fn success() {
        let err = op_run_error(&mut TestCpuState::new(), run);
        assert!(matches!(err, Error::Halt));
    }

    #[test]
    fn size() {
        let mut expected = TestCpuState::new();
        expected.pc(SIZE as u16 - 1);
        op_run_error_expected_state(&expected, &mut TestCpuState::new(), run);
    }
}
