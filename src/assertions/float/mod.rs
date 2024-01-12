use std::fmt::Debug;
use std::ops::{Range, RangeInclusive};

use crate::matchers::float::{be_nan, be_negative, be_positive, be_zero};
use crate::matchers::range::{be_in_exclusive_range, be_in_inclusive_range};
use crate::matchers::{Should, ShouldNot};

/// FloatAssertion enables assertions about various properties of floating-point numbers, considering their inherent imprecision.
///
/// It offers a fluent interface for chaining multiple assertions.
///
/// # Example
/// ```
/// use clearcheck::assertions::float::FloatAssertion;
///
/// let value: f64 = 1.34589;
/// value
///     .should_not_be_nan()
///     .should_be_positive()
///     .should_be_in_inclusive_range_with_tolerance(1.11..=1.3458, 0.23);
/// ```
pub trait FloatAssertion<T: num::Float + Debug + Default + PartialEq> {
    /// - Asserts that the floating-point value is NaN (Not a Number).
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::float::FloatAssertion;
    ///
    /// let value: f64 = num::Float::nan();
    /// value.should_be_nan();
    /// ```
    fn should_be_nan(&self) -> &Self;

    /// - Asserts that the floating-point value is not NaN (Not a Number).
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::float::FloatAssertion;
    ///
    /// let value: f64 = 1.23;
    /// value.should_not_be_nan();
    /// ```
    fn should_not_be_nan(&self) -> &Self;

    /// - Asserts that the floating-point value is zero.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::float::FloatAssertion;
    ///
    /// let value: f64 = 0.0;
    /// value.should_be_zero();
    /// ```
    fn should_be_zero(&self) -> &Self;

    /// - Asserts that the floating-point value is not zero.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::float::FloatAssertion;
    ///
    /// let value: f64 = 1.23;
    /// value.should_not_be_zero();
    /// ```
    fn should_not_be_zero(&self) -> &Self;

    /// - Asserts that the floating-point value is positive.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::float::FloatAssertion;
    ///
    /// let value: f64 = 1.24;
    /// value.should_be_positive();
    /// ```
    fn should_be_positive(&self) -> &Self;

    /// - Asserts that the floating-point value is negative.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::float::FloatAssertion;
    ///
    /// let value: f64 = -1.23;
    /// value.should_be_negative();
    /// ```
    fn should_be_negative(&self) -> &Self;

    /// - Asserts that the floating-point value falls within the given inclusive range with tolerance.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::float::FloatAssertion;
    ///
    /// let value: f64 = 1.34589;
    /// value.should_be_in_inclusive_range_with_tolerance(1.11..=1.3458, 0.23);
    /// ```
    fn should_be_in_inclusive_range_with_tolerance(
        &self,
        range: RangeInclusive<T>,
        tolerance: T,
    ) -> &Self;

    /// - Asserts that the floating-point value does not fall within the given inclusive range with tolerance.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::float::FloatAssertion;
    ///
    /// let value: f64 = 1.34589;
    /// value.should_not_be_in_inclusive_range_with_tolerance(1.11..=1.12, 0.12);
    /// ```
    fn should_not_be_in_inclusive_range_with_tolerance(
        &self,
        range: RangeInclusive<T>,
        tolerance: T,
    ) -> &Self;

    /// - Asserts that the floating-point value falls within the given exclusive range with tolerance.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::float::FloatAssertion;
    ///
    /// let value: f64 = 1.34589;
    /// value.should_be_in_exclusive_range_with_tolerance(1.11..1.3458, 0.23);
    /// ```
    fn should_be_in_exclusive_range_with_tolerance(&self, range: Range<T>, tolerance: T) -> &Self;

    /// - Asserts that the floating-point value does not fall within the given exclusive range with tolerance.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::float::FloatAssertion;
    ///
    /// let value: f64 = 1.34589;
    /// value.should_not_be_in_exclusive_range_with_tolerance(1.11..1.12, 0.10);
    /// ```
    fn should_not_be_in_exclusive_range_with_tolerance(
        &self,
        range: Range<T>,
        tolerance: T,
    ) -> &Self;
}

impl<T: num::Float + Debug + Default + PartialEq> FloatAssertion<T> for T {
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
        tolerance: T,
    ) -> &Self {
        self.should(&be_in_inclusive_range(RangeInclusive::new(
            range.start().add(tolerance),
            range.end().add(tolerance),
        )));
        self
    }

    fn should_not_be_in_inclusive_range_with_tolerance(
        &self,
        range: RangeInclusive<T>,
        tolerance: T,
    ) -> &Self {
        self.should_not(&be_in_inclusive_range(RangeInclusive::new(
            range.start().add(tolerance),
            range.end().add(tolerance),
        )));
        self
    }

    fn should_be_in_exclusive_range_with_tolerance(&self, range: Range<T>, tolerance: T) -> &Self {
        let range_with_tolerance = range.start.add(tolerance)..range.end.add(tolerance);
        self.should(&be_in_exclusive_range(range_with_tolerance));
        self
    }

    fn should_not_be_in_exclusive_range_with_tolerance(
        &self,
        range: Range<T>,
        tolerance: T,
    ) -> &Self {
        let range_with_tolerance = range.start.add(tolerance)..range.end.add(tolerance);
        self.should_not(&be_in_exclusive_range(range_with_tolerance));
        self
    }
}

#[cfg(all(test, feature = "num"))]
mod tests {
    use crate::assertions::float::FloatAssertion;

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
        value.should_be_in_inclusive_range_with_tolerance(6.10..=8.10, 0.123);
    }

    #[test]
    #[should_panic]
    fn should_be_in_inclusive_range_with_tolerance_but_was_not() {
        let value: f64 = 8.423;
        value.should_be_in_inclusive_range_with_tolerance(6.10..=8.10, 0.123);
    }

    #[test]
    fn should_not_be_in_inclusive_range_with_tolerance() {
        let value: f64 = 8.123;
        value.should_not_be_in_inclusive_range_with_tolerance(6.10..=7.10, 0.123);
    }

    #[test]
    #[should_panic]
    fn should_not_be_in_inclusive_range_with_tolerance_but_was() {
        let value: f64 = 8.123;
        value.should_not_be_in_inclusive_range_with_tolerance(6.10..=8.10, 0.123);
    }

    #[test]
    fn should_be_in_exclusive_range_with_tolerance() {
        let value: f64 = 8.123;
        value.should_be_in_exclusive_range_with_tolerance(6.10..8.10, 0.123);
    }

    #[test]
    #[should_panic]
    fn should_be_in_exclusive_range_with_tolerance_but_was_not() {
        let value: f64 = 8.423;
        value.should_be_in_exclusive_range_with_tolerance(6.10..8.10, 0.123);
    }

    #[test]
    fn should_not_be_in_exclusive_range_with_tolerance() {
        let value: f64 = 8.423;
        value.should_not_be_in_exclusive_range_with_tolerance(6.10..8.10, 0.123);
    }

    #[test]
    #[should_panic]
    fn should_not_be_in_exclusive_range_with_tolerance_but_was_not() {
        let value: f64 = 8.123;
        value.should_not_be_in_exclusive_range_with_tolerance(6.10..8.20, 0.123);
    }
}
