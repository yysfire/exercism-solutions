#![allow(unused_variables)]
pub fn find() -> Option<u32> {
    let a: u32;
    let b: u32;
    for a in 1..1000-1 {
        for b in 1..1000-a {
            let c: u32 = 1000 - a - b;
            if c <= 0 {
                continue;
            } else if a * a + b * b == c * c {
                println!("a: {}, b: {}, c: {}", a, b, c);
                return Some(a * b * c);
            }
        }
    }
    None
}
