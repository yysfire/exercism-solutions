/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    //use std::iter::once;

    cipher_str(plain)
        .enumerate()
        .flat_map(|(i, x)| {
            if i != 0 && i % 5 == 0 {
                //once(' ').chain(once(x)).take(2)
                vec![' ', x]
            } else {
                //once(x).chain(once(' ')).take(1)
                vec![x]
            }
        })
        .collect::<String>()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher_str(cipher).collect::<String>()
}

fn cipher_char(x: char) -> char {
    let letters = "abcdefghijklmnopqrstuvwxyz";
    let index = letters.chars().position(|l| l == x).unwrap();
    letters.chars().rev().nth(index).unwrap()
}

fn cipher_str<'a>(s: &'a str) -> impl Iterator<Item = char> + 'a {
    s.chars().filter(|x| x.is_ascii_alphanumeric()).map(|x| {
        if x.is_ascii_alphabetic() {
            cipher_char(x.to_ascii_lowercase())
        } else {
            x
        }
    })
}
