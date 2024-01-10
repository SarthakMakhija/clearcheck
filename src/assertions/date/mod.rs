use chrono::{Datelike, NaiveDate};

use crate::matchers::date::{be_a_leap_year, have_same_day, have_same_month, have_same_year};
use crate::matchers::{Should, ShouldNot};

/// DateAssertion enables assertions about various properties of NaiveDate.
///
///
/// It offers a fluent interface for chaining multiple assertions.
///
/// # Example
/// ```
/// use chrono::NaiveDate;
/// use clearcheck::assertions::date::DateAssertion;
/// use clearcheck::assertions::ordered::OrderedAssertion;
///
/// let date = NaiveDate::from_ymd_opt(2024, 1, 10).unwrap();
/// date
///     .should_be_a_leap_year()
///     .should_have_month(1)
///     .should_be_greater_than(&NaiveDate::from_ymd_opt(2023, 1, 10).unwrap());
/// ```
pub trait DateAssertion {

    /// - Asserts that the date has the same year as the other date.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use chrono::NaiveDate;
    /// use clearcheck::assertions::date::DateAssertion;
    ///
    /// let date = NaiveDate::from_ymd_opt(2024, 1, 10).unwrap();
    /// date.should_have_same_year_as(&NaiveDate::from_ymd_opt(2024, 10, 20).unwrap());
    /// ```
    fn should_have_same_year_as(&self, other: &NaiveDate) -> &Self;

    /// - Asserts that the date does not have the same year as the other date.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use chrono::NaiveDate;
    /// use clearcheck::assertions::date::DateAssertion;
    ///
    /// let date = NaiveDate::from_ymd_opt(2024, 1, 10).unwrap();
    /// date.should_not_have_same_year_as(&NaiveDate::from_ymd_opt(2020, 10, 20).unwrap());
    /// ```
    fn should_not_have_same_year_as(&self, other: &NaiveDate) -> &Self;

    /// - Asserts that the date has the same year as the given year.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use chrono::NaiveDate;
    /// use clearcheck::assertions::date::DateAssertion;
    ///
    /// let date = NaiveDate::from_ymd_opt(2024, 1, 10).unwrap();
    /// date.should_have_year(2024);
    /// ```
    fn should_have_year(&self, year: i32) -> &Self;

    /// - Asserts that the date does not have the same year as the given year.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use chrono::NaiveDate;
    /// use clearcheck::assertions::date::DateAssertion;
    ///
    /// let date = NaiveDate::from_ymd_opt(2024, 1, 10).unwrap();
    /// date.should_not_have_year(2020);
    /// ```
    fn should_not_have_year(&self, year: i32) -> &Self;

    /// - Asserts that the date has the same month as the other date.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use chrono::NaiveDate;
    /// use clearcheck::assertions::date::DateAssertion;
    ///
    /// let date = NaiveDate::from_ymd_opt(2024, 1, 30).unwrap();
    /// date.should_have_same_month_as(&NaiveDate::from_ymd_opt(2024, 1, 20).unwrap());
    /// ```
    fn should_have_same_month_as(&self, other: &NaiveDate) -> &Self;

    /// - Asserts that the date does not have the same month as the other date.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use chrono::NaiveDate;
    /// use clearcheck::assertions::date::DateAssertion;
    ///
    /// let date = NaiveDate::from_ymd_opt(2024, 10, 30).unwrap();
    /// date.should_not_have_same_month_as(&NaiveDate::from_ymd_opt(2024, 1, 20).unwrap());
    /// ```
    fn should_not_have_same_month_as(&self, other: &NaiveDate) -> &Self;

    /// - Asserts that the date has the same month as the given month.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use chrono::NaiveDate;
    /// use clearcheck::assertions::date::DateAssertion;
    ///
    /// let date = NaiveDate::from_ymd_opt(2024, 10, 30).unwrap();
    /// date.should_have_month(10);
    /// ```
    fn should_have_month(&self, month: u32) -> &Self;

    /// - Asserts that the date does not have the same month as the given month.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use chrono::NaiveDate;
    /// use clearcheck::assertions::date::DateAssertion;
    ///
    /// let date = NaiveDate::from_ymd_opt(2024, 10, 30).unwrap();
    /// date.should_not_have_month(1);
    /// ```
    fn should_not_have_month(&self, month: u32) -> &Self;

    /// - Asserts that the date has the same day as the other date.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use chrono::NaiveDate;
    /// use clearcheck::assertions::date::DateAssertion;
    ///
    /// let date = NaiveDate::from_ymd_opt(2024, 10, 30).unwrap();
    /// date.should_have_same_day_as(&NaiveDate::from_ymd_opt(2024, 1, 30).unwrap());
    /// ```
    fn should_have_same_day_as(&self, other: &NaiveDate) -> &Self;

    /// - Asserts that the date does not have the same day as the other date.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use chrono::NaiveDate;
    /// use clearcheck::assertions::date::DateAssertion;
    ///
    /// let date = NaiveDate::from_ymd_opt(2024, 10, 30).unwrap();
    /// date.should_not_have_same_day_as(&NaiveDate::from_ymd_opt(2024, 1, 20).unwrap());
    /// ```
    fn should_not_have_same_day_as(&self, other: &NaiveDate) -> &Self;

    /// - Asserts that the date has the same day as the other day.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use chrono::NaiveDate;
    /// use clearcheck::assertions::date::DateAssertion;
    ///
    /// let date = NaiveDate::from_ymd_opt(2024, 10, 30).unwrap();
    /// date.should_have_day(30);
    /// ```
    fn should_have_day(&self, day: u32) -> &Self;

    /// - Asserts that the date does not have the same day as the other day.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use chrono::NaiveDate;
    /// use clearcheck::assertions::date::DateAssertion;
    ///
    /// let date = NaiveDate::from_ymd_opt(2024, 10, 30).unwrap();
    /// date.should_not_have_day(10);
    /// ```
    fn should_not_have_day(&self, day: u32) -> &Self;

    /// - Asserts that the date falls in a leap year.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use chrono::NaiveDate;
    /// use clearcheck::assertions::date::DateAssertion;
    ///
    /// let date = NaiveDate::from_ymd_opt(2024, 10, 30).unwrap();
    /// date.should_be_a_leap_year();
    /// ```
    fn should_be_a_leap_year(&self) -> &Self;

    /// - Asserts that the date does not fall in a leap year.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use chrono::NaiveDate;
    /// use clearcheck::assertions::date::DateAssertion;
    ///
    /// let date = NaiveDate::from_ymd_opt(2021, 10, 30).unwrap();
    /// date.should_not_be_a_leap_year();
    /// ```
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

#[cfg(all(test, feature = "date"))]
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
    fn should_not_be_a_leap_year_but_was() {
        let date = NaiveDate::from_ymd_opt(2020, 1, 10).unwrap();
        date.should_not_be_a_leap_year();
    }
}
