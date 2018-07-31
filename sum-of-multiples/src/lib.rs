use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples = HashSet::new();
    for &f in factors {
        if f == 0 {
            continue;
        }

        for i in 1..=limit/f {
            if i * f < limit {
                multiples.insert(i * f);
            }
        }
    }

    let mut sum = 0;
    for m in multiples.iter() {
        sum += m;
    }

    sum
}
