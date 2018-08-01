extern crate rand;
use rand::prelude::*;

pub fn private_key(p: u64) -> u64 {
    if p == 0 || p - 1 <= 1 {
        panic!("Unable to pick a private key greater than 1 and less than {}", p)
    }

    let mut rng = thread_rng();
    rng.gen_range(2, p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modular_pow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modular_pow(b_pub, a, p)
}


#[allow(dead_code)]
fn modular_pow_slow(base: u64, exponent: u64, modulus: u64) -> u64 {
    match modulus {
        0 => panic!("modulus should be more than 0"),
        1 => return 0,
        _ => ()
    }

    let mut c = 1;
    for _e in 1..=exponent {
        c = (c * base) % modulus;
    }

    c
}


// Right-to-left binary method
fn modular_pow(base: u64, exponent: u64, modulus: u64) -> u64 {
    match modulus {
        0 => panic!("modulus should be more than 0"),
        1 => return 0,
        _ => (),
    }

    let mut result = 1;
    let mut base = base % modulus;
    let mut exponent = exponent;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % modulus;
        }
        exponent >>= 1;
        base = (base * base) % modulus;
    }

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_private_key_zero() {
        private_key(0);
    }

    #[test]
    #[should_panic]
    fn test_private_key_one() {
        private_key(1);
    }

    #[test]
    #[should_panic]
    fn test_private_key_two() {
        private_key(2);
    }

    #[test]
    fn test_modular_pow_slow_base_exponent_zero() {
        assert_eq!(modular_pow_slow(0, 0, 2), 1);
    }

    #[test]
    fn test_modular_pow_slow_base_is_zero() {
        assert_eq!(modular_pow_slow(0, 2, 2), 0);
    }

    #[test]
    fn test_modular_pow_slow_exponent_is_zero() {
        assert_eq!(modular_pow_slow(1, 0, 2), 1);
    }

    #[test]
    #[should_panic(expected = "modulus should be more than 0")]
    fn test_modular_pow_slow_modulus_is_zero() {
        modular_pow_slow(1, 0, 0);
    }

    #[test]
    fn test_modular_pow_slow_modulus_is_one() {
        assert_eq!(modular_pow_slow(1, 0, 1), 0);
    }

    #[test]
    fn test_modular_pow_base_exponent_zero() {
        assert_eq!(modular_pow(0, 0, 2), 1);
    }

    #[test]
    fn test_modular_pow_base_is_zero() {
        assert_eq!(modular_pow(0, 2, 2), 0);
    }

    #[test]
    fn test_modular_pow_exponent_is_zero() {
        assert_eq!(modular_pow(1, 0, 2), 1);
    }

    #[test]
    #[should_panic(expected = "modulus should be more than 0")]
    fn test_modular_pow_modulus_is_zero() {
        modular_pow(1, 0, 0);
    }

    #[test]
    fn test_modular_pow_modulus_is_one() {
        assert_eq!(modular_pow(1, 0, 1), 0);
    }
}
