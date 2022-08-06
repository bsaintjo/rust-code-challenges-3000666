use std::str::FromStr;

#[derive(Debug)]
enum IsbnError {
    InputTooLong,
    InputTooShort,
    FailedChecksum,
}

struct Isbn {
    raw: String,
    digits: Vec<u8>,
}

impl FromStr for Isbn {
    type Err = IsbnError; // TODO: replace with appropriate type

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let digits: Vec<u8> = s
            .chars()
            .filter(|x| x.is_numeric())
            .map(|x| x.to_digit(10).unwrap() as u8)
            .collect();
        let (&check_digit, digits) = digits.split_last().unwrap();
        let dlen = digits.len();
        if dlen > 12 {
            Err(IsbnError::InputTooLong)
        } else if dlen < 12 {
            Err(IsbnError::InputTooShort)
        } else if calculate_check_digit(digits) != check_digit {
            Err(IsbnError::FailedChecksum)
        } else {
            Ok(Isbn {
                raw: s.to_string(),
                digits: digits.to_vec(),
            })
        }
    }
}

impl std::fmt::Display for Isbn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}

// https://en.wikipedia.org/wiki/International_Standard_Book_Number#ISBN-13_check_digit_calculation
fn calculate_check_digit(digits: &[u8]) -> u8 {
    let result: u8 = digits
        .iter()
        .enumerate()
        .map(|(idx, &x)| if idx % 2 == 1 { x * 3 } else { x })
        .sum();
    (10u8 - (result % 10u8)) % 10
}

fn main() {
    let rust_in_action: Isbn = "978-3-16-148410-0".parse().unwrap();

    println!("Rust in Action's ISBN-13 ({})is valid!", rust_in_action);
}

#[test]
fn can_correctly_calculate_check_digits() {
    let cases = [
        ([9_u8, 7, 8, 1, 8, 6, 1, 9, 7, 8, 7, 6], 9_u8),
        ([9_u8, 7, 8, 3, 1, 6, 1, 4, 8, 4, 1, 0], 0_u8),
    ];

    for (case, check) in cases.iter() {
        let actual = calculate_check_digit(case);
        println!("{:?} -> {}?  {}", &case, check, actual);
        assert_eq!(calculate_check_digit(case), *check)
    }
}

#[test]
fn rust_in_action() {
    let _: Isbn = "978-3-16-148410-0".parse().unwrap();
}
