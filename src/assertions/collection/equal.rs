use std::fmt::Debug;

use crate::matchers::equal::be_equal_ignoring_case;
use crate::matchers::{Should, ShouldNot};

pub trait IgnoreCaseEqualityAssertion<T> {
    fn should_be_equal_ignoring_case(&self, other: &T) -> &Self;
    fn should_not_be_equal_ignoring_case(&self, other: &T) -> &Self;
}

impl<T> IgnoreCaseEqualityAssertion<Vec<T>> for Vec<T>
where
    T: AsRef<str> + Debug + Eq,
{
    fn should_be_equal_ignoring_case(&self, other: &Vec<T>) -> &Self {
        self.should(&be_equal_ignoring_case(other));
        self
    }

    fn should_not_be_equal_ignoring_case(&self, other: &Vec<T>) -> &Self {
        self.should_not(&be_equal_ignoring_case(other));
        self
    }
}

impl<T> IgnoreCaseEqualityAssertion<&[T]> for [T]
where
    T: AsRef<str> + Debug + Eq,
{
    fn should_be_equal_ignoring_case(&self, other: &&[T]) -> &Self {
        self.should(&be_equal_ignoring_case(other));
        self
    }

    fn should_not_be_equal_ignoring_case(&self, other: &&[T]) -> &Self {
        self.should_not(&be_equal_ignoring_case(other));
        self
    }
}

impl<const N: usize> IgnoreCaseEqualityAssertion<[String; N]> for [String; N] {
    fn should_be_equal_ignoring_case(&self, other: &[String; N]) -> &Self {
        self.should(&be_equal_ignoring_case(other));
        self
    }

    fn should_not_be_equal_ignoring_case(&self, other: &[String; N]) -> &Self {
        self.should_not(&be_equal_ignoring_case(other));
        self
    }
}

impl<const N: usize> IgnoreCaseEqualityAssertion<[&str; N]> for [&str; N] {
    fn should_be_equal_ignoring_case(&self, other: &[&str; N]) -> &Self {
        self.should(&be_equal_ignoring_case(other));
        self
    }

    fn should_not_be_equal_ignoring_case(&self, other: &[&str; N]) -> &Self {
        self.should_not(&be_equal_ignoring_case(other));
        self
    }
}

#[cfg(test)]
mod vector_tests {
    use crate::assertions::collection::equal::IgnoreCaseEqualityAssertion;

    #[test]
    fn should_equal_ignoring_case() {
        let collection = vec!["junit", "Assert4rs", "gotest"];
        let other = vec!["JUNIT", "ASSERT4RS", "GoTest"];

        collection.should_be_equal_ignoring_case(&other);
    }

    #[test]
    #[should_panic]
    fn should_equal_ignoring_case_but_was_not() {
        let collection = vec!["junit", "Assert4rs", "gotest"];
        let other = vec!["JUNIT", "ASSERT", "GoTest"];

        collection.should_be_equal_ignoring_case(&other);
    }

    #[test]
    fn should_not_equal_ignoring_case() {
        let collection = vec!["junit", "Assert4rs", "gotest"];
        let other = vec!["JUNIT", "ASSERT", "GoTest"];

        collection.should_not_be_equal_ignoring_case(&other);
    }

    #[test]
    #[should_panic]
    fn should_not_equal_ignoring_case_but_was() {
        let collection = vec!["junit", "Assert4rs", "gotest"];
        let other = vec!["JUNIT", "Assert4rs", "GoTest"];

        collection.should_not_be_equal_ignoring_case(&other);
    }
}

#[cfg(test)]
mod array_tests {
    use crate::assertions::collection::equal::IgnoreCaseEqualityAssertion;

    #[test]
    fn should_equal_ignoring_case() {
        let collection = ["junit", "Assert4rs", "gotest"];
        let other = ["JUNIT", "ASSERT4RS", "GoTest"];

        collection.should_be_equal_ignoring_case(&other);
    }

    #[test]
    #[should_panic]
    fn should_equal_ignoring_case_but_was_not() {
        let collection = ["junit", "Assert4rs", "gotest"];
        let other = ["JUNIT", "ASSERT", "GoTest"];

        collection.should_be_equal_ignoring_case(&other);
    }

    #[test]
    fn should_not_equal_ignoring_case() {
        let collection = ["junit", "Assert4rs", "gotest"];
        let other = ["JUNIT", "ASSERT", "GoTest"];

        collection.should_not_be_equal_ignoring_case(&other);
    }

    #[test]
    #[should_panic]
    fn should_not_equal_ignoring_case_but_was() {
        let collection = ["junit", "Assert4rs", "gotest"];
        let other = ["JUNIT", "Assert4rs", "GoTest"];

        collection.should_not_be_equal_ignoring_case(&other);
    }
}

#[cfg(test)]
mod slice_tests {
    use crate::assertions::collection::equal::IgnoreCaseEqualityAssertion;

    #[test]
    fn should_equal_ignoring_case() {
        let collection: &[&str] = &["junit", "Assert4rs", "gotest"];
        let other: &[&str] = &["JUNIT", "ASSERT4RS", "GoTest"];

        collection.should_be_equal_ignoring_case(&other);
    }

    #[test]
    #[should_panic]
    fn should_equal_ignoring_case_but_was_not() {
        let collection: &[&str] = &["junit", "Assert4rs", "gotest"];
        let other: &[&str] = &["JUNIT", "ASSERT", "GoTest"];

        collection.should_be_equal_ignoring_case(&other);
    }

    #[test]
    fn should_not_equal_ignoring_case() {
        let collection: &[&str] = &["junit", "Assert4rs", "gotest"];
        let other: &[&str] = &["JUNIT", "ASSERT", "GoTest"];

        collection.should_not_be_equal_ignoring_case(&other);
    }

    #[test]
    #[should_panic]
    fn should_not_equal_ignoring_case_but_was() {
        let collection: &[&str] = &["junit", "Assert4rs", "gotest"];
        let other: &[&str] = &["JUNIT", "Assert4rs", "GoTest"];

        collection.should_not_be_equal_ignoring_case(&other);
    }
}