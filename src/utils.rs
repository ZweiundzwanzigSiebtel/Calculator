#[derive(Debug)]
pub enum Base {
    Bin,
    Dec,
    Hex
}

impl Base {
    pub fn get_base_numeric(&self) -> i64 {
        match self {
            Base::Bin => 2,
            Base::Dec => 10,
            Base::Hex => 16,
        }
    }
}

//pub fn dec_to_base(dec: u64, base: Base) -> u64 {
//    match base {
//        Base::Bin => to_bin(dec),
//        err => panic!("error: {:?}", err),
//    }
//}

fn to_bin(mut dec: u64) -> u64 {
    let mut result = 0;
    let mut i = 0;
    while dec >= 1 {
        result += 10_u64.pow(i) * (dec%2);
        dec = dec/2;
        i += 1;
    }
    result
}

fn to_hex(mut dec: u64) -> u64 {
    let mut result = 0;
    let mut i = 0;
    while dec >= 1 {
        result += 10_u64.pow(i) * (dec%16);
        dec = dec/16;
        i += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_bin() {
        assert_eq!(0, to_bin(0));
        assert_eq!(1, to_bin(1));
        assert_eq!(10, to_bin(2));
        assert_eq!(11, to_bin(3));
        assert_eq!(100, to_bin(4));
        assert_eq!(101, to_bin(5));
        assert_eq!(110, to_bin(6));
        assert_eq!(111, to_bin(7));
        assert_eq!(1000, to_bin(8));
        assert_eq!(1001, to_bin(9));
        assert_eq!(1010, to_bin(10));
        assert_eq!(1011, to_bin(11));
        assert_eq!(1100, to_bin(12));
        assert_eq!(1101, to_bin(13));
        assert_eq!(1110, to_bin(14));
        assert_eq!(1111, to_bin(15));
    }

    #[test]
    fn test_to_hex() {

    }
}
