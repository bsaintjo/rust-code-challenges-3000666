use std::fmt::Display;
use std::slice::SliceIndex;
use std::str::FromStr;

#[derive(Debug)]
enum RgbError {
    Unknown,
}

#[derive(Debug, PartialEq)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
} // TODO: design data structure

trait RgbChannels {
    fn r(&self) -> u8;

    fn g(&self) -> u8;

    fn b(&self) -> u8;
}

impl RgbChannels for Rgb {
    // TODO: implement trait
    fn r(&self) -> u8 {
        self.r
    }

    fn b(&self) -> u8 {
        self.b
    }

    fn g(&self) -> u8 {
        self.g
    }
}

impl FromStr for Rgb {
    // TODO: implement trait
    type Err = RgbError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with('#') {
            return Err(RgbError::Unknown);
        }
        let r = parse_subslice(s, 1..=2)?;
        let g = parse_subslice(s, 3..=4)?;
        let b = parse_subslice(s, 5..=6)?;

        Ok(Rgb { r, g, b })
    }
}

fn parse_subslice<I>(src: &str, slice: I) -> Result<u8, RgbError>
where
    I: SliceIndex<str, Output = str>,
{
    let x: &str = src.get(slice).ok_or(RgbError::Unknown)?;
    u8::from_str_radix(x, 16).map_err(|_| RgbError::Unknown)
}

impl Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r(), self.g(), self.b())
    }
}

fn main() {
    //
}

#[test]
fn every_color() {
    let colors = (0_u8..255).zip(0_u8..255).zip(0_u8..255);

    for ((r, g), b) in colors {
        let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);
        let color: Rgb = hex.parse().unwrap();
        assert_eq!(hex, format!("{}", color));
    }
}

#[test]
#[should_panic]
fn too_short() {
    let _: Rgb = "1234".parse().unwrap();
}

#[test]
#[should_panic]
fn not_a_hex_code() {
    let _: Rgb = "?".parse().unwrap();
}

#[test]
#[should_panic]
fn invalid_literals() {
    let _: Rgb = "?".parse().unwrap();
}

#[test]
#[should_panic]
fn no_leading_hash() {
    let _: Rgb = "aabbcc".parse().unwrap();
}

#[test]
#[should_panic]
fn out_of_bounds() {
    let _: Rgb = "00gg00".parse().unwrap();
}
