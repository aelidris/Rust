#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let expected_ciphered = original
        .chars()
        .map(|c| {
            if c.is_ascii_uppercase() {
                let mirrored = b'Z' - ((c as u8) - b'A');
                mirrored as char
            } else if c.is_ascii_lowercase() {
                let mirrored = b'z' - ((c as u8) - b'a');
                mirrored as char
            } else {
                c
            }
        })
        .collect::<String>();

    if expected_ciphered == ciphered {
        Ok(())
    } else {
        Err(CipherError { expected: expected_ciphered })
    }
}
