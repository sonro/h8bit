use h8bit_vm::memory::{DynMem, MemoryMapper, RamArray};

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

    // run
}

fn boot_rom() -> Vec<u8> {
    vec![]
}
