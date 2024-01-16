use std::fmt::Debug;
use std::ops::{Range, RangeInclusive};

use crate::matchers::{Should, ShouldNot};
use crate::matchers::collection::min_max::{have_max, have_max_in_exclusive_range, have_max_in_inclusive_range, have_min, have_min_in_exclusive_range, have_min_in_inclusive_range};

pub trait MinMaxAssertion<T: Ord> {
    fn should_have_min(&self, min: T) -> &Self;

    fn should_not_have_min(&self, min: T) -> &Self;

    fn should_have_max(&self, max: T) -> &Self;

    fn should_not_have_max(&self, max: T) -> &Self;

    fn should_have_min_in_inclusive_range(&self, range: RangeInclusive<T>) -> &Self;

    fn should_not_have_min_in_inclusive_range(&self, range: RangeInclusive<T>) -> &Self;

    fn should_have_min_in_exclusive_range(&self, range: Range<T>) -> &Self;

    fn should_not_have_min_in_exclusive_range(&self, range: Range<T>) -> &Self;

    fn should_have_max_in_inclusive_range(&self, range: RangeInclusive<T>) -> &Self;

    fn should_not_have_max_in_inclusive_range(&self, range: RangeInclusive<T>) -> &Self;

    fn should_have_max_in_exclusive_range(&self, range: Range<T>) -> &Self;

    fn should_not_have_max_in_exclusive_range(&self, range: Range<T>) -> &Self;
}

impl<T> MinMaxAssertion<T> for Vec<T>
    where T: Ord + Debug
{
    fn should_have_min(&self, min: T) -> &Self {
        (self as &[T]).should_have_min(min);
        self
    }

    fn should_not_have_min(&self, min: T) -> &Self {
        (self as &[T]).should_not_have_min(min);
        self
    }

    fn should_have_max(&self, max: T) -> &Self {
        (self as &[T]).should_have_max(max);
        self
    }

    fn should_not_have_max(&self, max: T) -> &Self {
        (self as &[T]).should_not_have_max(max);
        self
    }

    fn should_have_min_in_inclusive_range(&self, range: RangeInclusive<T>) -> &Self {
        (self as &[T]).should_have_min_in_inclusive_range(range);
        self
    }

    fn should_not_have_min_in_inclusive_range(&self, range: RangeInclusive<T>) -> &Self {
        (self as &[T]).should_not_have_min_in_inclusive_range(range);
        self
    }

    fn should_have_min_in_exclusive_range(&self, range: Range<T>) -> &Self {
        (self as &[T]).should_have_min_in_exclusive_range(range);
        self
    }

    fn should_not_have_min_in_exclusive_range(&self, range: Range<T>) -> &Self {
        (self as &[T]).should_not_have_min_in_exclusive_range(range);
        self
    }

    fn should_have_max_in_inclusive_range(&self, range: RangeInclusive<T>) -> &Self {
        (self as &[T]).should_have_max_in_inclusive_range(range);
        self
    }

    fn should_not_have_max_in_inclusive_range(&self, range: RangeInclusive<T>) -> &Self {
        (self as &[T]).should_not_have_max_in_inclusive_range(range);
        self
    }

    fn should_have_max_in_exclusive_range(&self, range: Range<T>) -> &Self {
        (self as &[T]).should_have_max_in_exclusive_range(range);
        self
    }

    fn should_not_have_max_in_exclusive_range(&self, range: Range<T>) -> &Self {
        (self as &[T]).should_not_have_max_in_exclusive_range(range);
        self
    }
}

impl<T, const N: usize> MinMaxAssertion<T> for [T; N]
    where T: Ord + Debug
{
    fn should_have_min(&self, min: T) -> &Self {
        (self as &[T]).should_have_min(min);
        self
    }

    fn should_not_have_min(&self, min: T) -> &Self {
        (self as &[T]).should_not_have_min(min);
        self
    }

    fn should_have_max(&self, max: T) -> &Self {
        (self as &[T]).should_have_max(max);
        self
    }

    fn should_not_have_max(&self, max: T) -> &Self {
        (self as &[T]).should_not_have_max(max);
        self
    }

    fn should_have_min_in_inclusive_range(&self, range: RangeInclusive<T>) -> &Self {
        (self as &[T]).should_have_min_in_inclusive_range(range);
        self
    }

    fn should_not_have_min_in_inclusive_range(&self, range: RangeInclusive<T>) -> &Self {
        (self as &[T]).should_not_have_min_in_inclusive_range(range);
        self
    }

    fn should_have_min_in_exclusive_range(&self, range: Range<T>) -> &Self {
        (self as &[T]).should_have_min_in_exclusive_range(range);
        self
    }

    fn should_not_have_min_in_exclusive_range(&self, range: Range<T>) -> &Self {
        (self as &[T]).should_not_have_min_in_exclusive_range(range);
        self
    }

    fn should_have_max_in_inclusive_range(&self, range: RangeInclusive<T>) -> &Self {
        (self as &[T]).should_have_max_in_inclusive_range(range);
        self
    }

    fn should_not_have_max_in_inclusive_range(&self, range: RangeInclusive<T>) -> &Self {
        (self as &[T]).should_not_have_max_in_inclusive_range(range);
        self
    }

    fn should_have_max_in_exclusive_range(&self, range: Range<T>) -> &Self {
        (self as &[T]).should_have_max_in_exclusive_range(range);
        self
    }

    fn should_not_have_max_in_exclusive_range(&self, range: Range<T>) -> &Self {
        (self as &[T]).should_not_have_max_in_exclusive_range(range);
        self
    }
}

impl<T> MinMaxAssertion<T> for [T]
    where T: Ord + Debug
{
    fn should_have_min(&self, min: T) -> &Self {
        self.should(&have_min(min));
        self
    }

    fn should_not_have_min(&self, min: T) -> &Self {
        self.should_not(&have_min(min));
        self
    }

    fn should_have_max(&self, max: T) -> &Self {
        self.should(&have_max(max));
        self
    }

    fn should_not_have_max(&self, max: T) -> &Self {
        self.should_not(&have_max(max));
        self
    }

    fn should_have_min_in_inclusive_range(&self, range: RangeInclusive<T>) -> &Self {
        self.should(&have_min_in_inclusive_range(range));
        self
    }

    fn should_not_have_min_in_inclusive_range(&self, range: RangeInclusive<T>) -> &Self {
        self.should_not(&have_min_in_inclusive_range(range));
        self
    }

    fn should_have_min_in_exclusive_range(&self, range: Range<T>) -> &Self {
        self.should(&have_min_in_exclusive_range(range));
        self
    }

    fn should_not_have_min_in_exclusive_range(&self, range: Range<T>) -> &Self {
        self.should_not(&have_min_in_exclusive_range(range));
        self
    }

    fn should_have_max_in_inclusive_range(&self, range: RangeInclusive<T>) -> &Self {
        self.should(&have_max_in_inclusive_range(range));
        self
    }

    fn should_not_have_max_in_inclusive_range(&self, range: RangeInclusive<T>) -> &Self {
        self.should_not(&have_max_in_inclusive_range(range));
        self
    }

    fn should_have_max_in_exclusive_range(&self, range: Range<T>) -> &Self {
        self.should(&have_max_in_exclusive_range(range));
        self
    }

    fn should_not_have_max_in_exclusive_range(&self, range: Range<T>) -> &Self {
        self.should_not(&have_max_in_exclusive_range(range));
        self
    }
}


#[cfg(test)]
mod tests {
    use crate::assertions::collection::min_max::MinMaxAssertion;

    #[test]
    fn should_have_a_min_element() {
        let collection = vec!["assert", "clearcheck", "junit"];
        collection.should_have_min("assert");
    }

    #[test]
    #[should_panic]
    fn should_have_the_given_min_element_but_was_not() {
        let collection = vec!["assert", "clearcheck", "junit"];
        collection.should_have_min("clearcheck");
    }

    #[test]
    fn should_not_have_a_min_element_in_an_empty_collection() {
        let collection: Vec<&str> = vec![];
        collection.should_not_have_min("assert");
    }

    #[test]
    #[should_panic]
    fn should_not_have_the_given_min_element_but_was() {
        let collection: Vec<&str> = vec!["assert", "clearcheck", "junit"];
        collection.should_not_have_min("assert");
    }

    #[test]
    fn should_have_a_max_element() {
        let collection = vec!["assert", "clearcheck", "junit"];
        collection.should_have_max("junit");
    }

    #[test]
    #[should_panic]
    fn should_have_the_given_max_element_but_was_not() {
        let collection = vec!["assert", "clearcheck", "junit"];
        collection.should_have_max("clearcheck");
    }

    #[test]
    fn should_not_have_a_max_element_in_an_empty_collection() {
        let collection: Vec<&str> = vec![];
        collection.should_not_have_max("assert");
    }

    #[test]
    #[should_panic]
    fn should_not_have_the_given_max_element_but_was() {
        let collection: Vec<&str> = vec!["assert", "clearcheck", "junit"];
        collection.should_not_have_max("junit");
    }
}

#[cfg(test)]
mod range_tests {
    use crate::assertions::collection::min_max::MinMaxAssertion;

    #[test]
    fn should_have_a_min_in_inclusive_range() {
        let collection = vec!["assert", "clearcheck", "junit"];
        collection.should_have_min_in_inclusive_range("assert"..="junit");
    }

    #[test]
    #[should_panic]
    fn should_have_a_min_in_inclusive_range_but_was_not() {
        let collection = vec!["assert", "clearcheck", "junit"];
        collection.should_have_min_in_inclusive_range("clearcheck"..="junit");
    }

    #[test]
    fn should_have_a_min_in_exclusive_range() {
        let collection = vec!["assert", "clearcheck", "junit"];
        collection.should_have_min_in_exclusive_range("assert".."junit");
    }

    #[test]
    #[should_panic]
    fn should_have_a_min_in_exclusive_range_but_was_not() {
        let collection = vec!["assert", "clearcheck", "junit"];
        collection.should_have_min_in_exclusive_range("clearcheck".."junit");
    }

    #[test]
    fn should_have_a_max_in_inclusive_range() {
        let collection = vec!["assert", "clearcheck", "junit"];
        collection.should_have_max_in_inclusive_range("assert"..="junit");
    }

    #[test]
    #[should_panic]
    fn should_have_a_max_in_inclusive_range_but_was_not() {
        let collection = vec!["assert", "clearcheck", "junit"];
        collection.should_have_max_in_inclusive_range("clearcheck"..="gotest");
    }

    #[test]
    fn should_have_a_max_in_exclusive_range() {
        let collection = vec!["assert", "clearcheck", "junit"];
        collection.should_have_max_in_exclusive_range("assert".."testify");
    }

    #[test]
    #[should_panic]
    fn should_have_a_max_in_exclusive_range_but_was_not() {
        let collection = vec!["assert", "clearcheck", "junit"];
        collection.should_have_max_in_exclusive_range("clearcheck".."junit");
    }

    #[test]
    fn should_not_have_a_min_in_inclusive_range() {
        let collection = vec!["assert", "clearcheck", "junit"];
        collection.should_not_have_min_in_inclusive_range("clearcheck"..="junit");
    }

    #[test]
    #[should_panic]
    fn should_not_have_a_min_in_inclusive_range_but_was() {
        let collection = vec!["assert", "clearcheck", "junit"];
        collection.should_not_have_min_in_inclusive_range("assert"..="junit");
    }

    #[test]
    fn should_not_have_a_min_in_exclusive_range() {
        let collection = vec!["assert", "clearcheck", "junit"];
        collection.should_not_have_min_in_exclusive_range("clearcheck".."junit");
    }

    #[test]
    #[should_panic]
    fn should_not_have_a_min_in_exclusive_range_but_was() {
        let collection = vec!["assert", "clearcheck", "junit"];
        collection.should_not_have_min_in_exclusive_range("assert".."testify");
    }

    #[test]
    fn should_not_have_a_max_in_inclusive_range() {
        let collection = vec!["assert", "clearcheck", "junit"];
        collection.should_not_have_max_in_inclusive_range("clearcheck"..="gotest");
    }

    #[test]
    #[should_panic]
    fn should_not_have_a_max_in_inclusive_range_but_was() {
        let collection = vec!["assert", "clearcheck", "junit"];
        collection.should_not_have_max_in_inclusive_range("assert"..="junit");
    }

    #[test]
    fn should_not_have_a_max_in_exclusive_range() {
        let collection = vec!["assert", "clearcheck", "junit"];
        collection.should_not_have_max_in_exclusive_range("assert".."junit");
    }

    #[test]
    #[should_panic]
    fn should_not_have_a_max_in_exclusive_range_but_was() {
        let collection = vec!["assert", "clearcheck", "junit"];
        collection.should_not_have_max_in_exclusive_range("assert".."testify");
    }
}