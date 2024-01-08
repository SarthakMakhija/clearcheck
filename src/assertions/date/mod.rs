use chrono::{Datelike, NaiveDate};

use crate::matchers::date::{be_a_leap_year, have_same_day, have_same_month, have_same_year};
use crate::matchers::{Should, ShouldNot};

pub trait DateAssertion {
    fn should_have_same_year_as(&self, other: &NaiveDate) -> &Self;
    fn should_not_have_same_year_as(&self, other: &NaiveDate) -> &Self;

    fn should_have_year(&self, year: i32) -> &Self;
    fn should_not_have_year(&self, year: i32) -> &Self;

    fn should_have_same_month_as(&self, other: &NaiveDate) -> &Self;
    fn should_not_have_same_month_as(&self, other: &NaiveDate) -> &Self;

    fn should_have_month(&self, month: u32) -> &Self;
    fn should_not_have_month(&self, month: u32) -> &Self;

    fn should_have_same_day_as(&self, other: &NaiveDate) -> &Self;
    fn should_not_have_same_day_as(&self, other: &NaiveDate) -> &Self;

    fn should_have_day(&self, day: u32) -> &Self;
    fn should_not_have_day(&self, day: u32) -> &Self;

    fn should_be_a_leap_year(&self) -> &Self;
    fn should_not_be_a_leap_year(&self) -> &Self;
}

impl DateAssertion for NaiveDate {
    fn should_have_same_year_as(&self, other: &NaiveDate) -> &Self {
        self.should_have_year(other.year())
    }

    fn should_not_have_same_year_as(&self, other: &NaiveDate) -> &Self {
        self.should_not_have_year(other.year())
    }

    fn should_have_year(&self, year: i32) -> &Self {
        self.should(&have_same_year(year));
        self
    }

    fn should_not_have_year(&self, year: i32) -> &Self {
        self.should_not(&have_same_year(year));
        self
    }

    fn should_have_same_month_as(&self, other: &NaiveDate) -> &Self {
        self.should_have_month(other.month())
    }

    fn should_not_have_same_month_as(&self, other: &NaiveDate) -> &Self {
        self.should_not_have_month(other.month())
    }

    fn should_have_month(&self, month: u32) -> &Self {
        self.should(&have_same_month(month));
        self
    }

    fn should_not_have_month(&self, month: u32) -> &Self {
        self.should_not(&have_same_month(month));
        self
    }

    fn should_have_same_day_as(&self, other: &NaiveDate) -> &Self {
        self.should_have_day(other.day())
    }

    fn should_not_have_same_day_as(&self, other: &NaiveDate) -> &Self {
        self.should_not_have_day(other.day())
    }

    fn should_have_day(&self, day: u32) -> &Self {
        self.should(&have_same_day(day));
        self
    }

    fn should_not_have_day(&self, day: u32) -> &Self {
        self.should_not(&have_same_day(day));
        self
    }

    fn should_be_a_leap_year(&self) -> &Self {
        self.should(&be_a_leap_year());
        self
    }

    fn should_not_be_a_leap_year(&self) -> &Self {
        self.should_not(&be_a_leap_year());
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::assertions::date::DateAssertion;
    use chrono::NaiveDate;

    #[test]
    fn should_have_same_year_as_other() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 10).unwrap();
        date.should_have_same_year_as(&NaiveDate::from_ymd_opt(2024, 10, 20).unwrap());
    }

    #[test]
    #[should_panic]
    fn should_have_same_year_as_other_but_was_not() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 10).unwrap();
        date.should_have_same_year_as(&NaiveDate::from_ymd_opt(2020, 10, 20).unwrap());
    }

    #[test]
    fn should_not_have_same_year_as_other() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 10).unwrap();
        date.should_not_have_same_year_as(&NaiveDate::from_ymd_opt(2020, 10, 20).unwrap());
    }

    #[test]
    #[should_panic]
    fn should_not_have_same_year_as_other_but_was() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 10).unwrap();
        date.should_not_have_same_year_as(&NaiveDate::from_ymd_opt(2024, 10, 20).unwrap());
    }

    #[test]
    fn should_have_same_year() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 10).unwrap();
        date.should_have_year(2024);
    }

    #[test]
    #[should_panic]
    fn should_have_same_year_but_was_not() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 10).unwrap();
        date.should_have_year(2020);
    }

    #[test]
    fn should_not_have_same_year() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 10).unwrap();
        date.should_not_have_year(2020);
    }

    #[test]
    #[should_panic]
    fn should_not_have_same_year_but_was() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 10).unwrap();
        date.should_not_have_year(2024);
    }

    #[test]
    fn should_have_same_month_as_other() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 10).unwrap();
        date.should_have_same_month_as(&NaiveDate::from_ymd_opt(2024, 1, 20).unwrap());
    }

    #[test]
    #[should_panic]
    fn should_have_same_month_as_other_but_was_not() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 10).unwrap();
        date.should_have_same_month_as(&NaiveDate::from_ymd_opt(2020, 8, 20).unwrap());
    }

    #[test]
    fn should_not_have_same_month_as_other() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 10).unwrap();
        date.should_not_have_same_month_as(&NaiveDate::from_ymd_opt(2020, 10, 20).unwrap());
    }

    #[test]
    #[should_panic]
    fn should_not_have_same_month_as_other_but_was() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 10).unwrap();
        date.should_not_have_same_month_as(&NaiveDate::from_ymd_opt(2024, 1, 20).unwrap());
    }

    #[test]
    fn should_have_same_month() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 10).unwrap();
        date.should_have_month(1);
    }

    #[test]
    #[should_panic]
    fn should_have_same_month_but_was_not() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 10).unwrap();
        date.should_have_month(2);
    }

    #[test]
    fn should_not_have_same_month() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 10).unwrap();
        date.should_not_have_month(2);
    }

    #[test]
    #[should_panic]
    fn should_not_have_same_month_but_was() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 10).unwrap();
        date.should_not_have_month(1);
    }

    /////
    #[test]
    fn should_have_same_day_as_other() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 10).unwrap();
        date.should_have_same_day_as(&NaiveDate::from_ymd_opt(2024, 1, 10).unwrap());
    }

    #[test]
    #[should_panic]
    fn should_have_same_day_as_other_but_was_not() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 10).unwrap();
        date.should_have_same_day_as(&NaiveDate::from_ymd_opt(2020, 8, 20).unwrap());
    }

    #[test]
    fn should_not_have_same_day_as_other() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 10).unwrap();
        date.should_not_have_same_day_as(&NaiveDate::from_ymd_opt(2020, 10, 20).unwrap());
    }

    #[test]
    #[should_panic]
    fn should_not_have_same_day_as_other_but_was() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 10).unwrap();
        date.should_not_have_same_day_as(&NaiveDate::from_ymd_opt(2024, 1, 10).unwrap());
    }

    #[test]
    fn should_have_same_day() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 10).unwrap();
        date.should_have_day(10);
    }

    #[test]
    #[should_panic]
    fn should_have_same_day_but_was_not() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 10).unwrap();
        date.should_have_day(2);
    }

    #[test]
    fn should_not_have_same_day() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 10).unwrap();
        date.should_not_have_day(2);
    }

    #[test]
    #[should_panic]
    fn should_not_have_same_day_but_was() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 10).unwrap();
        date.should_not_have_day(10);
    }

    #[test]
    fn should_be_a_leap_year() {
        let date = NaiveDate::from_ymd_opt(2020, 1, 10).unwrap();
        date.should_be_a_leap_year();
    }

    #[test]
    #[should_panic]
    fn should_be_a_leap_year_but_was_not() {
        let date = NaiveDate::from_ymd_opt(2021, 1, 10).unwrap();
        date.should_be_a_leap_year();
    }

    #[test]
    fn should_not_be_a_leap_year() {
        let date = NaiveDate::from_ymd_opt(2021, 1, 10).unwrap();
        date.should_not_be_a_leap_year();
    }

    #[test]
    #[should_panic]
    fn should_be_a_leap_year_but_was() {
        let date = NaiveDate::from_ymd_opt(2020, 1, 10).unwrap();
        date.should_not_be_a_leap_year();
    }
}
