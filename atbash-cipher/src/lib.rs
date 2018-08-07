/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let plain = plain.to_ascii_lowercase();
    let iter = plain
        .chars()
        .filter(|x| x.is_ascii_alphanumeric())
        .map(|x| {
            if x.is_ascii_alphabetic() {
                cipher_char(x)
            } else {
                x
            }
        });
    let mut result = String::new();
    for (i, c) in iter.enumerate() {
        if i != 0 && i % 5 == 0 {
            result.push(' ');
        }
        result.push(c);
    }

    result
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter(|x| x.is_ascii_alphanumeric())
        .map(|x| {
            if x.is_ascii_alphabetic() {
                cipher_char(x)
            } else {
                x
            }
        })
        .collect::<String>()
}

fn cipher_char(x: char) -> char {
    let letters = "abcdefghijklmnopqrstuvwxyz";
    let index = letters.chars().position(|l| l == x).unwrap();
    letters.chars().rev().nth(index).unwrap()
}
