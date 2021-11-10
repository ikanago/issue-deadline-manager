use chrono::{DateTime, Utc};

#[derive(Debug, PartialEq, Eq)]
pub enum DeadlineLabel {
    Outdated,
    DaysBefore(i64),
    WeeksBefore(i64),
    MonthsBefore(i64),
}

pub fn determine_label(deadline: DateTime<Utc>, now: DateTime<Utc>) -> DeadlineLabel {
    if is_after(now, deadline) {
        return DeadlineLabel::Outdated;
    }

    let duration = deadline.signed_duration_since(now);

    let days_before = duration.num_days();
    if days_before < 7 {
        return DeadlineLabel::DaysBefore(days_before);
    }

    let weeks_before = duration.num_weeks();
    if weeks_before < 4 {
        return DeadlineLabel::WeeksBefore(weeks_before);
    }

    let months_before = duration.num_weeks() / 4;
    DeadlineLabel::MonthsBefore(months_before)
}

fn is_after(datetime1: DateTime<Utc>, datetime2: DateTime<Utc>) -> bool {
    datetime1.signed_duration_since(datetime2).num_seconds() > 0
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;

    #[test]
    fn outdated() {
        let deadline = Utc.ymd(2021, 11, 1).and_hms(0, 0, 0);
        let now = Utc.ymd(2021, 11, 2).and_hms(0, 0, 0);
        assert_eq!(determine_label(deadline, now), DeadlineLabel::Outdated);
    }

    #[test]
    fn one_day_before_and_less_than_48_hours_diff() {
        let deadline = Utc.ymd(2021, 11, 3).and_hms(8, 0, 0);
        let now = Utc.ymd(2021, 11, 1).and_hms(9, 0, 0);
        assert_eq!(determine_label(deadline, now), DeadlineLabel::DaysBefore(1));
    }

    #[test]
    fn two_day_before_and_more_than_48_hours_diff() {
        let deadline = Utc.ymd(2021, 11, 3).and_hms(10, 0, 0);
        let now = Utc.ymd(2021, 11, 1).and_hms(9, 0, 0);
        assert_eq!(determine_label(deadline, now), DeadlineLabel::DaysBefore(2));
    }

    #[test]
    fn just_one_week_before_seven_days_before() {
        let deadline = Utc.ymd(2021, 11, 8).and_hms(9, 0, 0);
        let now = Utc.ymd(2021, 11, 1).and_hms(9, 0, 0);
        assert_eq!(
            determine_label(deadline, now),
            DeadlineLabel::WeeksBefore(1)
        );
    }

    #[test]
    fn weeks_before() {
        let testcases = vec![
            (
                Utc.ymd(2021, 11, 14).and_hms(0, 0, 0),
                DeadlineLabel::WeeksBefore(1),
            ),
            (
                Utc.ymd(2021, 11, 15).and_hms(0, 0, 0),
                DeadlineLabel::WeeksBefore(2),
            ),
            (
                Utc.ymd(2021, 11, 21).and_hms(0, 0, 0),
                DeadlineLabel::WeeksBefore(2),
            ),
            (
                Utc.ymd(2021, 11, 22).and_hms(0, 0, 0),
                DeadlineLabel::WeeksBefore(3),
            ),
        ];

        let now = Utc.ymd(2021, 11, 1).and_hms(0, 0, 0);
        for (deadline, label) in testcases {
            assert_eq!(determine_label(deadline, now), label);
        }
    }

    #[test]
    fn four_weeks_before() {
        let testcases = vec![
            (
                Utc.ymd(2021, 11, 28).and_hms(0, 0, 0),
                DeadlineLabel::WeeksBefore(3),
            ),
            (
                Utc.ymd(2021, 11, 29).and_hms(0, 0, 0),
                DeadlineLabel::MonthsBefore(1),
            ),
        ];

        let now = Utc.ymd(2021, 11, 1).and_hms(0, 0, 0);
        for (deadline, label) in testcases {
            assert_eq!(determine_label(deadline, now), label);
        }
    }
}
