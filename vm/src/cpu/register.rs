use crate::util::{high_and_low_value, wide_value};
use std::fmt::{self, Write};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, FromRepr, IntoStaticStr};

#[derive(Debug, thiserror::Error)]
#[error("No register at address: {0:#04x}")]
pub struct InvalidRegister(u8);

#[derive(Debug, Clone, Copy, PartialEq, FromRepr, IntoStaticStr, PartialOrd, Ord, Eq, EnumIter)]
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
    MB,
}

#[derive(Debug, Clone, Copy, PartialEq, FromRepr, IntoStaticStr)]
#[repr(u8)]
pub enum WideRegister {
    AB = 0x12,
    CD = 0x34,
    EF = 0x56,
    GH = 0x78,
    PC = 0xF0,
    SP = 0xF1,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnyRegister {
    Std(Register),
    Wide(WideRegister),
}

#[derive(Debug, Default, PartialEq)]
pub struct RegisterState {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    g: u8,
    h: u8,
    mb: u8,
    pc: u16,
    sp: u16,
}

impl RegisterState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, reg: Register, value: u8) {
        match reg {
            Register::A => self.a = value,
            Register::B => self.b = value,
            Register::C => self.c = value,
            Register::D => self.d = value,
            Register::E => self.e = value,
            Register::F => self.f = value,
            Register::G => self.g = value,
            Register::H => self.h = value,
            Register::MB => self.mb = value,
        }
    }

    pub fn get(&self, reg: Register) -> u8 {
        match reg {
            Register::A => self.a,
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d,
            Register::E => self.e,
            Register::F => self.f,
            Register::G => self.g,
            Register::H => self.h,
            Register::MB => self.mb,
        }
    }

    pub fn set_wide(&mut self, wide_reg: WideRegister, value: u16) {
        match wide_reg {
            WideRegister::PC => self.pc = value,
            WideRegister::SP => self.sp = value,
            _ => {
                let (high_val, low_val) = high_and_low_value(value);
                let (high_reg, low_reg) = wide_reg.high_and_low();
                self.set(high_reg, high_val);
                self.set(low_reg, low_val);
            }
        }
    }

    pub fn get_wide(&self, wide_reg: WideRegister) -> u16 {
        match wide_reg {
            WideRegister::PC => self.pc,
            WideRegister::SP => self.sp,
            _ => {
                let (high_reg, low_reg) = wide_reg.high_and_low();
                let high_val = self.get(high_reg);
                let low_val = self.get(low_reg);
                wide_value(high_val, low_val)
            }
        }
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
            _ => panic!(""),
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
        let mut output = String::with_capacity(256);
        for reg in Register::iter() {
            writeln!(&mut output, "  {}:   {:#04x}", reg, self.get(reg)).expect("format register")
        }
        let wide_fmt = |wide_reg| format!("  {}:   {:#06x}", wide_reg, self.get_wide(wide_reg));
        output += &wide_fmt(WideRegister::PC);
        output += &wide_fmt(WideRegister::SP);
        output.fmt(f)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::cpu::Error;

    const NUM_REG: usize = 11;

    #[test]
    fn register_state_init() {
        let reg_state = RegisterState::new();
        let expected = RegisterState {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            g: 0,
            h: 0,
            mb: 0,
            pc: 0,
            sp: 0,
        };
        assert_eq!(expected, reg_state);
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
    fn any_register_from_u8() {
        let addr_reg = addr_name_reg_map();
        for (addr, _, expected_reg) in addr_reg {
            let actual = AnyRegister::try_from(addr).expect("valid address");
            assert_eq!(expected_reg, actual);
        }
    }

    #[test]
    fn any_register_from_u8_error() {
        let addr = 0x00;
        let err = AnyRegister::try_from(addr).expect_err("valid address");
        assert_invalid_register(err, addr);
    }

    #[test]
    fn register_from_u8() {
        let addr_reg = addr_name_reg_map();
        for (addr, _, reg) in addr_reg {
            if let AnyRegister::Std(reg) = reg {
                let actual = Register::try_from(addr).expect("valid address");
                assert_eq!(reg, actual);
            }
        }
    }

    #[test]
    fn register_from_u8_error() {
        let addr = 0x00;
        let err = Register::try_from(addr).expect_err("invalid address");
        assert_invalid_register(err, addr);
    }

    #[test]
    fn u8_from_register() {
        let addr_reg = addr_name_reg_map();
        for (expected_addr, _, reg) in addr_reg {
            if let AnyRegister::Std(reg) = reg {
                assert_eq!(expected_addr, u8::from(reg));
            }
        }
    }

    #[test]
    fn name_from_register() {
        let addr_reg = addr_name_reg_map();
        for (_, name, reg) in addr_reg {
            if let AnyRegister::Std(reg) = reg {
                assert_eq!(name, reg.to_string());
            }
        }
    }

    #[test]
    fn wide_register_from_u8() {
        let addr_reg = addr_name_reg_map();
        for (addr, _, reg) in addr_reg {
            if let AnyRegister::Wide(reg) = reg {
                let actual = WideRegister::try_from(addr).expect("valid address");
                assert_eq!(reg, actual);
            }
        }
    }

    #[test]
    fn wide_register_from_u8_error() {
        let addr = 0x00;
        let err = WideRegister::try_from(addr).expect_err("invalid address");
        assert_invalid_register(err, addr);
    }

    #[test]
    fn u8_from_wide_register() {
        let addr_reg = addr_name_reg_map();
        for (expected_addr, _, reg) in addr_reg {
            if let AnyRegister::Wide(reg) = reg {
                assert_eq!(expected_addr, u8::from(reg));
            }
        }
    }

    #[test]
    fn name_from_wide_register() {
        let addr_reg = addr_name_reg_map();
        for (_, name, reg) in addr_reg {
            if let AnyRegister::Wide(reg) = reg {
                assert_eq!(name, reg.to_string());
            }
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

    pub fn assert_cpu_error_is_invalid_register(err: Error, expected: u8) {
        match err {
            Error::InvalidRegister(err) => assert_invalid_register(err, expected),
            err => panic!("expected InvalidRegister, got {:?}", err),
        }
    }

    fn assert_invalid_register(err: InvalidRegister, expected: u8) {
        let InvalidRegister(actual) = err;
        assert_eq!(
            expected, actual,
            "\nexpected incorrect reg_addr: {:#04x}, got {:#04x}",
            expected, actual
        );
    }

    fn addr_name_reg_map() -> [(u8, &'static str, AnyRegister); NUM_REG + 4] {
        use AnyRegister::*;
        use Register::*;
        use WideRegister::*;
        [
            (1, "A", Std(A)),
            (2, "B", Std(B)),
            (3, "C", Std(C)),
            (4, "D", Std(D)),
            (5, "E", Std(E)),
            (6, "F", Std(F)),
            (7, "G", Std(G)),
            (8, "H", Std(H)),
            (9, "MB", Std(MB)),
            (0x12, "AB", Wide(AB)),
            (0x34, "CD", Wide(CD)),
            (0x56, "EF", Wide(EF)),
            (0x78, "GH", Wide(GH)),
            (0xF0, "PC", Wide(PC)),
            (0xF1, "SP", Wide(SP)),
        ]
    }
}
