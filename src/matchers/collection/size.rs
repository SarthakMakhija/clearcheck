use std::ops::{Range, RangeInclusive};

use crate::panicking::{assert_failed_binary, AssertKind};

pub trait Size {
    fn should_have_size(&self, size: usize) -> &Self;
    fn should_not_have_size(&self, size: usize) -> &Self;
    fn should_have_at_least_size(&self, size: usize) -> &Self;
    fn should_have_at_most_size(&self, size: usize) -> &Self;
    fn should_be_same_size_as<U>(&self, other: &[U]) -> &Self;
    fn should_have_size_in_inclusive_range(&self, range: RangeInclusive<usize>) -> &Self;
    fn should_not_have_size_in_inclusive_range(&self, range: RangeInclusive<usize>) -> &Self;
    fn should_have_size_in_exclusive_range(&self, range: Range<usize>) -> &Self;
    fn should_not_have_size_in_exclusive_range(&self, range: Range<usize>) -> &Self;
}

impl<T> Size for Vec<T>
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

impl<T> Size for [T]
where
    T: std::fmt::Debug,
{
    fn should_have_size(&self, size: usize) -> &Self {
        if self.len() != size {
            assert_failed_binary(AssertKind::EqualSize, self, &size);
        }
        self
    }

    fn should_not_have_size(&self, size: usize) -> &Self {
        if self.len() == size {
            assert_failed_binary(AssertKind::NotEqualSize, self, &size);
        }
        self
    }

    fn should_have_at_least_size(&self, size: usize) -> &Self {
        if !(self.len() >= size) {
            assert_failed_binary(AssertKind::AtleastSize, self, &size);
        }
        self
    }

    fn should_have_at_most_size(&self, size: usize) -> &Self {
        if !(self.len() <= size) {
            assert_failed_binary(AssertKind::AtmostSize, self, &size);
        }
        self
    }

    fn should_be_same_size_as<U>(&self, other: &[U]) -> &Self {
        if self.len() != other.len() {
            assert_failed_binary(AssertKind::EqualSize, self, &other.len());
        }
        self
    }

    fn should_have_size_in_inclusive_range(&self, range: RangeInclusive<usize>) -> &Self {
        if !range.contains(&self.len()) {
            assert_failed_binary(AssertKind::InRangeSize, self, &range);
        }
        self
    }

    fn should_not_have_size_in_inclusive_range(&self, range: RangeInclusive<usize>) -> &Self {
        if range.contains(&self.len()) {
            assert_failed_binary(AssertKind::NotInRangeSize, self, &range);
        }
        self
    }

    fn should_have_size_in_exclusive_range(&self, range: Range<usize>) -> &Self {
        if !range.contains(&self.len()) {
            assert_failed_binary(AssertKind::InRangeSize, self, &range);
        }
        self
    }

    fn should_not_have_size_in_exclusive_range(&self, range: Range<usize>) -> &Self {
        if range.contains(&self.len()) {
            assert_failed_binary(AssertKind::NotInRangeSize, self, &range);
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::matchers::collection::size::Size;

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
