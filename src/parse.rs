use chrono::{DateTime, NaiveDate, TimeZone};
use chrono_tz::Tz;

pub fn parse_issue(content: String, timezone: Tz) -> Option<DateTime<Tz>> {
    //     let local = NaiveDate::from_ymd(2021, 1, 1).and_hms(0, 0, 0);
    //     Some(Tz::UTC.from_local_datetime(&local).unwrap())
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_command() {
        assert_eq!(parse_issue("Blah".to_string(), Tz::UTC), None);
    }
}
