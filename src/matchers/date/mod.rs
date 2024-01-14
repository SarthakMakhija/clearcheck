use chrono::{Datelike, NaiveDate};

use crate::matchers::{Matcher, MatcherResult};

/// DateMatcher offers a flexible way to make assertions about specific date attributes.
///
/// # Example
///```
/// use chrono::NaiveDate;
/// use clearcheck::matchers::date::have_same_year;
/// use clearcheck::matchers::Matcher;
///
/// let date = NaiveDate::from_ymd_opt(2024, 1, 10).unwrap();
/// let matcher = have_same_year(2024);
///
/// assert!(matcher.test(&date).passed());
/// ```
pub enum DateMatcher {
    SameYear(i32),
    SameMonth(u32),
    SameDay(u32),
    LeapYear,
}

impl Matcher<NaiveDate> for DateMatcher {
    fn test(&self, value: &NaiveDate) -> MatcherResult {
        match self {
            DateMatcher::SameYear(other) => MatcherResult::formatted(
                value.year() == *other,
                format!("{:?} should have the same year as {:?}", value, other),
                format!("{:?} should not have the same year as {:?}", value, other),
            ),
            DateMatcher::SameMonth(other) => MatcherResult::formatted(
                value.month() == *other,
                format!("{:?} should have the same month as {:?}", value, other),
                format!("{:?} should not have the same month as {:?}", value, other),
            ),
            DateMatcher::SameDay(other) => MatcherResult::formatted(
                value.day() == *other,
                format!("{:?} should have the same day as {:?}", value, other),
                format!("{:?} should not have the same day as {:?}", value, other),
            ),
            DateMatcher::LeapYear => MatcherResult::formatted(
                value.leap_year(),
                format!("{:?} should be a leap year", value),
                format!("{:?} should not be a leap year", value),
            ),
        }
    }
}

/// Creates a DateMatcher that asserts whether a date has the same year as the given year.
pub fn have_same_year(year: i32) -> DateMatcher {
    DateMatcher::SameYear(year)
}

/// Creates a DateMatcher that asserts whether a date has the same month as the given month.
pub fn have_same_month(month: u32) -> DateMatcher {
    DateMatcher::SameMonth(month)
}

/// Creates a DateMatcher that asserts whether a date has the same day as the given day.
pub fn have_same_day(day: u32) -> DateMatcher {
    DateMatcher::SameDay(day)
}

/// Creates a DateMatcher that asserts whether a date falls in the leap year.
pub fn be_a_leap_year() -> DateMatcher {
    DateMatcher::LeapYear
}

#[cfg(all(test, feature = "date"))]
mod tests {
    use crate::assertions::bool::TrueFalseAssertion;
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
