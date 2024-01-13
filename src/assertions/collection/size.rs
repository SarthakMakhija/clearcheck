use std::ops::{Range, RangeInclusive};

use crate::matchers::collection::length::{
    have_atleast_same_length, have_atmost_same_length, have_same_length,
};
use crate::matchers::range::{have_length_in_exclusive_range, have_length_in_inclusive_range};
use crate::matchers::{Should, ShouldNot};

/// SizeAssertion enables assertions about the size of the underlying collection.
///
/// It offers a fluent interface for chaining multiple assertions.
///
/// # Example
/// ```
/// use clearcheck::assertions::collection::size::SizeAssertion;
///
/// let collection = vec!["clearcheck", "testify"];
/// collection
///     .should_have_at_least_size(1)
///     .should_have_size_in_inclusive_range(1..=5);
/// ```
pub trait SizeAssertion {
    /// - Asserts that the size of the underlying collection is exactly the given size.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::collection::size::SizeAssertion;
    ///
    /// let collection = vec!["clearcheck", "testify"];
    /// collection.should_have_size(2);
    /// ```
    fn should_have_size(&self, size: usize) -> &Self;

    /// - Asserts that the size of the underlying collection is not the given size.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::collection::size::SizeAssertion;
    ///
    /// let collection = vec!["clearcheck", "testify"];
    /// collection.should_not_have_size(3);
    /// ```
    fn should_not_have_size(&self, size: usize) -> &Self;

    /// - Asserts that the size of the underlying collection is greater than or equal to the given size.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::collection::size::SizeAssertion;
    ///
    /// let collection = vec!["clearcheck", "testify"];
    /// collection.should_have_at_least_size(2);
    /// ```
    fn should_have_at_least_size(&self, size: usize) -> &Self;

    /// - Asserts that the size of the underlying collection is less than or equal to the given size.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::collection::size::SizeAssertion;
    ///
    /// let collection = vec!["clearcheck", "testify"];
    /// collection.should_have_at_most_size(2);
    /// ```
    fn should_have_at_most_size(&self, size: usize) -> &Self;

    /// - Asserts that the size of the underlying collection is same as that of the given collection.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::collection::size::SizeAssertion;
    ///
    /// let collection = vec!["clearcheck", "testify"];
    /// collection.should_be_same_size_as(&[1, 2]);
    /// ```
    fn should_be_same_size_as<U>(&self, other: &[U]) -> &Self;

    /// - Asserts that the size of the underlying collection falls within the given inclusive range.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::collection::size::SizeAssertion;
    ///
    /// let collection = vec!["clearcheck", "testify"];
    /// collection.should_have_size_in_inclusive_range(2..=5);
    /// ```
    fn should_have_size_in_inclusive_range(&self, range: RangeInclusive<usize>) -> &Self;

    /// - Asserts that the size of the underlying collection does not fall within the given inclusive range.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::collection::size::SizeAssertion;
    ///
    /// let collection = vec!["clearcheck", "testify"];
    /// collection.should_not_have_size_in_inclusive_range(3..=5);
    /// ```
    fn should_not_have_size_in_inclusive_range(&self, range: RangeInclusive<usize>) -> &Self;

    /// - Asserts that the size of the underlying collection falls within the given exclusive range.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::collection::size::SizeAssertion;
    ///
    /// let collection = vec!["clearcheck", "testify"];
    /// collection.should_have_size_in_exclusive_range(1..3);
    /// ```
    fn should_have_size_in_exclusive_range(&self, range: Range<usize>) -> &Self;

    /// - Asserts that the size of the underlying collection does not fall within the given exclusive range.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::collection::size::SizeAssertion;
    ///
    /// let collection = vec!["clearcheck", "testify"];
    /// collection.should_not_have_size_in_exclusive_range(3..5);
    /// ```
    fn should_not_have_size_in_exclusive_range(&self, range: Range<usize>) -> &Self;
}

impl<T> SizeAssertion for Vec<T>
where
    T: std::fmt::Debug,
{
    fn should_have_size(&self, size: usize) -> &Self {
        (self as &[T]).should_have_size(size);
        self
    }

    fn should_not_have_size(&self, size: usize) -> &Self {
        (self as &[T]).should_not_have_size(size);
        self
    }

    fn should_have_at_least_size(&self, size: usize) -> &Self {
        (self as &[T]).should_have_at_least_size(size);
        self
    }

    fn should_have_at_most_size(&self, size: usize) -> &Self {
        (self as &[T]).should_have_at_most_size(size);
        self
    }

    fn should_be_same_size_as<U>(&self, other: &[U]) -> &Self {
        (self as &[T]).should_be_same_size_as(other);
        self
    }

    fn should_have_size_in_inclusive_range(&self, range: RangeInclusive<usize>) -> &Self {
        (self as &[T]).should_have_size_in_inclusive_range(range);
        self
    }

    fn should_not_have_size_in_inclusive_range(&self, range: RangeInclusive<usize>) -> &Self {
        (self as &[T]).should_not_have_size_in_inclusive_range(range);
        self
    }

    fn should_have_size_in_exclusive_range(&self, range: Range<usize>) -> &Self {
        (self as &[T]).should_have_size_in_exclusive_range(range);
        self
    }

    fn should_not_have_size_in_exclusive_range(&self, range: Range<usize>) -> &Self {
        (self as &[T]).should_not_have_size_in_exclusive_range(range);
        self
    }
}

impl<T, const N: usize> SizeAssertion for [T; N]
where
    T: std::fmt::Debug,
{
    fn should_have_size(&self, size: usize) -> &Self {
        (self as &[T]).should_have_size(size);
        self
    }

    fn should_not_have_size(&self, size: usize) -> &Self {
        (self as &[T]).should_not_have_size(size);
        self
    }

    fn should_have_at_least_size(&self, size: usize) -> &Self {
        (self as &[T]).should_have_at_least_size(size);
        self
    }

    fn should_have_at_most_size(&self, size: usize) -> &Self {
        (self as &[T]).should_have_at_most_size(size);
        self
    }

    fn should_be_same_size_as<U>(&self, other: &[U]) -> &Self {
        (self as &[T]).should_be_same_size_as(other);
        self
    }

    fn should_have_size_in_inclusive_range(&self, range: RangeInclusive<usize>) -> &Self {
        (self as &[T]).should_have_size_in_inclusive_range(range);
        self
    }

    fn should_not_have_size_in_inclusive_range(&self, range: RangeInclusive<usize>) -> &Self {
        (self as &[T]).should_not_have_size_in_inclusive_range(range);
        self
    }

    fn should_have_size_in_exclusive_range(&self, range: Range<usize>) -> &Self {
        (self as &[T]).should_have_size_in_exclusive_range(range);
        self
    }

    fn should_not_have_size_in_exclusive_range(&self, range: Range<usize>) -> &Self {
        (self as &[T]).should_not_have_size_in_exclusive_range(range);
        self
    }
}

impl<T> SizeAssertion for [T]
where
    T: std::fmt::Debug,
{
    fn should_have_size(&self, size: usize) -> &Self {
        self.should(&have_same_length(size));
        self
    }

    fn should_not_have_size(&self, size: usize) -> &Self {
        self.should_not(&have_same_length(size));
        self
    }

    fn should_have_at_least_size(&self, size: usize) -> &Self {
        self.should(&have_atleast_same_length(size));
        self
    }

    fn should_have_at_most_size(&self, size: usize) -> &Self {
        self.should(&have_atmost_same_length(size));
        self
    }

    fn should_be_same_size_as<U>(&self, other: &[U]) -> &Self {
        self.should(&have_same_length(other.len()));
        self
    }

    fn should_have_size_in_inclusive_range(&self, range: RangeInclusive<usize>) -> &Self {
        self.len().should(&have_length_in_inclusive_range(range));
        self
    }

    fn should_not_have_size_in_inclusive_range(&self, range: RangeInclusive<usize>) -> &Self {
        self.len()
            .should_not(&have_length_in_inclusive_range(range));
        self
    }

    fn should_have_size_in_exclusive_range(&self, range: Range<usize>) -> &Self {
        self.len().should(&have_length_in_exclusive_range(range));
        self
    }

    fn should_not_have_size_in_exclusive_range(&self, range: Range<usize>) -> &Self {
        self.len()
            .should_not(&have_length_in_exclusive_range(range));
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::assertions::collection::size::SizeAssertion;

    #[test]
    fn should_have_size_as_2() {
        let collection = vec!["junit", "testify"];
        collection.should_have_size(2);
    }

    #[test]
    #[should_panic]
    fn should_have_size_as_3_but_was_not() {
        let collection = vec!["junit", "testify"];
        collection.should_have_size(3);
    }

    #[test]
    fn should_not_have_size_as_3() {
        let collection = vec!["junit", "testify"];
        collection.should_not_have_size(3);
    }

    #[test]
    #[should_panic]
    fn should_not_have_size_as_2_but_was() {
        let collection = vec!["junit", "testify"];
        collection.should_not_have_size(2);
    }

    #[test]
    fn should_have_at_least_size_3() {
        let collection = vec!["junit", "testify", "catch2"];
        collection.should_have_at_least_size(3);
    }

    #[test]
    #[should_panic]
    fn should_have_at_least_size_4_but_was_not() {
        let collection = vec!["junit", "testify", "catch2"];
        collection.should_have_at_least_size(4);
    }

    #[test]
    fn should_have_at_most_size_3() {
        let collection = vec!["junit", "testify", "catch2"];
        collection.should_have_at_most_size(3);
    }

    #[test]
    #[should_panic]
    fn should_have_at_most_size_2_but_was_not() {
        let collection = vec!["junit", "testify", "catch2"];
        collection.should_have_at_most_size(2);
    }

    #[test]
    fn should_be_same_size_as_other() {
        let collection = vec!["junit", "testify", "catch2"];
        collection.should_be_same_size_as(&[1, 2, 3]);
    }

    #[test]
    #[should_panic]
    fn should_be_same_size_as_other_but_was_not() {
        let collection = vec!["junit", "testify"];
        collection.should_be_same_size_as(&[1, 2, 3]);
    }

    #[test]
    fn should_have_size_in_the_inclusive_range() {
        let collection = vec!["junit", "testify"];
        collection.should_have_size_in_inclusive_range(2..=8);
    }

    #[test]
    #[should_panic]
    fn should_have_size_in_the_inclusive_range_but_was_not() {
        let collection = vec!["junit", "testify"];
        collection.should_have_size_in_inclusive_range(3..=4);
    }

    #[test]
    fn should_not_have_size_in_the_inclusive_range() {
        let collection = vec!["junit", "testify"];
        collection.should_not_have_size_in_inclusive_range(3..=4);
    }

    #[test]
    #[should_panic]
    fn should_not_have_size_in_the_inclusive_range_but_was() {
        let collection = vec!["junit", "testify"];
        collection.should_not_have_size_in_inclusive_range(1..=2);
    }

    #[test]
    fn should_have_size_in_the_exclusive_range() {
        let collection = vec!["junit", "testify"];
        collection.should_have_size_in_exclusive_range(1..3);
    }

    #[test]
    #[should_panic]
    fn should_have_size_in_the_range_but_was_not() {
        let collection = vec!["junit", "testify"];
        collection.should_have_size_in_exclusive_range(3..8);
    }

    #[test]
    fn should_not_have_size_in_the_exclusive_range() {
        let collection = vec!["junit", "testify"];
        collection.should_not_have_size_in_exclusive_range(3..4);
    }

    #[test]
    #[should_panic]
    fn should_not_have_size_in_the_exclusive_range_but_was() {
        let collection = vec!["junit", "testify"];
        collection.should_not_have_size_in_exclusive_range(1..9);
    }
}

#[cfg(test)]
mod array_tests {
    use crate::assertions::collection::size::SizeAssertion;

    #[test]
    fn should_have_size_as_2() {
        let collection = ["junit", "testify"];
        collection.should_have_size(2);
    }

    #[test]
    #[should_panic]
    fn should_have_size_as_3_but_was_not() {
        let collection = ["junit", "testify"];
        collection.should_have_size(3);
    }

    #[test]
    fn should_not_have_size_as_3() {
        let collection = ["junit", "testify"];
        collection.should_not_have_size(3);
    }

    #[test]
    #[should_panic]
    fn should_not_have_size_as_2_but_was() {
        let collection = ["junit", "testify"];
        collection.should_not_have_size(2);
    }

    #[test]
    fn should_have_at_least_size_3() {
        let collection = ["junit", "testify", "catch2"];
        collection.should_have_at_least_size(3);
    }

    #[test]
    #[should_panic]
    fn should_have_at_least_size_4_but_was_not() {
        let collection = ["junit", "testify", "catch2"];
        collection.should_have_at_least_size(4);
    }

    #[test]
    fn should_have_at_most_size_3() {
        let collection = ["junit", "testify", "catch2"];
        collection.should_have_at_most_size(3);
    }

    #[test]
    #[should_panic]
    fn should_have_at_most_size_2_but_was_not() {
        let collection = ["junit", "testify", "catch2"];
        collection.should_have_at_most_size(2);
    }

    #[test]
    fn should_be_same_size_as_other() {
        let collection = ["junit", "testify", "catch2"];
        collection.should_be_same_size_as(&[1, 2, 3]);
    }

    #[test]
    #[should_panic]
    fn should_be_same_size_as_other_but_was_not() {
        let collection = ["junit", "testify"];
        collection.should_be_same_size_as(&[1, 2, 3]);
    }

    #[test]
    fn should_have_size_in_the_inclusive_range() {
        let collection = ["junit", "testify"];
        collection.should_have_size_in_inclusive_range(2..=8);
    }

    #[test]
    #[should_panic]
    fn should_have_size_in_the_inclusive_range_but_was_not() {
        let collection = ["junit", "testify"];
        collection.should_have_size_in_inclusive_range(3..=4);
    }

    #[test]
    fn should_not_have_size_in_the_inclusive_range() {
        let collection = ["junit", "testify"];
        collection.should_not_have_size_in_inclusive_range(3..=4);
    }

    #[test]
    #[should_panic]
    fn should_not_have_size_in_the_inclusive_range_but_was() {
        let collection = ["junit", "testify"];
        collection.should_not_have_size_in_inclusive_range(1..=2);
    }

    #[test]
    fn should_have_size_in_the_exclusive_range() {
        let collection = ["junit", "testify"];
        collection.should_have_size_in_exclusive_range(1..3);
    }

    #[test]
    #[should_panic]
    fn should_have_size_in_the_range_but_was_not() {
        let collection = ["junit", "testify"];
        collection.should_have_size_in_exclusive_range(3..8);
    }

    #[test]
    fn should_not_have_size_in_the_exclusive_range() {
        let collection = ["junit", "testify"];
        collection.should_not_have_size_in_exclusive_range(3..4);
    }

    #[test]
    #[should_panic]
    fn should_not_have_size_in_the_exclusive_range_but_was() {
        let collection = ["junit", "testify"];
        collection.should_not_have_size_in_exclusive_range(1..9);
    }
}
