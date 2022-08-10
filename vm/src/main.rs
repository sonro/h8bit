use h8bit_vm::{
    cpu::Cpu,
    cpu::{operation::*, Register, WideRegister},
    memory::{DynMem, MemoryMapper, RamArray},
    util::high_and_low_value,
};

fn main() {
    // create memory
    let mut mem_map = MemoryMapper::new();
    let ram = Box::new(RamArray::new());
    mem_map.add_device(ram, 0, 0xfffd);

    // load boot rom
    let mut boot_mem = Box::new(DynMem::new(0xff + 1));
    boot_mem.replace(&boot_rom(), 0);
    mem_map.add_device(boot_mem, 0, 0xff);

    // create cpu
    let mut cpu = Cpu::new(mem_map).expect("valid CPU");
    println!("{}", cpu);
    cpu.display_memory_at(0);

    // run
    cpu.run();
    println!("{}", cpu);
    cpu.display_memory_at(0x01f0);
}

fn boot_rom() -> Vec<u8> {
    let addr = 0x01f0;
    let (addr_high, addr_low) = high_and_low_value(addr);
    [
        vec![
            mov::lit_reg_wide::CODE,
            addr_high,
            addr_low,
            WideRegister::CD.into(),
        ],
        vec![mov::lit_mem::CODE, 0xab, addr_high, addr_low],
        vec![
            mov::reg_ptr_reg::CODE,
            WideRegister::CD.into(),
            Register::A.into(),
        ],
        vec![hlt::CODE],
    ]
    .into_iter()
    .flatten()
    .collect()
}
