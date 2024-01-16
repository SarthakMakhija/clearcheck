use std::fmt::Debug;

use crate::matchers::collection::min_max::{have_max, have_min};
use crate::matchers::{Should, ShouldNot};

pub trait MinMaxAssertion<T: Ord> {
    fn should_have_min(&self, min: T) -> &Self;

    fn should_not_have_min(&self, min: T) -> &Self;

    fn should_have_max(&self, max: T) -> &Self;

    fn should_not_have_max(&self, min: T) -> &Self;
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

    fn should_not_have_max(&self, min: T) -> &Self {
        (self as &[T]).should_not_have_max(min);
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

    fn should_not_have_max(&self, min: T) -> &Self {
        (self as &[T]).should_not_have_max(min);
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

    fn should_not_have_max(&self, min: T) -> &Self {
        self.should_not(&have_max(min));
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