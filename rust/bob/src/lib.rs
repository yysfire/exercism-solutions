pub fn reply(message: &str) -> &str {
    match judge(message) {
        Message::Question => "Sure.",
        Message::Yell => "Whoa, chill out!",
        Message::YellQuestion => "Calm down, I know what I'm doing!",
        Message::Nothing => "Fine. Be that way!",
        Message::Anything => "Whatever.",
    }
}


enum Message {
    Question,
    Yell,
    YellQuestion,
    Nothing,
    Anything,
}


fn judge(message: &str) -> Message {
    let message = message.trim();
    if message.is_empty() {
        return Message::Nothing;
    }
    let is_upper = is_ascii_uppercase(message);
    if is_upper && message.ends_with('?') {
        return Message::YellQuestion;
    } else if is_upper {
        return Message::Yell;
    } else if message.ends_with('?') {
        return Message::Question;
    } else {
        return Message::Anything;
    }
}


// Checks if all characters in string are ASCII uppercase character
fn is_ascii_uppercase(msg: &str) -> bool {
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
