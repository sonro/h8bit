mod device;
mod mapper;

#[cfg(test)]
pub use device::tests::TestDevice;

pub use device::ram::*;
pub use device::{Device, Error as DeviceError};
pub use mapper::MemoryMapper;
