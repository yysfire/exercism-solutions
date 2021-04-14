use std::cmp::Ordering;

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

    let sum: u64 = (0..=num/2)
        .filter(|x| *x != 0)
        .filter(|x| num % x == 0)
        .sum();
    match sum.cmp(&num) {
        Ordering::Equal => Some(Classification::Perfect),
        Ordering::Greater => Some(Classification::Abundant),
        Ordering::Less => Some(Classification::Deficient),
    }
}
