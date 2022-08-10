pub mod ram;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("invalid device address: {0:#06x}")]
    OutOfBounds(u16),
    #[error("unkown error")]
    Other,
}

pub trait Device {
    fn set(&mut self, addr: u16, data: u8) -> Result<(), Error>;
    fn get(&self, addr: u16) -> Result<u8, Error>;
    fn set_wide(&mut self, addr: u16, data: u16) -> Result<(), Error>;
    fn get_wide(&self, addr: u16) -> Result<u16, Error>;
}

macro_rules! device_impl {
    (@wide) => {
        fn set_wide(&mut self, addr: u16, data: u16) -> Result<(), crate::memory::DeviceError> {
            let (high, low) = crate::util::high_and_low_value(data);
            self.set(addr, high)?;
            self.set(addr + 1, low)
        }

        fn get_wide(&self, addr: u16) -> Result<u16, crate::memory::DeviceError> {
            let high = self.get(addr)?;
            let low = self.get(addr + 1)?;
            Ok(crate::util::wide_value(high, low))
        }
    };

    ($type:ty, $self:ident, $prop:tt) => {
        impl crate::memory::Device for $type {
            fn set(&mut self, addr: u16, data: u8) -> Result<(), crate::memory::DeviceError> {
                let size = self.$prop.len();
                match addr {
                    a if a as usize >= size => Err(crate::memory::DeviceError::OutOfBounds(a)),
                    _ => Ok(self.$prop[addr as usize] = data),
                }
            }

            fn get(&self, addr: u16) -> Result<u8, crate::memory::DeviceError> {
                let size = self.$prop.len();
                match addr {
                    a if a as usize >= size => Err(crate::memory::DeviceError::OutOfBounds(a)),
                    _ => Ok(self.$prop[addr as usize]),
                }
            }

            device_impl!(@wide);
        }
    };

    ($type:ty, $self:ident) => {
        impl crate::memory::Device for $type {
            fn set(&mut self, addr: u16, data: u8) -> Result<(), crate::memory::DeviceError> {
                let size = self.len() as u16;
                match addr {
                    a if a >= size => Err(crate::memory::DeviceError::OutOfBounds(a)),
                    _ => Ok(self[addr as usize] = data),
                }
            }

            fn get(&self, addr: u16) -> Result<u8, crate::memory::DeviceError> {
                let size = self.len() as u16;
                match addr {
                    a if a >= size => Err(crate::memory::DeviceError::OutOfBounds(a)),
                    _ => Ok(self[addr as usize]),
                }
            }

            device_impl!(@wide);
        }
    };
}
pub(crate) use device_impl;

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn assert_device_error_is_out_of_bounds(err: Error, addr: u16) {
        assert!(matches!(err, Error::OutOfBounds(_)));
        match err {
            Error::OutOfBounds(actual) => assert_eq!(addr, actual),
            _ => unreachable!(),
        }
    }

    macro_rules! device_tests {
        ($name:ident, $constructor:expr) => {
            paste::paste! {
                #[test]
                fn [<$name _set_get>]() {
                    let mut dev = $constructor();
                    let (addr, value) = (1, 1);
                    dev.set(addr, value).expect("setting valid address");
                    let actual = dev.get(addr).expect("getting valid address");
                    assert_eq!(value, actual);
                }

                #[test]
                fn [<$name _set_get_wide>]() {
                    let mut dev = $constructor();
                    let (addr, value) = (1, 1);
                    dev.set_wide(addr, value).expect("setting valid address");
                    let actual = dev.get_wide(addr).expect("getting valid address");
                    assert_eq!(value, actual);
                }

                #[test]
                fn [<$name _set_wide_get_byte>]() {
                    let mut dev = $constructor();
                    let (addr, value) = (1, 0x0102);
                    dev.set_wide(addr, value).expect("setting valid address");
                    let (high, low) = crate::util::high_and_low_value(value);
                    let expect_msg = "getting valid address";
                    let actual_high = dev.get(addr).expect(expect_msg);
                    let actual_low = dev.get(addr + 1).expect(expect_msg);
                    assert_eq!(high, actual_high);
                    assert_eq!(low, actual_low);
                }

                #[test]
                fn [<$name _set_byte_get_wide>]() {
                    let mut dev = $constructor();
                    let (addr, value) = (1, 0x0102);
                    let (high, low) = crate::util::high_and_low_value(value);
                    let expect_msg = "setting valid address";
                    dev.set(addr, high).expect(expect_msg);
                    dev.set(addr + 1, low).expect(expect_msg);
                    let actual = dev.get_wide(addr).expect("getting valid address");
                    assert_eq!(value, actual);
                }
            }
        };
    }
    pub(crate) use device_tests;

    #[derive(Debug, Clone)]
    pub struct TestDevice(Vec<u8>);

    impl TestDevice {
        pub fn new(size: u16) -> Self {
            Self(vec![0; size as usize])
        }

        pub fn resize(&mut self, size: u16) {
            let mut new = Self::new(size);
            let data = match size {
                n if n > self.size() => &self.0,
                n => &self.0[0..n as usize],
            };
            new.write_slice(data);
        }

        pub fn size(&self) -> u16 {
            self.0.len() as u16
        }

        pub fn write_slice(&mut self, data: &[u8]) {
            self.0[0..data.len()].copy_from_slice(data);
        }

        pub fn get_slice(&self, start: usize, len: usize) -> &[u8] {
            &self.0[start..len + start]
        }

        pub fn end(&self) -> u16 {
            match self.size() {
                0 => 0,
                x => x - 1,
            }
        }
    }

    device_impl!(TestDevice, self, 0);
}
