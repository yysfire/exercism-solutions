use std::cmp::Ordering;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }
    if num == 1 {
        return Some(Classification::Deficient);
    }

    let mut factors = HashSet::new();
    for i in 1..=(num as f64).sqrt() as u64 {
        if num % i == 0 {
            factors.insert(i);
            let m = num / i;
            if m != num {
                factors.insert(m);
            }
        }
    }
    let sum: u64 = factors.iter().sum();
    match sum.cmp(&num) {
        Ordering::Equal => Some(Classification::Perfect),
        Ordering::Greater => Some(Classification::Abundant),
        Ordering::Less => Some(Classification::Deficient),
    }
}
