use std::borrow::Borrow;
use std::fmt::Debug;
use std::ops::{Range, RangeInclusive};

use crate::matchers::ordered::{
    be_greater_than, be_greater_than_equal_to, be_less_than, be_less_than_equal_to,
};
use crate::matchers::range::{be_in_exclusive_range, be_in_inclusive_range};
use crate::matchers::{Should, ShouldNot};

/// OrderedAssertion enables assertions about the relative ordering of values that implement the PartialOrd trait.
///
/// # Example
/// ```
/// use clearcheck::assertions::ordered::OrderedAssertion;
///
/// let value = 12.56;
/// value
///     .should_be_greater_than(&10.90)
///     .should_be_less_than(&15.98)
///     .should_be_in_inclusive_range(10.90..=13.10);
/// ```
pub trait OrderedAssertion<T: PartialOrd> {

    /// - Asserts that the self value is greater than the provided value (other) according to the PartialOrd implementation.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::ordered::OrderedAssertion;
    ///
    /// let value = 12.5;
    /// value.should_be_greater_than(&10.98);
    /// ```
    fn should_be_greater_than<Q>(&self, other: &Q) -> &Self
    where
        T: Borrow<Q>,
        Q: PartialOrd + Debug + ?Sized;

    /// - Asserts that the self value is greater than or equal to the provided value (other) according to the PartialOrd implementation.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::ordered::OrderedAssertion;
    ///
    /// let value = 12.5;
    /// value.should_be_greater_than_equal_to(&10.98);
    /// ```
    fn should_be_greater_than_equal_to<Q>(&self, other: &Q) -> &Self
    where
        T: Borrow<Q>,
        Q: PartialOrd + Debug + ?Sized;

    /// - Asserts that the self value is less than the provided value (other) according to the PartialOrd implementation.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::ordered::OrderedAssertion;
    ///
    /// let value = 10.5;
    /// value.should_be_less_than(&10.98);
    /// ```
    fn should_be_less_than<Q>(&self, other: &Q) -> &Self
    where
        T: Borrow<Q>,
        Q: PartialOrd + Debug + ?Sized;

    /// - Asserts that the self value is less than or equal to the provided value (other) according to the PartialOrd implementation.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::ordered::OrderedAssertion;
    ///
    /// let value = 10.5;
    /// value.should_be_less_than_equal_to(&10.98);
    /// ```
    fn should_be_less_than_equal_to<Q>(&self, other: &Q) -> &Self
    where
        T: Borrow<Q>,
        Q: PartialOrd + Debug + ?Sized;

    /// - Asserts that the self value is not greater than the provided value (other) according to the PartialOrd implementation.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::ordered::OrderedAssertion;
    ///
    /// let value = 10.5;
    /// value.should_not_be_greater_than(&10.98);
    /// ```
    fn should_not_be_greater_than<Q>(&self, other: &Q) -> &Self
    where
        T: Borrow<Q>,
        Q: PartialOrd + Debug + ?Sized;

    /// - Asserts that the self value is not greater than or equal to the provided value (other) according to the PartialOrd implementation.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::ordered::OrderedAssertion;
    ///
    /// let value = 10.5;
    /// value.should_not_be_greater_than_equal_to(&10.98);
    /// ```
    fn should_not_be_greater_than_equal_to<Q>(&self, other: &Q) -> &Self
    where
        T: Borrow<Q>,
        Q: PartialOrd + Debug + ?Sized;

    /// - Asserts that the self value is not less than the provided value (other) according to the PartialOrd implementation.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::ordered::OrderedAssertion;
    ///
    /// let value = 11.5;
    /// value.should_not_be_less_than(&10.98);
    /// ```
    fn should_not_be_less_than<Q>(&self, other: &Q) -> &Self
    where
        T: Borrow<Q>,
        Q: PartialOrd + Debug + ?Sized;

    /// - Asserts that the self value is not less than or equal to the provided value (other) according to the PartialOrd implementation.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::ordered::OrderedAssertion;
    ///
    /// let value = 11.5;
    /// value.should_not_be_less_than_equal_to(&10.98);
    /// ```
    fn should_not_be_less_than_equal_to<Q>(&self, other: &Q) -> &Self
    where
        T: Borrow<Q>,
        Q: PartialOrd + Debug + ?Sized;

    /// - Asserts that the self value falls within the given inclusive range.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::ordered::OrderedAssertion;
    ///
    /// let name = "junit";
    /// name.should_be_in_inclusive_range("clearcheck"..="scalatest");
    /// ```
    fn should_be_in_inclusive_range(&self, range: RangeInclusive<T>) -> &Self;

    /// - Asserts that the self value does not fall within the given inclusive range.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::ordered::OrderedAssertion;
    ///
    /// let name = "junit";
    /// name.should_not_be_in_inclusive_range("clearcheck"..="gotest");
    /// ```
    fn should_not_be_in_inclusive_range(&self, range: RangeInclusive<T>) -> &Self;

    /// - Asserts that the self value falls within the given exclusive range.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::ordered::OrderedAssertion;
    ///
    /// let name = "junit";
    /// name.should_be_in_exclusive_range("clearcheck".."scalatest");
    /// ```
    fn should_be_in_exclusive_range(&self, range: Range<T>) -> &Self;

    /// - Asserts that the self value falls within the given exclusive range.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::ordered::OrderedAssertion;
    ///
    /// let name = "junit";
    /// name.should_not_be_in_exclusive_range("clearcheck".."gotest");
    /// ```
    fn should_not_be_in_exclusive_range(&self, range: Range<T>) -> &Self;
}

impl<T: PartialOrd + Debug> OrderedAssertion<T> for T {
    fn should_be_greater_than<Q>(&self, other: &Q) -> &Self
    where
        T: Borrow<Q>,
        Q: PartialOrd + Debug + ?Sized,
    {
        self.borrow().should(&be_greater_than(other));
        self
    }

    fn should_be_greater_than_equal_to<Q>(&self, other: &Q) -> &Self
    where
        T: Borrow<Q>,
        Q: PartialOrd + Debug + ?Sized,
    {
        self.borrow().should(&be_greater_than_equal_to(other));
        self
    }

    fn should_be_less_than<Q>(&self, other: &Q) -> &Self
    where
        T: Borrow<Q>,
        Q: PartialOrd + Debug + ?Sized,
    {
        self.borrow().should(&be_less_than(other));
        self
    }

    fn should_be_less_than_equal_to<Q>(&self, other: &Q) -> &Self
    where
        T: Borrow<Q>,
        Q: PartialOrd + Debug + ?Sized,
    {
        self.borrow().should(&be_less_than_equal_to(other));
        self
    }

    fn should_not_be_greater_than<Q>(&self, other: &Q) -> &Self
    where
        T: Borrow<Q>,
        Q: PartialOrd + Debug + ?Sized,
    {
        self.borrow().should_not(&be_greater_than(other));
        self
    }

    fn should_not_be_greater_than_equal_to<Q>(&self, other: &Q) -> &Self
    where
        T: Borrow<Q>,
        Q: PartialOrd + Debug + ?Sized,
    {
        self.borrow().should_not(&be_greater_than_equal_to(other));
        self
    }

    fn should_not_be_less_than<Q>(&self, other: &Q) -> &Self
    where
        T: Borrow<Q>,
        Q: PartialOrd + Debug + ?Sized,
    {
        self.borrow().should_not(&be_less_than(other));
        self
    }

    fn should_not_be_less_than_equal_to<Q>(&self, other: &Q) -> &Self
    where
        T: Borrow<Q>,
        Q: PartialOrd + Debug + ?Sized,
    {
        self.borrow().should_not(&be_less_than_equal_to(other));
        self
    }

    fn should_be_in_inclusive_range(&self, range: RangeInclusive<T>) -> &Self {
        self.should(&be_in_inclusive_range(range));
        self
    }

    fn should_not_be_in_inclusive_range(&self, range: RangeInclusive<T>) -> &Self {
        self.should_not(&be_in_inclusive_range(range));
        self
    }

    fn should_be_in_exclusive_range(&self, range: Range<T>) -> &Self {
        self.should(&be_in_exclusive_range(range));
        self
    }

    fn should_not_be_in_exclusive_range(&self, range: Range<T>) -> &Self {
        self.should_not(&be_in_exclusive_range(range));
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::assertions::ordered::OrderedAssertion;

    #[test]
    fn should_be_greater_than() {
        let value = 12.5;
        value.should_be_greater_than(&10.98);
    }

    #[test]
    fn should_be_greater_than_equal_to() {
        let value = 12.59;
        value.should_be_greater_than_equal_to(&12.59);
    }

    #[test]
    fn should_be_less_than() {
        let value = 6.98;
        value.should_be_less_than(&9.98);
    }

    #[test]
    fn should_be_less_than_equal_to() {
        let value = 12.59;
        value.should_be_less_than_equal_to(&12.59);
    }

    #[test]
    fn should_not_be_greater_than() {
        let value = 12.5;
        value.should_not_be_greater_than(&14.98);
    }

    #[test]
    fn should_not_be_greater_than_equal_to() {
        let value = 12.59;
        value.should_not_be_greater_than_equal_to(&13.59);
    }

    #[test]
    fn should_not_be_less_than() {
        let value = 9.98;
        value.should_not_be_less_than(&8.98);
    }

    #[test]
    fn should_not_be_less_than_equal_to() {
        let value = 13.59;
        value.should_not_be_less_than_equal_to(&12.59);
    }

    #[test]
    fn should_be_in_inclusive_range() {
        let value = 9.98;
        value.should_be_in_inclusive_range(8.90..=9.99);
    }

    #[test]
    fn should_not_be_in_inclusive_range() {
        let value = 9.98;
        value.should_not_be_in_inclusive_range(8.90..=9.10);
    }

    #[test]
    fn should_be_in_exclusive_range() {
        let value = 9.98;
        value.should_be_in_exclusive_range(8.90..9.99);
    }

    #[test]
    fn should_not_be_in_exclusive_range() {
        let value = 9.98;
        value.should_not_be_in_exclusive_range(8.90..9.10);
    }

    #[test]
    #[should_panic]
    fn should_be_greater_than_but_was_not() {
        let value = 1.1;
        value.should_be_greater_than(&10.98);
    }

    #[test]
    #[should_panic]
    fn should_be_greater_than_equal_to_but_was_not() {
        let value = 1.59;
        value.should_be_greater_than_equal_to(&12.59);
    }

    #[test]
    #[should_panic]
    fn should_be_less_than_but_was_not() {
        let value = 16.98;
        value.should_be_less_than(&9.98);
    }

    #[test]
    #[should_panic]
    fn should_be_less_than_equal_to_but_was_not() {
        let value = 18.59;
        value.should_be_less_than_equal_to(&12.59);
    }

    #[test]
    #[should_panic]
    fn should_not_be_greater_than_but_was() {
        let value = 15.5;
        value.should_not_be_greater_than(&14.98);
    }

    #[test]
    #[should_panic]
    fn should_not_be_greater_than_equal_to_but_was() {
        let value = 14.59;
        value.should_not_be_greater_than_equal_to(&13.59);
    }

    #[test]
    #[should_panic]
    fn should_not_be_less_than_but_was() {
        let value = 1.98;
        value.should_not_be_less_than(&8.98);
    }

    #[test]
    #[should_panic]
    fn should_not_be_less_than_equal_to_but_was() {
        let value = 11.59;
        value.should_not_be_less_than_equal_to(&12.59);
    }

    #[test]
    #[should_panic]
    fn should_be_in_inclusive_range_but_was_not() {
        let value = 9.98;
        value.should_be_in_inclusive_range(8.90..=9.90);
    }

    #[test]
    #[should_panic]
    fn should_not_be_in_inclusive_range_but_was() {
        let value = 9.98;
        value.should_not_be_in_inclusive_range(8.90..=9.99);
    }

    #[test]
    #[should_panic]
    fn should_be_in_exclusive_range_but_was_not() {
        let value = 9.98;
        value.should_be_in_exclusive_range(8.90..9.90);
    }

    #[test]
    #[should_panic]
    fn should_not_be_in_exclusive_range_but_was_not() {
        let value = 9.98;
        value.should_not_be_in_exclusive_range(8.90..9.99);
    }
}

#[cfg(test)]
mod string_tests {
    use crate::assertions::ordered::OrderedAssertion;

    #[test]
    fn should_be_less_than() {
        let name = "junit";
        name.should_be_less_than("scalatest");
    }

    #[test]
    fn should_not_be_less_than() {
        let name = "junit";
        name.should_not_be_less_than("clearcheck");
    }

    #[test]
    fn should_be_in_inclusive_range() {
        let name = "junit";
        name.should_be_in_inclusive_range("clearcheck"..="scalatest");
    }

    #[test]
    fn should_be_in_exclusive_range() {
        let name = "junit";
        name.should_be_in_exclusive_range("clearcheck".."scalatest");
    }
}
