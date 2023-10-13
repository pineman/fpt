pub fn get_byte16<const INDEX: u8>(word: u16) -> u8 {
    ((word >> (8 * INDEX)) & 0xFF) as u8
}

pub fn test_bit8<const INDEX: u8>(word: u8) -> bool {
    let mask: u8 = 1 << INDEX;
    word & mask == mask
}

pub fn test_bit16<const INDEX: u8>(word: u16) -> bool {
    let mask: u16 = 1 << INDEX;
    word & mask == mask
}

pub fn set_bit16<const INDEX: u8>(word: u16, value: bool) -> u16 {
    word & !(1 << INDEX) | (u16::from(value) << INDEX)
}

pub fn set_byte16<const INDEX: u8>(word: u16, byte: u8) -> u16 {
    let mask = 0xFF << (INDEX * 8);
    let word = word & !mask;
    word | ((byte as u16) << (INDEX * 8))
}

pub fn word16(msb: u8, lsb: u8) -> u16 {
    ((msb as u16) << 8) | (lsb as u16)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_byte16() {
        assert_eq!(get_byte16::<1>(0xABDC), 0xAB);
    }

    #[test]
    fn test_test_bit16() {
        assert_eq!(test_bit16::<0>(0x1234), false);
        assert_eq!(test_bit16::<1>(0x1234), false);
        assert_eq!(test_bit16::<2>(0x1234), true);
    }

    #[test]
    fn test_set_bit16() {
        assert_eq!(set_bit16::<0>(0x0000, true), 0x0001);
        assert_eq!(set_bit16::<1>(0x0000, true), 0x0002);
        assert_eq!(set_bit16::<8>(0x0101, false), 0x0001);
    }

    #[test]
    fn test_set_byte16() {
        assert_eq!(set_byte16::<0>(0x0000, 0xAB), 0x00AB);
        assert_eq!(set_byte16::<1>(0x0000, 0xAB), 0xAB00);
        assert_eq!(set_byte16::<0>(0xABDC, 0xAB), 0xABAB);
        assert_eq!(set_byte16::<1>(0xABCD, 0xAB), 0xABCD);
    }
}
