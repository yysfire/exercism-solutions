pub fn reply(message: &str) -> &str {
    let message = message.trim();
    if message.is_empty() {
        return "Fine. Be that way!";
    }

    let is_all_letters_upper = is_all_letters_uppercase(message);

    if is_all_letters_upper && message.ends_with('?') {
        "Calm down, I know what I'm doing!"
    } else if is_all_letters_upper {
        "Whoa, chill out!"
    } else if message.ends_with('?') {
        "Sure."
    } else {
        "Whatever."
    }
}

// Checks if all letters in string are ASCII uppercase
fn is_all_letters_uppercase(msg: &str) -> bool {
    let mut flag = false;
    for c in msg.chars() {
        if c.is_ascii_uppercase() {
            flag |= true;
        } else if c.is_ascii_lowercase() {
            return false;
        }
    }
    flag
}
