use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, TimeZone};
use chrono_tz::Tz;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ParseError {
    #[error("invalid format: {0}")]
    InvalidFormat(#[from] chrono::ParseError),
    #[error("issue content does not include any commands")]
    Empty,
}

pub fn parse_issue(content: &str, timezone: Tz, year: i32) -> Result<DateTime<Tz>, ParseError> {
    content
        .split('\n')
        .map(|line| parse_line(line, timezone, year))
        .reduce(|acc, result| match (acc, result) {
            (Ok(_), Ok(result)) => Ok(result),
            (Ok(acc), Err(_)) => Ok(acc),
            (Err(_), result) => result,
        })
        .unwrap_or(Err(ParseError::Empty))
}

fn parse_line(line: &str, timezone: Tz, year: i32) -> Result<DateTime<Tz>, ParseError> {
    let mut tokens = line.split(' ');
    tokens.find(|&x| x == "/deadline");

    let naive_date_time = match (tokens.next(), tokens.next()) {
        (None, _) => return Err(ParseError::Empty),
        (Some(date), None) => parse_date(date, year)?.and_hms(0, 0, 0),
        (Some(date), Some(time)) => {
            let naive_date = parse_date(date, year)?;
            let naive_time = NaiveTime::parse_from_str(time, "%H:%M")?;
            NaiveDateTime::new(naive_date, naive_time)
        }
    };
    Ok(timezone.from_local_datetime(&naive_date_time).unwrap())
}

fn parse_date(date: &str, year: i32) -> Result<NaiveDate, ParseError> {
    let naive_date = match NaiveDate::parse_from_str(date, "%Y/%m/%d") {
        Ok(date) => date,
        Err(_) => NaiveDate::parse_from_str(&format!("{}/{}", year, date), "%Y/%m/%d")?,
    };
    Ok(naive_date)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_command() {
        assert_eq!(parse_issue("Blah", Tz::UTC, 2021), Err(ParseError::Empty));
    }

    #[test]
    fn lack_of_deadline() {
        assert_eq!(
            parse_issue("2021/12/01", Tz::UTC, 2021),
            Err(ParseError::Empty)
        );
    }

    #[test]
    fn yyyy_mm_dd_hh_mm() {
        let comment = "Homework\n/deadline 2021/12/01 09:12";
        assert_eq!(
            parse_issue(comment, Tz::UTC, 2021),
            Ok(Tz::UTC.ymd(2021, 12, 1).and_hms(9, 12, 0))
        );
    }

    #[test]
    fn yyyy_mm_dd() {
        let comment = "Homework\n/deadline 2021/12/01";
        assert_eq!(
            parse_issue(comment, Tz::UTC, 2021),
            Ok(Tz::UTC.ymd(2021, 12, 1).and_hms(0, 0, 0))
        );
    }

    #[test]
    fn mm_dd() {
        let comment = "Homework\n/deadline 12/01";
        assert_eq!(
            parse_issue(comment, Tz::UTC, 2021),
            Ok(Tz::UTC.ymd(2021, 12, 1).and_hms(0, 0, 0))
        );
    }

    #[test]
    fn line_not_starting_with_command() {
        let comment = "Homework /deadline 2021/12/1";
        assert_eq!(
            parse_issue(comment, Tz::UTC, 2021),
            Ok(Tz::UTC.ymd(2021, 12, 1).and_hms(0, 0, 0))
        );
    }

    #[test]
    fn multiple_commands_results_in_last_one() {
        let comment =
            "Homework /deadline 2021/12/01 09:12\nBlah blah\n/deadline 2021/11/1\n/deadline 2021/1/1";
        assert_eq!(
            parse_issue(comment, Tz::UTC, 2021),
            Ok(Tz::UTC.ymd(2021, 1, 1).and_hms(0, 0, 0))
        );
    }

    #[test]
    fn non_utc_timezone() {
        let comment = "Homework\n/deadline 2021/12/01 9:00";
        assert_eq!(
            parse_issue(comment, Tz::Asia__Tokyo, 2021),
            Ok(Tz::Asia__Tokyo.ymd(2021, 12, 1).and_hms(9, 0, 0))
        );
    }
}
