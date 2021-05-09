///Check if the string contains balanced brackets.
pub fn brackets_are_balanced(string: &str) -> bool {
    let mut brackets = vec![];

    string
        .chars()
        .filter(|&c| "()[]{}".contains(c))
        .map(|c| {
            if "([{".contains(c) {
                match c {
                    '(' => brackets.push(')'),
                    '[' => brackets.push(']'),
                    '{' => brackets.push('}'),
                    _ => (),
                };
                true
            } else {
                if let Some(b) = brackets.pop() {
                    b == c
                } else {
                    false
                }
            }
        })
        .all(|x| x)
        && brackets.len() == 0
}
