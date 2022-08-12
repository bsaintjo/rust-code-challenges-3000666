mod vigenere {
    use std::char::from_digit;

    struct Vigenere {
        table: Vec<u8>,
    }

    impl Vigenere {
        fn new() -> Self {
            Vigenere {
                table: (b'A'..=b'Z').collect(),
            }
        }

        fn get_encrypted(&self, start: usize, idx: usize) -> u8 {
            self.table[(start + idx) % 26]
        }

        fn get_decrypted(&self, c: usize, k: usize) -> u8 {
            self.table[(c + 26 - k) % 26]
        }
    }

    fn key_to_same_length(text: &str, key: &str) -> String {
        key.chars().cycle().take(text.len()).collect()
    }

    pub fn encrypt(plaintext: &str, key: &str) -> String {
        let key = key_to_same_length(plaintext, key);
        let plaintext = plaintext.to_ascii_uppercase();
        let table = Vigenere::new();

        plaintext
            .bytes()
            .zip(key.bytes())
            .map(|(t, k)| table.get_encrypted((t - 65) as usize, (k - 65) as usize) as char)
            .collect()
    }

    pub fn decrypt(ciphertext: &str, key: &str) -> String {
        let ciphertext = ciphertext
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .collect::<String>();
        let key = key_to_same_length(&ciphertext, key);
        let table = Vigenere::new();

        ciphertext
            .bytes()
            .zip(key.bytes())
            .map(|(c, k)| {
                if c >= 65 {
                    table.get_decrypted((c - 65) as usize, (k - 65) as usize) as char
                } else {
                    c as char
                }
            })
            .collect()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_get() {
            let table = Vigenere::new();
            assert_eq!(table.get_encrypted(0, 3), b'D');
            assert_eq!(table.get_encrypted(1, 0), b'B');
            assert_eq!(table.get_encrypted(25, 1), b'A');
        }

        #[test]
        fn test_ascii() {
            assert_eq!(b'A', 65);
        }
    }
}

fn main() {
    let key = "WHYRUST";
    let ciphertext = "
    PVCDJG
    PAYCMY
    JR KUC
    ";
    let plaintext = vigenere::decrypt(ciphertext, key);

    println!("{}", plaintext);
}
