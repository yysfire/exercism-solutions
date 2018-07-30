pub fn nth(n: u32) -> Option<u32> {
    if n == 0 {
        return None;
    }
    let mut count = 0;
    for num in 2..=u32::max_value() {
        if is_prime(num) {
            count += 1;
            if count == n {
                return Some(num);
            }
        }
    }
    None
}

pub fn is_prime(num: u32) -> bool {
    if num == 0 || num == 1 {
        return false;
    } else if num == 2 {
        return true;
    }
    for i in 2..num {
        if num % i == 0 {
            return false;
        }
        if i.pow(2) > num {
            break;
        }
    }
    true
}
