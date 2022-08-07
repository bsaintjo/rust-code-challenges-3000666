use chrono::NaiveDate;

enum Parser {
    Year(i32),
    Month(u32),
    Day(u32),
    Sep,
}

fn parse_ymd(src: &str) -> Option<Vec<Parser>> {
    let mut acc = Vec::new();
    let x = parse_year(src)
        .and_then(|(p, src)| {
            acc.push(p);
            parse_sep(src)
        })
        .and_then(|(_, src)| parse_month(src))
        .and_then(|(p, src)| {
            acc.push(p);
            parse_sep(src)
        })
        .and_then(|(_, src)| parse_day(src))
        .map(|(p, _)| {
            acc.push(p);
            acc
        });
    x
}

fn parse_dmy(src: &str) -> Option<Vec<Parser>> {
    let mut acc = Vec::new();
    let x = parse_day(src)
        .and_then(|(p, src)| {
            acc.push(p);
            parse_sep(src)
        })
        .and_then(|(_, src)| parse_month(src))
        .and_then(|(p, src)| {
            acc.push(p);
            parse_sep(src)
        })
        .and_then(|(_, src)| parse_year(src))
        .map(|(p, _)| {
            acc.push(p);
            acc
        });
    x
}

fn parse_mdy(src: &str) -> Option<Vec<Parser>> {
    let mut acc = Vec::new();
    let x = parse_month(src)
        .and_then(|(p, src)| {
            acc.push(p);
            parse_sep(src)
        })
        .and_then(|(_, src)| parse_day(src))
        .and_then(|(p, src)| {
            acc.push(p);
            parse_sep(src)
        })
        .and_then(|(_, src)| parse_year(src))
        .map(|(p, _)| {
            acc.push(p);
            acc
        });
    x
}

fn parse_year(src: &str) -> Option<(Parser, &str)> {
    src.get(..4)
        .and_then(|s| s.parse::<i32>().ok())
        .map(|y| (Parser::Year(y), &src[4..]))
}

fn parse_sep(src: &str) -> Option<(Parser, &str)> {
    src.chars().next().and_then(|s| match s {
        ' ' | '/' | '.' | '-' => Some((Parser::Sep, &src[1..])),
        _ => None,
    })
}

fn parse_abbr_month_three(src: &str) -> Option<(Parser, &str)> {
    src.get(..3)
        .and_then(|month| match month {
            "Jan" => Some(Parser::Month(1)),
            "Feb" => Some(Parser::Month(2)),
            "Mar" => Some(Parser::Month(3)),
            "Apr" => Some(Parser::Month(4)),
            "May" => Some(Parser::Month(5)),
            "Aug" => Some(Parser::Month(8)),
            "Oct" => Some(Parser::Month(10)),
            "Nov" => Some(Parser::Month(11)),
            "Dec" => Some(Parser::Month(12)),
            _ => None,
        })
        .map(|p| (p, &src[3..]))
}

fn parse_abbr_month_four(src: &str) -> Option<(Parser, &str)> {
    src.get(..4)
        .and_then(|month| match month {
            "June" => Some(Parser::Month(6)),
            "July" => Some(Parser::Month(7)),
            "Sept" => Some(Parser::Month(9)),
            _ => None,
        })
        .map(|p| (p, &src[4..]))
}

fn parse_abbr_month(src: &str) -> Option<(Parser, &str)> {
    parse_abbr_month_three(src).or_else(|| parse_abbr_month_four(src))
}

fn parse_num_month(src: &str) -> Option<(Parser, &str)> {
    src.get(..2)
        .and_then(|s| s.parse::<u32>().ok())
        .map(|y| (Parser::Month(y), &src[2..]))
}

fn parse_month(src: &str) -> Option<(Parser, &str)> {
    parse_abbr_month(src).or_else(|| parse_num_month(src))
}

fn parse_day(src: &str) -> Option<(Parser, &str)> {
    src.get(..2)
        .and_then(|s| s.parse::<u32>().ok())
        .map(|y| (Parser::Day(y), &src[2..]))
}

/// Parses a string that represents a date. When a date
/// is unable to be determined, return `None`.
fn flexible_date_parse(text: &str) -> Option<NaiveDate> {
    use Parser::*;

    parse_dmy(text)
        .or_else(|| parse_mdy(text))
        .or_else(|| parse_ymd(text))
        .and_then(|pr| match (&pr[0], &pr[1], &pr[2]) {
            (&Year(y), &Month(m), &Day(d)) => Some(NaiveDate::from_ymd(y, m, d)),
            (&Day(d), &Month(m), &Year(y)) => Some(NaiveDate::from_ymd(y, m, d)),
            (&Month(m), &Day(d), &Year(y)) => Some(NaiveDate::from_ymd(y, m, d)),
            _ => None,
        })
}

fn main() {
    let dates = [
        "2010-12-11",
        "1999/Mar/02",
        "01.Mar.2021",
        "Mar.05.2021",
        "not a date",
    ];

    for d in dates.iter() {
        println!("{} -> {:?}", d, flexible_date_parse(d));
    }
}

#[test]
fn ymd_hyphen() {
    assert_eq!(
        flexible_date_parse("2010-12-11"),
        Some(NaiveDate::from_ymd(2010, 12, 11))
    )
}

#[test]
fn ymd_slash() {
    assert_eq!(
        flexible_date_parse("1999/Mar/02"),
        Some(NaiveDate::from_ymd(1999, 3, 2))
    )
}

#[test]
fn dmy_dot() {
    assert_eq!(
        flexible_date_parse("01.Mar.2021"),
        Some(NaiveDate::from_ymd(2021, 3, 1))
    )
}

#[test]
fn mdy_dot() {
    assert_eq!(
        flexible_date_parse("Apr.05.2021"),
        Some(NaiveDate::from_ymd(2021, 4, 5))
    )
}

#[test]
fn invalid() {
    assert_eq!(flexible_date_parse("not a date"), None)
}
