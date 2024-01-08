use chrono::{Datelike, NaiveDate};

use crate::matchers::{Matcher, MatcherResult};

pub enum DateBased {
    SameYear(i32),
    SameMonth(u32),
    SameDay(u32),
    LeapYear,
}

impl Matcher<NaiveDate> for DateBased {
    fn test(&self, value: &NaiveDate) -> MatcherResult {
        match self {
            DateBased::SameYear(other) => MatcherResult::formatted(
                value.year() == *other,
                format!("{:?} should have the same year as {:?}", value, other),
                format!("{:?} should not have the same year as {:?}", value, other),
            ),
            DateBased::SameMonth(other) => MatcherResult::formatted(
                value.month() == *other,
                format!("{:?} should have the same month as {:?}", value, other),
                format!("{:?} should not have the same month as {:?}", value, other),
            ),
            DateBased::SameDay(other) => MatcherResult::formatted(
                value.day() == *other,
                format!("{:?} should have the same day as {:?}", value, other),
                format!("{:?} should not have the same day as {:?}", value, other),
            ),
            DateBased::LeapYear => MatcherResult::formatted(
                value.leap_year(),
                format!("{:?} should be a leap year", value),
                format!("{:?} should not be a leap year", value),
            ),
        }
    }
}

pub fn have_same_year(year: i32) -> DateBased {
    DateBased::SameYear(year)
}

pub fn have_same_month(month: u32) -> DateBased {
    DateBased::SameMonth(month)
}

pub fn have_same_day(day: u32) -> DateBased {
    DateBased::SameDay(day)
}

pub fn be_a_leap_year() -> DateBased {
    DateBased::LeapYear
}

#[cfg(test)]
mod tests {
    use crate::assertions::bool::TrueFalseAssertions;
    use crate::matchers::date::{be_a_leap_year, have_same_day, have_same_month, have_same_year};
    use crate::matchers::Matcher;
    use chrono::NaiveDate;

    #[test]
    fn should_have_same_year() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 10).unwrap();
        let matcher = have_same_year(2024);
        matcher.test(&date).passed.should_be_true();
    }

    #[test]
    fn should_have_same_month() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 10).unwrap();
        let matcher = have_same_month(1);
        matcher.test(&date).passed.should_be_true();
    }

    #[test]
    fn should_have_same_day() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 10).unwrap();
        let matcher = have_same_day(10);
        matcher.test(&date).passed.should_be_true();
    }

    #[test]
    fn should_be_a_leap_year() {
        let date = NaiveDate::from_ymd_opt(2020, 1, 10).unwrap();
        let matcher = be_a_leap_year();
        matcher.test(&date).passed.should_be_true();
    }
}
