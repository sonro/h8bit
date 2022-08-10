use super::*;
use crate::memory::TestDevice;

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

const TEST_DEVICE_SIZE: u16 = 0xffff;

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
