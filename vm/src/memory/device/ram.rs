use super::device_impl;

#[derive(Debug)]
pub struct RamArray([u8; 256 * 256]);

impl RamArray {
    pub fn new() -> Self {
        Self::default()
    }
}

device_impl!(RamArray, self, 0);

impl Default for RamArray {
    fn default() -> Self {
        Self([0; 256 * 256])
    }
}

#[derive(Debug)]
pub struct DynMem(Vec<u8>);

impl DynMem {
    pub fn new(size: usize) -> Self {
        Self(vec![0; size])
    }

    pub fn replace(&mut self, replace_vec: &[u8], start_index: usize) {
        let self_range = start_index..replace_vec.len() + start_index;
        self.0[self_range].copy_from_slice(replace_vec);
    }
}

device_impl!(DynMem, self, 0);

impl Default for DynMem {
    fn default() -> Self {
        Self(Vec::with_capacity(0xffff + 1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::{device::tests::device_tests, Device, DeviceError};

    #[test]
    fn ram_array_new_and_default_is_zeroed() {
        let expected = [0; 0xffff + 1];
        assert_eq!(expected, RamArray::new().0);
        assert_eq!(expected, RamArray::default().0);
    }

    #[test]
    fn ram_array_new_get() {
        let ram = RamArray::new();
        let actual = ram.get(0).expect("ram array access first value");
        assert_eq!(0, actual);
    }

    device_tests!(ram_array, RamArray::new);

    #[test]
    fn dyn_mem_new_is_zeroed() {
        let size = 8;
        let expected = vec![0; size];
        assert_eq!(expected, DynMem::new(size).0);
    }

    #[test]
    fn dyn_mem_new_get() {
        let ram = DynMem::new(8);
        assert_eq!(0, ram.get(0).expect("dyn mem access first value"));
    }

    #[test]
    fn dyn_mem_new_empty_get_error() {
        assert_dyn_mem_get_out_of_bounds(0);
    }

    #[test]
    fn dyn_mem_out_of_bounds_error() {
        assert_dyn_mem_get_out_of_bounds(5);
    }

    fn assert_dyn_mem_get_out_of_bounds(addr: u16) {
        let ram = DynMem::new(addr as usize);
        let err = ram.get(addr).expect_err("all addr out of bounds");
        assert!(matches!(err, DeviceError::OutOfBounds(_)));
        match err {
            DeviceError::OutOfBounds(actual) => assert_eq!(addr, actual),
            _ => unreachable!(),
        }
    }

    device_tests!(dyn_mem, || DynMem::new(8));
}
