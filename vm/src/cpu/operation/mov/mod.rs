use crate::{
    cpu::{AnyRegister, Cpu, OpResult},
    memory::Device,
};

pub mod lit_reg;
pub mod lit_reg_wide;

pub mod reg_reg;

pub mod reg_mem;

pub mod mem_reg;

pub mod lit_mem;
pub mod lit_mem_wide;

pub mod reg_ptr_reg;

pub mod lit_off_reg;

fn mov_mem_reg(cpu: &mut Cpu, addr: u16) -> OpResult {
    match cpu.fetch_any_register()? {
        AnyRegister::Std(reg) => {
            let value = cpu.memory.get(addr)?;
            cpu.registers.set(reg, value);
        }
        AnyRegister::Wide(reg) => {
            let value = cpu.memory.get_wide(addr)?;
            cpu.registers.set_wide(reg, value);
        }
    }
    Ok(())
}
