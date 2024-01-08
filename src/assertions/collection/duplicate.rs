use std::fmt::Debug;

use crate::matchers::collection::duplicate::contain_duplicates;
use crate::matchers::{Should, ShouldNot};

pub trait DuplicateContentAssertion {
    fn should_contain_duplicates(&self) -> &Self;
    fn should_not_contain_duplicates(&self) -> &Self;
}

impl<T> DuplicateContentAssertion for Vec<T>
where
    T: Debug,
    T: Eq,
{
    fn should_contain_duplicates(&self) -> &Self {
        (self as &[T]).should_contain_duplicates();
        self
    }

    fn should_not_contain_duplicates(&self) -> &Self {
        (self as &[T]).should_not_contain_duplicates();
        self
    }
}

impl<T, const N: usize> DuplicateContentAssertion for [T; N]
where
    T: Debug,
    T: Eq,
{
    fn should_contain_duplicates(&self) -> &Self {
        (self as &[T]).should_contain_duplicates();
        self
    }

    fn should_not_contain_duplicates(&self) -> &Self {
        (self as &[T]).should_not_contain_duplicates();
        self
    }
}

impl<T> DuplicateContentAssertion for [T]
where
    T: Debug,
    T: Eq,
{
    fn should_contain_duplicates(&self) -> &Self {
        self.should(&contain_duplicates());
        self
    }

    fn should_not_contain_duplicates(&self) -> &Self {
        self.should_not(&contain_duplicates());
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::assertions::collection::duplicate::DuplicateContentAssertion;

    #[test]
    fn should_contain_duplicates() {
        let collection = vec!["junit", "testify", "assert4j", "testify"];
        collection.should_contain_duplicates();
    }

    #[test]
    #[should_panic]
    fn should_contain_duplicates_but_it_did_not() {
        let collection = vec!["junit", "testify", "assert4j", ""];
        collection.should_contain_duplicates();
    }

    #[test]
    fn should_not_contain_duplicates() {
        let collection = vec!["junit", "testify", "assert4j", "catch"];
        collection.should_not_contain_duplicates();
    }

    #[test]
    #[should_panic]
    fn should_not_contain_duplicates_but_it_contained() {
        let collection = vec!["junit", "testify", "assert4j", "testify"];
        collection.should_not_contain_duplicates();
    }
}
