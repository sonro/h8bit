use self::register::RegisterState;
use self::{operation::Operation, register::InvalidRegister};
use crate::memory::{Device, DeviceError, MemoryMapper};
use std::fmt;
use std::fmt::Write;

pub use register::{AnyRegister, Register, WideRegister};

pub mod operation;
mod register;
#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Cpu {
    pc: u16,
    sp: u16,
    registers: RegisterState,
    memory: MemoryMapper,
}

impl Cpu {
    pub fn new(mem_map: MemoryMapper) -> Result<Self, Error> {
        let pc = mem_map.start().ok_or(Error::NoMemory)?;
        let sp = mem_map.end().ok_or(Error::NoMemory)?;
        Ok(Self {
            registers: RegisterState::new(),
            pc,
            sp,
            memory: mem_map,
        })
    }

    pub fn display_memory_at(&self, addr: u16) {
        let mut output = format!("{:#06x}:", addr);
        for a in addr..addr + 8 {
            let value = self.memory.get(a).unwrap();
            write!(output, " {:02x}", value).expect("display memory");
        }
        println!("{}", output);
    }

    pub fn run(&mut self) {
        loop {
            if let Err(_err) = self.step() {
                return;
            }
        }
    }

    pub fn step(&mut self) -> OpResult {
        let opcode = self.fetch()?;
        let operation = Operation::from(opcode);
        operation.execute(self)
    }

    fn fetch(&mut self) -> Result<u8, Error> {
        let addr = self.pc;
        let value = self.memory.get(addr)?;
        self.pc += 1;
        Ok(value)
    }

    fn fetch_wide(&mut self) -> Result<u16, Error> {
        let addr = self.pc;
        let value = self.memory.get_wide(addr)?;
        self.pc += 2;
        Ok(value)
    }

    fn fetch_register(&mut self) -> Result<Register, Error> {
        Ok(Register::try_from(self.fetch()?)?)
    }

    fn fetch_register_wide(&mut self) -> Result<WideRegister, Error> {
        Ok(WideRegister::try_from(self.fetch()?)?)
    }

    fn fetch_any_register(&mut self) -> Result<AnyRegister, Error> {
        Ok(AnyRegister::try_from(self.fetch()?)?)
    }
}

pub type OpResult = Result<(), Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("CPU halted")]
    Halt,
    #[error("device error: {0}")]
    Device(DeviceError),
    #[error("memory out of bounds: {0:#06x}")]
    OutOfBounds(u16),
    #[error(transparent)]
    InvalidRegister(#[from] InvalidRegister),
    #[error("no memory")]
    NoMemory,
    #[error("unknown internal error")]
    Unknown(Box<dyn std::error::Error>),
}

impl From<DeviceError> for Error {
    fn from(dev_err: DeviceError) -> Self {
        match dev_err {
            DeviceError::OutOfBounds(addr) => Self::OutOfBounds(addr),
            err => Self::Device(err),
        }
    }
}

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CPU\n pc: {:#06x}\n sp: {:#06x}\n{}",
            self.pc, self.sp, self.registers
        )
    }
}
