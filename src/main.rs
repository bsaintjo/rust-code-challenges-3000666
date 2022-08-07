mod run_length_encoding {
    use std::iter::repeat;

    pub fn encode(text: &str) -> String {
        let mut chars = text.chars();
        let mut count = 1;
        let mut init_ch = chars.next().unwrap();
        let mut acc = Vec::new();
        for ch in chars {
            if ch == init_ch {
                count += 1;
                if count == 9 {
                    acc.push((9, init_ch));
                    count = 0;
                }
            } else {
                acc.push((count, init_ch));
                init_ch = ch;
                count = 1;
            }
        }
        acc.push((count, init_ch));
        let mut encoded = String::new();
        acc.into_iter()
            .map(|(n, c)| format!("{n}{c}"))
            .for_each(|s| encoded.push_str(&s));
        encoded
    }

    pub fn decode(text: &str) -> String {
        let mut acc = String::new();
        let mut iter = text.chars();
        while let Some(count) = iter.next() {
            let count = count.to_digit(10).unwrap() as usize;
            let ch = iter.next().unwrap();
            repeat(ch).take(count).for_each(|c| acc.push(c));
        }
        acc
    }
}

fn main() {
    //
}

#[test]
fn abc() {
    use run_length_encoding::*;

    assert_eq!(encode("abc"), "1a1b1c");
}

#[test]
fn round_trip() {
    use run_length_encoding::*;

    let input = "LinkedIn";
    println!("{}", encode(input));
    assert_eq!(decode(&encode(input)), input);
}

#[test]
fn long_run() {
    use run_length_encoding::*;

    let input = "AAAAA AAAAAAAAAA AAAAAAAAAAAAAAAAAAAA";
    assert_eq!(encode(input), "5A1 9A1A1 9A9A2A");
}
