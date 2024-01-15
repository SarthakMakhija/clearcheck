use std::fmt::Debug;

use crate::matchers::collection::duplicate::contain_duplicates;
use crate::matchers::{Should, ShouldNot};

/// DuplicateContentAssertion enables assertions about whether a collection contains duplicate elements.
pub trait DuplicateContentAssertion<T: Eq> {
    /// - Asserts that the collection contains atleast one duplicate element.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::collection::duplicate::DuplicateContentAssertion;
    ///
    /// let collection = ["junit", "testify", "clearcheck", "testify"];
    /// collection.should_contain_duplicates();
    /// ```
    fn should_contain_duplicates(&self) -> &Self;

    /// - Asserts that the collection does not contain any duplicate element.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::collection::duplicate::DuplicateContentAssertion;
    ///
    /// let collection = ["junit", "testify", "clearcheck"];
    /// collection.should_not_contain_duplicates();
    /// ```
    fn should_not_contain_duplicates(&self) -> &Self;
}

impl<T> DuplicateContentAssertion<T> for Vec<T>
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

impl<T, const N: usize> DuplicateContentAssertion<T> for [T; N]
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

impl<T> DuplicateContentAssertion<T> for [T]
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

#[cfg(test)]
mod array_tests {
    use crate::assertions::collection::duplicate::DuplicateContentAssertion;

    #[test]
    fn should_contain_duplicates() {
        let collection = ["junit", "testify", "assert4j", "testify"];
        collection.should_contain_duplicates();
    }

    #[test]
    #[should_panic]
    fn should_contain_duplicates_but_it_did_not() {
        let collection = ["junit", "testify", "assert4j", ""];
        collection.should_contain_duplicates();
    }

    #[test]
    fn should_not_contain_duplicates() {
        let collection = ["junit", "testify", "assert4j", "catch"];
        collection.should_not_contain_duplicates();
    }

    #[test]
    #[should_panic]
    fn should_not_contain_duplicates_but_it_contained() {
        let collection = ["junit", "testify", "assert4j", "testify"];
        collection.should_not_contain_duplicates();
    }
}
