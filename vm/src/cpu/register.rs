use crate::util::{high_and_low_value, wide_value};
use std::fmt::{self, Write};
use strum_macros::{FromRepr, IntoStaticStr};

const NUM_REG: usize = 8;

#[derive(Debug, thiserror::Error)]
#[error("No register at address: {0:#04x}")]
pub struct InvalidRegister(u8);

#[derive(Debug, Clone, Copy, PartialEq, FromRepr, IntoStaticStr, PartialOrd, Ord, Eq)]
#[repr(u8)]
pub enum Register {
    A = 1,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

#[derive(Debug, Clone, Copy, PartialEq, FromRepr, IntoStaticStr)]
#[repr(u8)]
pub enum WideRegister {
    AB = 0x12,
    CD = 0x34,
    EF = 0x56,
    GH = 0x78,
}

#[derive(Debug, Clone, Copy)]
pub enum AnyRegister {
    Std(Register),
    Wide(WideRegister),
}

#[derive(Debug, Default)]
pub struct RegisterState([u8; NUM_REG]);

impl RegisterState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, reg: Register, value: u8) {
        let addr = reg as usize - 1;
        self.0[addr] = value;
    }

    pub fn get(&self, reg: Register) -> u8 {
        let addr = reg as usize - 1;
        self.0[addr]
    }

    pub fn set_wide(&mut self, wide_reg: WideRegister, value: u16) {
        let (high_val, low_val) = high_and_low_value(value);
        let (high_reg, low_reg) = wide_reg.high_and_low();
        self.set(high_reg, high_val);
        self.set(low_reg, low_val);
    }

    pub fn get_wide(&self, wide_reg: WideRegister) -> u16 {
        let (high_reg, low_reg) = wide_reg.high_and_low();
        let high_val = self.get(high_reg);
        let low_val = self.get(low_reg);
        wide_value(high_val, low_val)
    }
}

impl Register {
    pub fn as_str(&self) -> &'static str {
        self.into()
    }

    pub fn addr(&self) -> u8 {
        (*self).into()
    }
}

impl WideRegister {
    pub fn as_str(&self) -> &'static str {
        self.into()
    }

    pub fn addr(&self) -> u8 {
        (*self).into()
    }

    pub fn high_and_low(&self) -> (Register, Register) {
        match self {
            WideRegister::AB => (Register::A, Register::B),
            WideRegister::CD => (Register::C, Register::D),
            WideRegister::EF => (Register::E, Register::F),
            WideRegister::GH => (Register::G, Register::H),
        }
    }
}

impl TryFrom<u8> for Register {
    type Error = InvalidRegister;

    fn try_from(reg_addr: u8) -> Result<Self, Self::Error> {
        Self::from_repr(reg_addr).ok_or(InvalidRegister(reg_addr))
    }
}

impl TryFrom<u8> for WideRegister {
    type Error = InvalidRegister;

    fn try_from(reg_addr: u8) -> Result<Self, Self::Error> {
        Self::from_repr(reg_addr).ok_or(InvalidRegister(reg_addr))
    }
}

impl TryFrom<u8> for AnyRegister {
    type Error = InvalidRegister;

    fn try_from(reg_addr: u8) -> Result<Self, Self::Error> {
        match Register::try_from(reg_addr) {
            Ok(reg) => Ok(Self::from(reg)),
            Err(_) => WideRegister::try_from(reg_addr).map(Self::from),
        }
    }
}

impl From<Register> for AnyRegister {
    fn from(r: Register) -> Self {
        Self::Std(r)
    }
}

impl From<WideRegister> for AnyRegister {
    fn from(wr: WideRegister) -> Self {
        Self::Wide(wr)
    }
}

impl From<Register> for u8 {
    fn from(reg: Register) -> Self {
        reg as u8
    }
}

impl From<WideRegister> for u8 {
    fn from(reg: WideRegister) -> Self {
        reg as u8
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl fmt::Display for WideRegister {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl fmt::Display for RegisterState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::with_capacity(128);
        for i in 1..=NUM_REG {
            let reg_name = Register::try_from(i as u8).expect("valid address");
            let reg_value = self.0[i - 1];
            writeln!(&mut output, "  {}: {:#04x}", reg_name, reg_value).expect("format register");
        }
        output.fmt(f)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::cpu::Error;

    #[test]
    fn register_state_init() {
        let reg_state = RegisterState::new();
        let expected: [u8; 8] = [0; 8];
        for (i, &x) in expected.iter().enumerate() {
            assert_eq!(x, reg_state.0[i]);
            let reg = Register::try_from(i as u8 + 1).expect("valid address");
            assert_eq!(x, reg_state.get(reg));
        }
    }

    #[test]
    fn register_state_set_and_get() {
        let mut reg_state = RegisterState::new();
        let reg = Register::A;
        let value = 1;
        reg_state.set(reg, value);
        assert_eq!(value, reg_state.get(reg));
    }

    #[test]
    fn register_state_set_and_get_twice() {
        let mut reg_state = RegisterState::new();
        let reg = Register::A;
        let value_a = 1;
        let value_b = 2;
        reg_state.set(reg, value_a);
        assert_eq!(value_a, reg_state.get(reg));
        reg_state.set(reg, value_b);
        assert_eq!(value_b, reg_state.get(reg));
    }

    #[test]
    fn register_state_set_wide_and_get_wide() {
        let mut reg_state = RegisterState::new();
        let wide = WideRegister::AB;
        let value = 0x0101;
        reg_state.set_wide(wide, value);
        assert_eq!(value, reg_state.get_wide(wide));
    }

    #[test]
    fn register_state_set_wide_and_get_byte() {
        let mut reg_state = RegisterState::new();
        let wide = WideRegister::AB;
        let (high, low) = wide.high_and_low();
        let value = 0xAB12;
        let (high_val, low_val) = high_and_low_value(value);
        reg_state.set_wide(wide, value);
        assert_eq!(low_val, reg_state.get(low));
        assert_eq!(high_val, reg_state.get(high));
    }

    #[test]
    fn register_state_set_bytes_and_get_wide() {
        let mut reg_state = RegisterState::new();
        let wide = WideRegister::AB;
        let (high, low) = wide.high_and_low();
        let value = 0xAB12;
        let (high_val, low_val) = high_and_low_value(value);
        reg_state.set(high, high_val);
        reg_state.set(low, low_val);
        assert_eq!(value, reg_state.get_wide(wide));
    }

    #[test]
    fn register_from_u8() {
        let addr_reg = addr_name_reg_map();
        for (addr, _, expected_reg) in addr_reg {
            let actual = Register::try_from(addr).expect("valid address");
            assert_eq!(expected_reg, actual);
        }
    }

    #[test]
    fn register_from_u8_error() {
        let addr = NUM_REG as u8 + 1;
        let err = Register::try_from(addr).expect_err("invalid address");
        assert!(matches!(err, InvalidRegister(_)));
        let InvalidRegister(x) = err;
        assert_eq!(addr, x);
    }

    #[test]
    fn u8_from_register() {
        let addr_reg = addr_name_reg_map();
        for (expected_addr, _, reg) in addr_reg {
            assert_eq!(expected_addr, u8::from(reg));
        }
    }

    #[test]
    fn name_from_register() {
        let name_reg = addr_name_reg_map();
        for (_, expected_name, reg) in name_reg {
            assert_eq!(expected_name, reg.to_string());
        }
    }

    #[test]
    fn wide_register_from_u8() {
        let addr_reg = addr_name_wide_map();
        for (addr, _, expected_reg) in addr_reg {
            let expected = WideRegister::try_from(addr).expect("valid address");
            assert_eq!(expected_reg, expected);
        }
    }

    #[test]
    fn wide_register_from_u8_error() {
        let addr = 0x00;
        let err = WideRegister::try_from(addr).expect_err("invalid address");
        assert!(matches!(err, InvalidRegister(_)));
        let InvalidRegister(x) = err;
        assert_eq!(addr, x);
    }

    #[test]
    fn u8_from_wide_register() {
        let addr_reg = addr_name_wide_map();
        for (expected_addr, _, reg) in addr_reg {
            assert_eq!(expected_addr, u8::from(reg));
        }
    }

    #[test]
    fn name_from_wide_register() {
        let name_reg = addr_name_wide_map();
        for (_, expected_name, reg) in name_reg {
            assert_eq!(expected_name, reg.to_string());
        }
    }

    #[test]
    fn any_register_std_from_u8() {
        let addr = 0x01;
        let any = AnyRegister::try_from(addr).expect("valid address");
        assert!(matches!(any, AnyRegister::Std(Register::A)));
    }

    #[test]
    fn any_register_wide_from_u8() {
        let addr = 0x12;
        let any = AnyRegister::try_from(addr).expect("valid address");
        assert!(matches!(any, AnyRegister::Wide(WideRegister::AB)));
    }

    #[test]
    fn any_register_from_u8_error() {
        let addr = 0xAB;
        let err = AnyRegister::try_from(addr).expect_err("invalid address");
        assert!(matches!(err, InvalidRegister(_)));
        let InvalidRegister(x) = err;
        assert_eq!(addr, x);
    }

    pub fn assert_cpu_error_is_invalid_register(err: Error, expected: u8) {
        assert!(matches!(err, Error::InvalidRegister(_)));
        if let Error::InvalidRegister(InvalidRegister(actual)) = err {
            assert_eq!(
                expected, actual,
                "\nexpected incorrect reg_addr: {:#04x}, got {:#04x}",
                expected, actual
            );
        }
    }

    fn addr_name_reg_map() -> [(u8, &'static str, Register); NUM_REG] {
        use Register::*;
        [
            (1, "A", A),
            (2, "B", B),
            (3, "C", C),
            (4, "D", D),
            (5, "E", E),
            (6, "F", F),
            (7, "G", G),
            (8, "H", H),
        ]
    }

    fn addr_name_wide_map() -> [(u8, &'static str, WideRegister); NUM_REG / 2] {
        use WideRegister::*;
        [
            (0x12, "AB", AB),
            (0x34, "CD", CD),
            (0x56, "EF", EF),
            (0x78, "GH", GH),
        ]
    }
}
