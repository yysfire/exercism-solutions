pub fn find() -> Option<u32> {
    for a in 1..1000-1 {
        for b in a..1000-a {
            let c = 1000 - a - b;
            if c >= a + b {
                continue;
            } else if a * a + b * b == c * c {
                println!("a: {}, b: {}, c: {}", a, b, c);
                return Some(a * b * c);
            }
        }
    }
    None
}
