pub fn build_proverb(list: Vec<&str>) -> String {
    let len = list.len();
    if len == 0 {
        return String::new();
    }

    let last = format!("And all for the want of a {}.", list[0]);
    let mut result = String::new();
    for i in 0..len - 1 {
        let s = format!("For want of a {} the {} was lost.\n", list[i], list[i+1]);
        result.push_str(&s);
    }
    result.push_str(&last);

    result
}
