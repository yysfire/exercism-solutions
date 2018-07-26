pub fn is_armstrong_number(num: u32) -> bool {
    is_armstrong_number_for_base(num, 10)
}


pub fn is_armstrong_number_for_base(num: u32, base: u32) -> bool {
    let mut numstring = match base {
        2 => format!("{:b}", num),
        8 => format!("{:o}", num),
        10 => format!("{}", num),
        16 => format!("{:x}", num),
        _ => panic!("Invalid base: {}", base),
    };

    let len = numstring.len() as u32;

    let mut sum: u32 = 0;
    loop {
        let c = match numstring.pop() {
            Some(c) => c,
            None => break,
        };
        let d = match c.to_digit(base) {
            Some(d) => d,
            None => panic!("It is impossible!"),
        };
        sum += d.pow(len);
    }

    if sum == num {
        true
    } else {
        false
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_invalid_base() {
        assert!(is_armstrong_number_for_base(100, 4))
    }

    #[test]
    fn test_one_digit_binary_armstrong_number() {
        assert!(is_armstrong_number_for_base(0b0001, 2))
    }

    #[test]
    fn test_two_digits_binary_non_armstrong_number() {
        assert!(!is_armstrong_number_for_base(0b11, 2))
    }

    #[test]
    fn test_two_digits_octal_armstrong_number() {
        assert!(is_armstrong_number_for_base(0o64, 8))
    }

    #[test]
    fn test_three_digits_octal_armstrong_number() {
        assert!(is_armstrong_number_for_base(0o463, 8))
    }

    #[test]
    fn test_one_digit_hex_armstrong_number() {
        assert!(is_armstrong_number_for_base(0x0B, 16))
    }

    #[test]
    fn test_no_two_digits_hex_armstrong_number() {
        assert!(!is_armstrong_number_for_base(0x34, 16))
    }

    #[test]
    fn test_three_digits_hex_armstrong_number() {
        assert!(is_armstrong_number_for_base(0x060B, 16))
    }
}
