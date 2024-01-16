use std::fmt::Debug;

use crate::matchers::{Should, ShouldNot};
use crate::matchers::collection::predicate::{satisfy_for_all, satisfy_for_any};

pub trait PredicateAssertion<T>
    where
        T: Eq
{
    fn should_satisfy_for_all<F>(&self, predicate: F) -> &Self
        where
            F: Fn(&T) -> bool;

    fn should_not_satisfy_for_all<F>(&self, predicate: F) -> &Self
        where
            F: Fn(&T) -> bool;

    fn should_satisfy_for_any<F>(&self, predicate: F) -> &Self
        where
            F: Fn(&T) -> bool;

    fn should_not_satisfy_for_any<F>(&self, predicate: F) -> &Self
        where
            F: Fn(&T) -> bool;
}

impl<T> PredicateAssertion<T> for Vec<T>
    where
        T: Debug,
        T: Eq,
{
    fn should_satisfy_for_all<F>(&self, predicate: F) -> &Self where F: Fn(&T) -> bool {
        (self as &[T]).should_satisfy_for_all(predicate);
        self
    }

    fn should_not_satisfy_for_all<F>(&self, predicate: F) -> &Self where F: Fn(&T) -> bool {
        (self as &[T]).should_not_satisfy_for_all(predicate);
        self
    }

    fn should_satisfy_for_any<F>(&self, predicate: F) -> &Self where F: Fn(&T) -> bool {
        (self as &[T]).should_satisfy_for_any(predicate);
        self
    }

    fn should_not_satisfy_for_any<F>(&self, predicate: F) -> &Self where F: Fn(&T) -> bool {
        (self as &[T]).should_not_satisfy_for_any(predicate);
        self
    }
}

impl<T, const N: usize> PredicateAssertion<T> for [T; N]
    where
        T: Debug,
        T: Eq,
{
    fn should_satisfy_for_all<F>(&self, predicate: F) -> &Self where F: Fn(&T) -> bool {
        (self as &[T]).should_satisfy_for_all(predicate);
        self
    }

    fn should_not_satisfy_for_all<F>(&self, predicate: F) -> &Self where F: Fn(&T) -> bool {
        (self as &[T]).should_not_satisfy_for_all(predicate);
        self
    }

    fn should_satisfy_for_any<F>(&self, predicate: F) -> &Self where F: Fn(&T) -> bool {
        (self as &[T]).should_satisfy_for_any(predicate);
        self
    }

    fn should_not_satisfy_for_any<F>(&self, predicate: F) -> &Self where F: Fn(&T) -> bool {
        (self as &[T]).should_not_satisfy_for_any(predicate);
        self
    }
}

impl<T> PredicateAssertion<T> for [T]
    where
        T: Debug,
        T: Eq,
{
    fn should_satisfy_for_all<F>(&self, predicate: F) -> &Self where F: Fn(&T) -> bool {
        self.should(&satisfy_for_all(predicate));
        self
    }

    fn should_not_satisfy_for_all<F>(&self, predicate: F) -> &Self where F: Fn(&T) -> bool {
        self.should_not(&satisfy_for_all(predicate));
        self
    }

    fn should_satisfy_for_any<F>(&self, predicate: F) -> &Self where F: Fn(&T) -> bool {
        self.should(&satisfy_for_any(predicate));
        self
    }

    fn should_not_satisfy_for_any<F>(&self, predicate: F) -> &Self where F: Fn(&T) -> bool {
        self.should_not(&satisfy_for_any(predicate));
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::assertions::collection::predicate::PredicateAssertion;

    #[test]
    fn should_satisfy_for_all_a_character_must_be_numeric() {
        let collection = vec!["clearcheck-1", "junit-2", "assert-3"];
        collection.should_satisfy_for_all(|element| element.chars().any(|ch| ch.is_numeric()));
    }

    #[test]
    #[should_panic]
    fn should_satisfy_for_all_a_character_must_be_numeric_but_it_did_not() {
        let collection = vec!["clearcheck", "junit-2", "assert-3"];
        collection.should_satisfy_for_all(|element| element.chars().any(|ch| ch.is_numeric()));
    }

    #[test]
    fn should_not_satisfy_for_all_a_character_must_be_numeric() {
        let collection = vec!["clearcheck", "junit-2", "assert-3"];
        collection.should_not_satisfy_for_all(|element| element.chars().any(|ch| ch.is_numeric()));
    }

    #[test]
    #[should_panic]
    fn should_not_satisfy_for_all_a_character_must_be_numeric_but_it_did() {
        let collection = vec!["clearcheck-1", "junit-2", "assert-3"];
        collection.should_not_satisfy_for_all(|element| element.chars().any(|ch| ch.is_numeric()));
    }

    #[test]
    fn should_satisfy_for_any_a_character_must_be_numeric() {
        let collection = vec!["clearcheck-1", "junit", "assert-3"];
        collection.should_satisfy_for_any(|element| element.chars().any(|ch| ch.is_numeric()));
    }

    #[test]
    #[should_panic]
    fn should_satisfy_for_any_a_character_must_be_numeric_but_it_did_not() {
        let collection = vec!["clearcheck", "junit", "assert"];
        collection.should_satisfy_for_any(|element| element.chars().any(|ch| ch.is_numeric()));
    }

    #[test]
    fn should_not_satisfy_for_any_a_character_must_be_numeric() {
        let collection = vec!["clearcheck", "junit", "assert"];
        collection.should_not_satisfy_for_any(|element| element.chars().any(|ch| ch.is_numeric()));
    }

    #[test]
    #[should_panic]
    fn should_not_satisfy_for_any_a_character_must_be_numeric_but_it_did() {
        let collection = vec!["clearcheck", "junit-2", "assert"];
        collection.should_not_satisfy_for_any(|element| element.chars().any(|ch| ch.is_numeric()));
    }
}