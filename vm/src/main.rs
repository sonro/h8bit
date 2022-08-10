use h8bit_vm::{
    cpu::operation::*,
    cpu::Cpu,
    memory::{DynMem, MemoryMapper, RamArray},
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
    cpu.display_memory_at(0);
}

fn boot_rom() -> Vec<u8> {
    vec![nop::CODE, hlt::CODE]
}
