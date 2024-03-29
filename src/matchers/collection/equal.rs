//! provides [IgnoreCaseEqualityMatcher] for collection of elements where the elements can be represented as strings.

use std::collections::HashSet;
use std::fmt::Debug;

use crate::matchers::equal::IgnoreCaseEqualityMatcher;
use crate::matchers::{Matcher, MatcherResult};

impl<const N: usize> Matcher<[String; N]> for IgnoreCaseEqualityMatcher<[String; N]> {
    fn test(&self, collection: &[String; N]) -> MatcherResult {
        let one: HashSet<_> = collection
            .iter()
            .map(|source| source.to_lowercase())
            .collect();
        let other: HashSet<_> = self
            .other
            .iter()
            .map(|source| source.to_lowercase())
            .collect();

        MatcherResult::formatted(
            one == other,
            format!("{:?} should equal {:?}", collection, self.other),
            format!("{:?} should not equal {:?}", collection, self.other),
        )
    }
}

impl<const N: usize> Matcher<[&str; N]> for IgnoreCaseEqualityMatcher<[&str; N]> {
    fn test(&self, collection: &[&str; N]) -> MatcherResult {
        let one: HashSet<_> = collection
            .iter()
            .map(|source| source.to_lowercase())
            .collect();
        let other: HashSet<_> = self
            .other
            .iter()
            .map(|source| source.to_lowercase())
            .collect();

        MatcherResult::formatted(
            one == other,
            format!("{:?} should equal {:?}", collection, self.other),
            format!("{:?} should not equal {:?}", collection, self.other),
        )
    }
}

impl<T> Matcher<Vec<T>> for IgnoreCaseEqualityMatcher<Vec<T>>
where
    T: AsRef<str> + Debug + Eq,
{
    fn test(&self, collection: &Vec<T>) -> MatcherResult {
        let one: HashSet<_> = collection
            .iter()
            .map(|source| source.as_ref().to_lowercase())
            .collect();
        let other: HashSet<_> = self
            .other
            .iter()
            .map(|source| source.as_ref().to_lowercase())
            .collect();

        MatcherResult::formatted(
            one == other,
            format!("{:?} should equal {:?}", collection, self.other),
            format!("{:?} should not equal {:?}", collection, self.other),
        )
    }
}

impl<T> Matcher<&[T]> for IgnoreCaseEqualityMatcher<&[T]>
where
    T: AsRef<str> + Debug + Eq,
{
    fn test(&self, collection: &&[T]) -> MatcherResult {
        let one: HashSet<_> = collection
            .iter()
            .map(|source| source.as_ref().to_lowercase())
            .collect();
        let other: HashSet<_> = self
            .other
            .iter()
            .map(|source| source.as_ref().to_lowercase())
            .collect();

        MatcherResult::formatted(
            one == other,
            format!("{:?} should equal {:?}", collection, self.other),
            format!("{:?} should not equal {:?}", collection, self.other),
        )
    }
}

#[cfg(test)]
mod vector_tests {
    use crate::assertions::bool::TrueFalseAssertion;
    use crate::matchers::equal::be_equal_ignoring_case;
    use crate::matchers::Matcher;

    #[test]
    fn should_equal_ignoring_case() {
        let collection = vec!["junit", "clearcheck", "gotest"];
        let other = vec!["JUNIT", "clearcheck", "GoTest"];

        let matcher = be_equal_ignoring_case(other);
        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_equal_ignoring_case_but_was_not() {
        let collection = vec!["junit", "clearcheck", "gotest"];
        let other = vec!["JUNIT", "ASSERT", "GoTest"];

        let matcher = be_equal_ignoring_case(other);
        matcher.test(&collection).passed.should_be_true();
    }
}

#[cfg(test)]
mod array_tests {
    use crate::assertions::bool::TrueFalseAssertion;
    use crate::matchers::equal::be_equal_ignoring_case;
    use crate::matchers::Matcher;

    #[test]
    fn should_equal_ignoring_case() {
        let collection = ["junit", "clearcheck", "gotest"];
        let other = ["JUNIT", "clearcheck", "GoTest"];

        let matcher = be_equal_ignoring_case(other);
        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_equal_ignoring_case_but_was_not() {
        let collection = ["junit", "clearcheck", "gotest"];
        let other = ["JUNIT", "ASSERT", "GoTest"];

        let matcher = be_equal_ignoring_case(other);
        matcher.test(&collection).passed.should_be_true();
    }
}

#[cfg(test)]
mod slice_tests {
    use crate::assertions::bool::TrueFalseAssertion;
    use crate::matchers::equal::be_equal_ignoring_case;
    use crate::matchers::Matcher;

    #[test]
    fn should_equal_ignoring_case() {
        let collection: &[&str] = &["junit", "clearcheck", "gotest"];
        let other: &[&str] = &["JUNIT", "clearcheck", "GoTest"];

        let matcher = be_equal_ignoring_case(other);
        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_equal_ignoring_case_but_was_not() {
        let collection: &[&str] = &["junit", "clearcheck", "gotest"];
        let other: &[&str] = &["JUNIT", "ASSERT", "GoTest"];

        let matcher = be_equal_ignoring_case(other);
        matcher.test(&collection).passed.should_be_true();
    }
}
