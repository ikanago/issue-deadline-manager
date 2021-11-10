use chrono::{DateTime, NaiveDate, TimeZone};
use chrono_tz::Tz;

pub fn parse_issue(content: &str, timezone: Tz, year: i32) -> Option<DateTime<Tz>> {
    //     let local = NaiveDate::from_ymd(2021, 1, 1).and_hms(0, 0, 0);
    //     Some(Tz::UTC.from_local_datetime(&local).unwrap())
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_command() {
        assert_eq!(parse_issue("Blah", Tz::UTC, 2021), None);
    }

    #[test]
    fn lack_of_deadline() {
        assert_eq!(parse_issue("2021/12/01", Tz::UTC, 2021), None);
    }

    #[test]
    fn yyyy_mm_dd_hh_mm() {
        let comment = "Homework\n/deadline 2021/12/01 09:12";
        assert_eq!(
            parse_issue(comment, Tz::UTC, 2021),
            Some(Tz::UTC.ymd(2021, 12, 1).and_hms(9, 12, 0))
        );
    }

    #[test]
    fn yyyy_mm_dd() {
        let comment = "Homework\n/deadline 2021/12/01";
        assert_eq!(
            parse_issue(comment, Tz::UTC, 2021),
            Some(Tz::UTC.ymd(2021, 12, 1).and_hms(0, 0, 0))
        );
    }

    #[test]
    fn mm_dd() {
        let comment = "Homework\n/deadline 12/01";
        assert_eq!(
            parse_issue(comment, Tz::UTC, 2021),
            Some(Tz::UTC.ymd(2021, 12, 1).and_hms(0, 0, 0))
        );
    }

    #[test]
    fn line_not_starting_with_command() {
        let comment = "Homework /deadline 2021/12/01";
        assert_eq!(
            parse_issue(comment, Tz::UTC, 2021),
            Some(Tz::UTC.ymd(2021, 12, 1).and_hms(0, 0, 0))
        );
    }

    #[test]
    fn multiple_commands_results_in_last_one() {
        let comment =
            "Homework /deadline 2021/12/01 09:12\n/deadline 2021/11/1\n/deadline 2021/1/1";
        assert_eq!(
            parse_issue(comment, Tz::UTC, 2021),
            Some(Tz::UTC.ymd(2021, 1, 1).and_hms(0, 0, 0))
        );
    }

    #[test]
    fn non_utc_timezone() {
        let comment = "Homework\n/deadline 2021/12/01 12:00";
        assert_eq!(
            parse_issue(comment, Tz::Asia__Tokyo, 2021),
            Some(Tz::Asia__Tokyo.ymd(2021, 12, 1).and_hms(12, 0, 0))
        );
    }
}
