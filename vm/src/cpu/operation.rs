use super::{Cpu, OpResult};
use std::fmt;
use strum_macros::{FromRepr, IntoStaticStr};

pub mod hlt;
pub mod mov;
pub mod nop;

#[derive(Debug, PartialEq, Copy, Clone, FromRepr, IntoStaticStr)]
#[repr(u8)]
pub enum Operation {
    /// No operation
    Nop = nop::CODE,

    /// Halt
    Hlt = hlt::CODE,

    /// Move literal to register
    MovLitReg = mov::lit_reg::CODE,

    /// Move wide literal to wide register
    MovLitRegWide = mov::lit_reg_wide::CODE,

    /// Move register to register
    MovRegReg = mov::reg_reg::CODE,

    /// Move register to memory
    MovRegMem = mov::reg_mem::CODE,

    /// Move memory to register
    MovMemReg = mov::mem_reg::CODE,

    /// Move literal to memory
    MovLitMem = mov::lit_mem::CODE,

    /// Move wide literal to memory
    MovLitMemWide = mov::lit_mem_wide::CODE,

    /// Move value at memory[wide register] to register
    MovRegPtrReg = mov::reg_ptr_reg::CODE,

    /// Move value at memory[wide literal + wide register] to register
    MovLitOffReg = mov::lit_off_reg::CODE,
}

impl Operation {
    pub fn execute(&self, cpu: &mut Cpu) -> OpResult {
        match self {
            Operation::Nop => nop::run(cpu),
            Operation::Hlt => hlt::run(cpu),
            Operation::MovLitReg => mov::lit_reg::run(cpu),
            Operation::MovLitRegWide => mov::lit_reg_wide::run(cpu),
            Operation::MovRegReg => mov::reg_reg::run(cpu),
            Operation::MovRegMem => mov::reg_mem::run(cpu),
            Operation::MovMemReg => mov::mem_reg::run(cpu),
            Operation::MovLitMem => mov::lit_mem::run(cpu),
            Operation::MovLitMemWide => mov::lit_mem_wide::run(cpu),
            Operation::MovRegPtrReg => mov::reg_ptr_reg::run(cpu),
            Operation::MovLitOffReg => mov::lit_off_reg::run(cpu),
        }
    }

    pub fn as_str(&self) -> &'static str {
        self.into()
    }

    pub fn addr(&self) -> u8 {
        (*self).into()
    }
}

impl From<u8> for Operation {
    fn from(code: u8) -> Self {
        if let Some(op) = Self::from_repr(code) {
            op
        } else {
            Self::Nop
        }
    }
}

impl From<Operation> for u8 {
    fn from(op: Operation) -> Self {
        op as Self
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name: &str = self.into();
        write!(f, "{}", name)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::cpu::{
        tests::{assert_cpu_state, TestCpuState},
        Error,
    };

    pub const TEST_OP_MEM_SIZE: u16 = 256;

    pub fn op_run_success(
        expected: &TestCpuState,
        build: &mut TestCpuState,
        run: fn(&mut Cpu) -> OpResult,
    ) {
        let mut cpu = build.build();
        run(&mut cpu).expect("success");
        assert_cpu_state(&cpu, expected);
    }

    pub fn op_run_error(build: &mut TestCpuState, run: fn(&mut Cpu) -> OpResult) -> Error {
        let mut cpu = build.build();
        run(&mut cpu).expect_err("error")
    }

    pub fn op_run_error_expected_state(
        expected: &TestCpuState,
        build: &mut TestCpuState,
        run: fn(&mut Cpu) -> OpResult,
    ) -> Error {
        let mut cpu = build.build();
        let err = run(&mut cpu).expect_err("error");
        assert_cpu_state(&cpu, expected);
        err
    }

    macro_rules! test_run_no_mem {
        () => {
            #[test]
            fn no_mem() {
                let mem = crate::memory::TestDevice::new(0);
                let mut cpu = crate::cpu::tests::create_cpu_with_memory(mem);
                let err = run(&mut cpu).expect_err("error");
                crate::cpu::tests::assert_cpu_error_is_out_of_bounds(err, 0);
            }
        };
    }
    pub(super) use test_run_no_mem;

    macro_rules! test_builder_size {
        (@RUN $builder:expr, $size:expr, $test_name:ident) => {
            #[test]
            fn $test_name() {
                let mut expected = TestCpuState::new();
                expected.pc($size as u16 - 1);
                op_run_success(&expected, &mut $builder, run);
            }
        };

        ($builder:expr, $size:expr) => {
            test_builder_size!(@RUN $builder, $size, size);
        };

        ($builder:expr, $size:expr, $width:ident) => {
            paste::paste! {
                test_builder_size!(@RUN $builder, $size, [<size_ $width>]);
            }
        };
    }
    pub(super) use test_builder_size;

    macro_rules! test_invalid_register {
        (@RUN $opargs:expr, $addr:expr, $test_name:ident) => {
            #[test]
            fn $test_name() {
                let mut build = TestCpuState::new_with_program($opargs);
                let err = crate::cpu::operation::tests::op_run_error(&mut build, run);
                crate::cpu::register::tests::assert_cpu_error_is_invalid_register(err, $addr);
            }
        };

        ($opargs:expr, $addr:expr) => {
            test_invalid_register!(@RUN $opargs, $addr, invalid_register);
        };

        ($opargs:expr, $addr:expr, $reg_name:ident) => {
            paste::paste! {
                test_invalid_register!(@RUN $opargs, $addr, [<invalid_register_ $reg_name>]);
            }
        };
    }
    pub(super) use test_invalid_register;

    macro_rules! test_invalid_memory {
        (@RUN $builder:expr, $addr:expr, $test_name:ident) => {
            #[test]
            fn $test_name() {
                let err = crate::cpu::operation::tests::op_run_error(&mut $builder, run);
                crate::cpu::tests::assert_cpu_error_is_out_of_bounds(err, $addr);
            }
        };

        ($builder:expr, $addr:expr) => {
            test_invalid_memory!(@RUN $builder, $addr, invalid_memory);
        };

        ($builder:expr, $addr:expr, $mem_name:ident) => {
            paste::paste! {
                test_invalid_memory!(@RUN $builder, $addr, [<invalid_memory_ $mem_name>]);
            }
        };
    }
    pub(super) use test_invalid_memory;
}
