use super::*;
use crate::{memory::TestDevice, util::high_and_low_value};
use std::collections::BTreeMap;

#[test]
fn new_cpu_error() {
    let mapper = MemoryMapper::new();
    let err = Cpu::new(mapper).expect_err("invalid cpu");
    assert!(matches!(err, Error::NoMemory));
}

#[test]
fn step_halt_program() {
    let program = [0xFF];
    let mut cpu = create_cpu_with_boot(&program);
    let err = cpu.step().expect_err("halting error");
    assert!(matches!(err, Error::Halt));
}

#[test]
fn step_nop_halt_program() {
    let program = [0x00, 0xFF];
    let mut cpu = create_cpu_with_boot(&program);
    cpu.step().expect("NOP doesn't error");
    let err = cpu.step().expect_err("halting error");
    assert!(matches!(err, Error::Halt));
}

#[test]
fn fetch_no_mem() {
    let mut cpu = create_cpu_with_boot_only(&[]);
    let err = cpu.fetch().expect_err("out of bounds error");
    assert_cpu_error_is_out_of_bounds(err, 0);
}

#[test]
fn fetch_wide_no_mem() {
    let mut cpu = create_cpu_with_boot_only(&[]);
    let err = cpu.fetch_wide().expect_err("out of bounds error");
    assert_cpu_error_is_out_of_bounds(err, 0);
}

#[test]
fn run_no_program() {
    let mut cpu = create_cpu_with_boot(&[]);
    cpu.run();
}

pub fn assert_cpu_error_is_out_of_bounds(err: Error, expected: u16) {
    assert!(matches!(err, Error::OutOfBounds(_)));
    match err {
        Error::OutOfBounds(actual) => assert_eq!(
            expected, actual,
            "\nexpected incorrect addr: {:#06x}, got {:#06x}",
            expected, actual
        ),
        _ => unreachable!(),
    }
}

pub fn create_cpu_with_memory(mem: TestDevice) -> Cpu {
    let mut mapper = MemoryMapper::new();
    let end = mem.end();
    let device = Box::new(mem);
    mapper.add_device(device, 0, end);
    Cpu::new(mapper).expect("valid CPU")
}

#[derive(Debug, Default)]
pub struct TestCpuState {
    program_size: usize,
    pc: Option<u16>,
    sp: Option<u16>,
    registers: Option<TestRegState>,
    memory: Option<TestDevice>,
}

impl TestCpuState {
    const MEM_SIZE: u16 = 256;

    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_program(program: &[u8]) -> Self {
        let mut state = Self::default();
        state.program(program);
        state
    }

    pub fn pc(&mut self, pc: u16) -> &mut Self {
        self.pc = Some(pc);
        self
    }

    pub fn sp(&mut self, sp: u16) -> &mut Self {
        self.sp = Some(sp);
        self
    }

    pub fn reg(&mut self, reg: Register, value: u8) -> &mut Self {
        if self.registers.is_none() {
            self.registers = Some(TestRegState::default());
        }
        if let Some(ref mut registers) = self.registers {
            registers.0.insert(reg, value);
        }
        self
    }

    pub fn reg_wide(&mut self, reg: WideRegister, value: u16) -> &mut Self {
        let (high_val, low_val) = high_and_low_value(value);
        let (high_reg, low_reg) = reg.high_and_low();
        self.reg(high_reg, high_val);
        self.reg(low_reg, low_val)
    }

    pub fn mem(&mut self, size: u16) -> &mut Self {
        match self.memory {
            Some(ref mut mem) => mem.resize(size),
            None => self.memory = Some(TestDevice::new(size)),
        }
        self
    }

    pub fn mem_at(&mut self, addr: u16, value: u8) -> &mut Self {
        match self.memory {
            // init memory then call this method again
            None => self.mem(Self::MEM_SIZE).mem_at(addr, value),
            Some(ref mut mem) => {
                mem.set(addr, value).expect("valid address");
                self
            }
        }
    }

    pub fn mem_at_wide(&mut self, addr: u16, value: u16) -> &mut Self {
        let (high, low) = high_and_low_value(value);
        self.mem_at(addr, high);
        self.mem_at(addr + 1, low)
    }

    pub fn program(&mut self, program: &[u8]) -> &mut Self {
        match self.memory {
            // init memory then call this method again
            None => self.mem(Self::MEM_SIZE).program(program),
            Some(ref mut mem) => {
                mem.write_slice(program);
                self.program_size = program.len();
                self
            }
        }
    }

    pub fn get_program(&self) -> &[u8] {
        match self.memory {
            // init memory then call this method again
            None => &[],
            Some(ref mem) => mem.get_slice(0, self.program_size),
        }
    }

    pub fn build(&mut self) -> Cpu {
        let mem = match &self.memory {
            Some(mem) => mem.clone(),
            None => TestDevice::new(Self::MEM_SIZE),
        };
        let mut cpu = create_cpu_with_memory(mem);
        if let Some(ref mut registers) = self.registers {
            for (&reg, &value) in &registers.0 {
                cpu.registers.set(reg, value);
            }
        }
        cpu
    }
}

#[derive(Debug, Default)]
pub struct TestRegState(BTreeMap<Register, u8>);

pub fn assert_cpu_state(cpu: &Cpu, expected: &TestCpuState) {
    assert_pc(cpu, expected.pc);
    assert_sp(cpu, expected.sp);
    assert_all_registers(cpu, expected.registers.as_ref());
    assert_mem(cpu, expected.memory.as_ref());
}

const TEST_DEVICE_SIZE: u16 = 0xfffe;

fn create_bootstraped_cpu(program: &[u8], mem_size: u16) -> Cpu {
    let mut device = TestDevice::new(mem_size);
    device.write_slice(program);
    create_cpu_with_memory(device)
}

fn create_cpu_with_boot(program: &[u8]) -> Cpu {
    create_bootstraped_cpu(program, TEST_DEVICE_SIZE)
}

fn create_cpu_with_boot_only(program: &[u8]) -> Cpu {
    create_bootstraped_cpu(program, program.len() as u16)
}

fn assert_pc(cpu: &Cpu, pc: Option<u16>) {
    if let Some(pc) = pc {
        let actual = cpu.registers.get_wide(PC);
        assert_eq!(
            pc,
            actual,
            "{}",
            assertion_msg_wide("Program Counter", pc, actual)
        );
    }
}

fn assert_sp(cpu: &Cpu, sp: Option<u16>) {
    if let Some(sp) = sp {
        let actual = cpu.registers.get_wide(SP);
        assert_eq!(
            sp,
            actual,
            "{}",
            assertion_msg_wide("Stack Pointer", sp, actual)
        );
    }
}

fn assert_all_registers(cpu: &Cpu, registers: Option<&TestRegState>) {
    if let Some(registers) = registers {
        for (&reg, &expected) in registers.0.iter() {
            assert_register(cpu, reg, expected);
        }
    }
}

fn assert_mem(cpu: &Cpu, mem: Option<&TestDevice>) {
    if let Some(mem) = mem {
        for i in 0..mem.size() {
            let actual = cpu.memory.get(i).expect("valid address");
            let expected = mem.get(i).expect("valid address");
            let title = format!("Memory: {:#06x}", i);
            assert_eq!(
                expected,
                actual,
                "{}",
                assertion_msg(&title, expected, actual)
            );
        }
    }
}

fn assert_register(cpu: &Cpu, reg: Register, expected: u8) {
    let actual = cpu.registers.get(reg);
    let title = format!("Register {}", reg.as_str());
    assert_eq!(
        expected,
        actual,
        "{}",
        assertion_msg(&title, expected, actual)
    );
}

fn assertion_msg(title: &str, expected: u8, actual: u8) -> String {
    format!(
        "\n{}\n{:>12}: {:#04x}\n{:>12}: {:#04x}\n",
        title, "expected", expected, "actual", actual
    )
}

fn assertion_msg_wide(title: &str, expected: u16, actual: u16) -> String {
    format!(
        "\n{}\n{:>12}: {:#06x}\n{:>12}: {:#06x}\n",
        title, "expected", expected, "actual", actual
    )
}
