#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let expected = atbash_cipher(original);
    if ciphered == expected {
        Ok(())
    } else {
        Err(CipherError { expected })
    }
}

fn atbash_cipher(text: &str) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let (a, z) = if c.is_ascii_lowercase() {
                    (b'a', b'z')
                } else {
                    (b'A', b'Z')
                };
                (a + (z - c as u8)) as char
            } else {
                c
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{:?}", cipher("1Hello 2world!", "1Svool 2dliow!"));
        println!("{:?}", cipher("1Hello 2world!", "svool"));
    }
}
