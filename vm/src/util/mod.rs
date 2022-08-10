/// Get upper and lower byte of a [`u16`](std::primitive::u16)
pub fn high_and_low_value(value: u16) -> (u8, u8) {
    let high_val = ((value & 0xff00) >> 8) as u8;
    let low_val = (value & 0x00ff) as u8;
    (high_val, low_val)
}

/// Combine two bytes to create a wide value
pub fn wide_value(high: u8, low: u8) -> u16 {
    ((high as u16) << 8) + low as u16
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn high_and_low_value_success() {
        let value = 0x0102;
        let (high, low) = high_and_low_value(value);
        assert_eq!(0x01, high);
        assert_eq!(0x02, low);
    }

    #[test]
    fn wide_value_success() {
        let (high, low) = (0x01, 0x02);
        let actual = wide_value(high, low);
        assert_eq!(0x0102, actual);
    }
}
