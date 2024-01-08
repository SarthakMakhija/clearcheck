use std::fmt::Debug;
use std::ops::{Range, RangeInclusive};

use crate::matchers::float::{be_nan, be_negative, be_positive, be_zero};
use crate::matchers::range::{be_in_exclusive_range, be_in_inclusive_range};
use crate::matchers::{Should, ShouldNot};

pub trait FloatAssertions<T: num::Float + Debug + Default + PartialEq> {
    fn should_be_nan(&self) -> &Self;
    fn should_not_be_nan(&self) -> &Self;
    fn should_be_zero(&self) -> &Self;
    fn should_not_be_zero(&self) -> &Self;

    fn should_be_positive(&self) -> &Self;
    fn should_be_negative(&self) -> &Self;

    fn should_be_in_inclusive_range_with_tolerance(
        &self,
        range: RangeInclusive<T>,
        tolerance: &T,
    ) -> &Self;

    fn should_not_be_in_inclusive_range_with_tolerance(
        &self,
        range: RangeInclusive<T>,
        tolerance: &T,
    ) -> &Self;

    fn should_be_in_exclusive_range_with_tolerance(&self, range: Range<T>, tolerance: &T) -> &Self;

    fn should_not_be_in_exclusive_range_with_tolerance(
        &self,
        range: Range<T>,
        tolerance: &T,
    ) -> &Self;
}

impl<T: num::Float + Debug + Default + PartialEq> FloatAssertions<T> for T {
    fn should_be_nan(&self) -> &Self {
        self.should(&be_nan());
        self
    }

    fn should_not_be_nan(&self) -> &Self {
        self.should_not(&be_nan());
        self
    }

    fn should_be_zero(&self) -> &Self {
        self.should(&be_zero());
        self
    }

    fn should_not_be_zero(&self) -> &Self {
        self.should_not(&be_zero());
        self
    }

    fn should_be_positive(&self) -> &Self {
        self.should(&be_positive());
        self
    }

    fn should_be_negative(&self) -> &Self {
        self.should(&be_negative());
        self
    }

    fn should_be_in_inclusive_range_with_tolerance(
        &self,
        range: RangeInclusive<T>,
        tolerance: &T,
    ) -> &Self {
        self.should(&be_in_inclusive_range(&RangeInclusive::new(
            range.start().add(*tolerance),
            range.end().add(*tolerance),
        )));
        self
    }

    fn should_not_be_in_inclusive_range_with_tolerance(
        &self,
        range: RangeInclusive<T>,
        tolerance: &T,
    ) -> &Self {
        self.should_not(&be_in_inclusive_range(&RangeInclusive::new(
            range.start().add(*tolerance),
            range.end().add(*tolerance),
        )));
        self
    }

    fn should_be_in_exclusive_range_with_tolerance(&self, range: Range<T>, tolerance: &T) -> &Self {
        let range_with_tolerance = range.start.add(*tolerance)..range.end.add(*tolerance);
        self.should(&be_in_exclusive_range(&range_with_tolerance));
        self
    }

    fn should_not_be_in_exclusive_range_with_tolerance(
        &self,
        range: Range<T>,
        tolerance: &T,
    ) -> &Self {
        let range_with_tolerance = range.start.add(*tolerance)..range.end.add(*tolerance);
        self.should_not(&be_in_exclusive_range(&range_with_tolerance));
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::assertions::float::FloatAssertions;

    #[test]
    fn should_be_nan() {
        let value: f64 = num::Float::nan();
        value.should_be_nan();
    }

    #[test]
    #[should_panic]
    fn should_be_nan_but_was_not() {
        let value: f64 = 1.23;
        value.should_be_nan();
    }

    #[test]
    fn should_not_be_nan() {
        let value: f64 = 1.23;
        value.should_not_be_nan();
    }

    #[test]
    #[should_panic]
    fn should_not_be_nan_but_was() {
        let value: f64 = num::Float::nan();
        value.should_not_be_nan();
    }

    #[test]
    fn should_be_zero() {
        let value: f64 = 0.0;
        value.should_be_zero();
    }

    #[test]
    #[should_panic]
    fn should_not_be_zero_but_was() {
        let value: f64 = 0.0;
        value.should_not_be_zero();
    }

    #[test]
    fn should_be_positive() {
        let value: f64 = 0.123;
        value.should_be_positive();
    }

    #[test]
    fn should_be_negative() {
        let value: f64 = -1.23;
        value.should_be_negative();
    }

    #[test]
    fn should_be_in_inclusive_range_with_tolerance() {
        let value: f64 = 8.123;
        value.should_be_in_inclusive_range_with_tolerance(6.10..=8.10, &0.123);
    }

    #[test]
    #[should_panic]
    fn should_be_in_inclusive_range_with_tolerance_but_was_not() {
        let value: f64 = 8.423;
        value.should_be_in_inclusive_range_with_tolerance(6.10..=8.10, &0.123);
    }

    #[test]
    fn should_be_in_exclusive_range_with_tolerance() {
        let value: f64 = 8.123;
        value.should_be_in_exclusive_range_with_tolerance(6.10..8.10, &0.123);
    }

    #[test]
    #[should_panic]
    fn should_be_in_exclusive_range_with_tolerance_but_was_not() {
        let value: f64 = 8.423;
        value.should_be_in_exclusive_range_with_tolerance(6.10..8.10, &0.123);
    }
}
