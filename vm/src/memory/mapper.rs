use super::device::Device;
use crate::memory::DeviceError;

#[derive(Default, Debug)]
pub struct MemoryMapper {
    regions: Vec<Region>,
    start: u16,
    end: u16,
}

impl MemoryMapper {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_device(&mut self, device: Box<dyn Device>, start: u16, end: u16) {
        match self.start() {
            // map starts at new device start
            Some(x) if start < x => self.start = start,
            None => self.start = start,
            _ => (),
        }
        match self.end() {
            // map ends at new device start
            Some(x) if end > x => self.end = end,
            None => self.end = end,
            _ => (),
        }
        // insert is expensive but saves on reversing vector when finding regions
        self.regions.insert(0, Region { device, start, end });
    }

    pub fn start(&self) -> Option<u16> {
        match self.regions.len() {
            0 => None,
            _ => Some(self.start),
        }
    }

    pub fn end(&self) -> Option<u16> {
        match self.regions.len() {
            0 => None,
            _ => Some(self.end),
        }
    }

    fn find_region(&self, addr: u16) -> Option<&Region> {
        self.regions
            .iter()
            .find(|region| is_addr_in_region(addr, region))
    }

    fn find_region_mut(&mut self, addr: u16) -> Option<&mut Region> {
        self.regions
            .iter_mut()
            .find(|region| is_addr_in_region(addr, region))
    }
}

impl Device for MemoryMapper {
    fn set(&mut self, addr: u16, data: u8) -> Result<(), DeviceError> {
        if let Some(region) = self.find_region_mut(addr) {
            region.device.set(addr - region.start, data)
        } else {
            Err(DeviceError::OutOfBounds(addr))
        }
    }

    fn get(&self, addr: u16) -> Result<u8, DeviceError> {
        if let Some(region) = self.find_region(addr) {
            region.device.get(addr - region.start)
        } else {
            Err(DeviceError::OutOfBounds(addr))
        }
    }

    fn set_wide(&mut self, addr: u16, data: u16) -> Result<(), DeviceError> {
        if let Some(region) = self.find_region_mut(addr) {
            region.device.set_wide(addr - region.start, data)
        } else {
            Err(DeviceError::OutOfBounds(addr))
        }
    }

    fn get_wide(&self, addr: u16) -> Result<u16, DeviceError> {
        if let Some(region) = self.find_region(addr) {
            region.device.get_wide(addr - region.start)
        } else {
            Err(DeviceError::OutOfBounds(addr))
        }
    }
}

#[derive(custom_debug::Debug)]
struct Region {
    #[debug(skip)]
    device: Box<dyn Device>,
    start: u16,
    end: u16,
}

fn is_addr_in_region(addr: u16, region: &Region) -> bool {
    addr >= region.start && addr <= region.end
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::device::tests::{
        assert_device_error_is_out_of_bounds, device_tests, TestDevice,
    };

    #[test]
    fn new_mapper_get_error() {
        let mapper = MemoryMapper::new();
        let addr = 0;
        let err = mapper.get(addr).expect_err("out of bounds error");
        assert_device_error_is_out_of_bounds(err, addr)
    }

    #[test]
    fn new_mapper_set_error() {
        let mut mapper = MemoryMapper::new();
        let addr = 0;
        let err = mapper.set(addr, 1).expect_err("out of bounds error");
        assert_device_error_is_out_of_bounds(err, addr)
    }

    #[test]
    fn new_mapper_start_none() {
        let mapper = MemoryMapper::new();
        assert_eq!(None, mapper.start());
    }

    #[test]
    fn new_mapper_end_none() {
        let mapper = MemoryMapper::new();
        assert_eq!(None, mapper.end());
    }

    #[test]
    fn mapper_simple_start() {
        assert_mapper_start_offset(0);
    }

    #[test]
    fn mapper_simple_end() {
        assert_mapper_end_offset(0);
    }

    #[test]
    fn mapper_offset_start() {
        assert_mapper_start_offset(5);
    }

    #[test]
    fn mapper_offset_end() {
        assert_mapper_end_offset(5);
    }

    #[test]
    fn mapper_simple_addr_get() {
        let addr = 1;
        let value = 1;
        let mapper = test_mapper_with_set_device_at(0, addr, value);
        let actual = mapper.get(addr).expect("valid address");
        assert_eq!(value, actual);
    }

    #[test]
    fn mapper_offset_addr_get() {
        let addr = 1;
        let value = 1;
        let offset = 1;
        let mapper = test_mapper_with_set_device_at(offset, addr, value);
        let actual = mapper.get(addr + offset).expect("valid address");
        assert_eq!(value, actual);
    }

    device_tests!(mapper_simple, || test_mapper_with_device_at(0));
    device_tests!(mapper_offset, || test_mapper_with_device_at(1));

    fn test_mapper_with_device_at(offset: u16) -> MemoryMapper {
        test_mapper_with_set_device_at(offset, 0, 0)
    }

    fn test_mapper_with_set_device_at(offset: u16, set_addr: u16, set_val: u8) -> MemoryMapper {
        let mut mapper = MemoryMapper::new();
        let mut device = test_device(TEST_DEVICE_SIZE);
        device.set(set_addr, set_val).unwrap();
        mapper.add_device(device, offset, TEST_DEVICE_SIZE - 1 + offset);
        mapper
    }

    const TEST_DEVICE_SIZE: u16 = 8;

    fn test_device(size: u16) -> Box<TestDevice> {
        Box::new(TestDevice::new(size))
    }

    fn assert_mapper_start_offset(offset: u16) {
        let mapper = test_mapper_with_device_at(offset);
        assert_eq!(offset, mapper.start().expect("mapper has start"));
    }

    fn assert_mapper_end_offset(offset: u16) {
        let mapper = test_mapper_with_device_at(offset);
        assert_eq!(
            TEST_DEVICE_SIZE + offset - 1,
            mapper.end().expect("mapper has end")
        );
    }
}
