use std::fmt::Debug;
use std::marker::PhantomData;

use crate::matchers::{Matcher, MatcherResult};

/// PredicateMatcher offers a flexible way to assert whether the elements in a collection satisfy the given predicate.
///
/// clearcheck implements PredicateMatcher for collection types including vector, arrays and reference to slices.
///
/// # Example
///```
/// use clearcheck::matchers::collection::predicate::satisfy_for_any;
/// use clearcheck::matchers::Matcher;
///
/// let collection = vec!["junit", "testify", "xunit"];
/// let matcher = satisfy_for_any(|element: &&str| element.len() > 6);
///
/// assert!(matcher.test(&collection).passed());
/// ```
pub enum PredicateMatcher<F, T>
    where F: Fn(&T) -> bool,
          T: Eq
{
    SatisfyAny(F, PhantomData<T>),
    SatisfyAll(F, PhantomData<T>),
}

impl<F, T> PredicateMatcher<F, T>
    where F: Fn(&T) -> bool,
          T: Eq + Debug
{
    fn test(&self, collection: &[T]) -> MatcherResult {
        match self {
            PredicateMatcher::SatisfyAny(predicate, _) =>
                MatcherResult::formatted(
                    collection.iter().any(predicate),
                    format!("{:?} should satisfy the given predicate for any of the elements", collection),
                    format!("{:?} should not satisfy the given predicate for any of the elements", collection),
                ),
            PredicateMatcher::SatisfyAll(predicate, _) =>
                MatcherResult::formatted(
                    collection.iter().all(predicate),
                    format!("{:?} should satisfy the given predicate for all the elements", collection),
                    format!("{:?} should not satisfy the given predicate for all the elements", collection),
                ),
        }
    }
}

impl<F, T> Matcher<Vec<T>> for PredicateMatcher<F, T>
    where
        F: Fn(&T) -> bool,
        T: Eq + Debug,
{
    fn test(&self, collection: &Vec<T>) -> MatcherResult {
        self.test(collection)
    }
}

impl<F, T, const N: usize> Matcher<[T; N]> for PredicateMatcher<F, T>
    where
        F: Fn(&T) -> bool,
        T: Eq + Debug,
{
    fn test(&self, collection: &[T; N]) -> MatcherResult {
        self.test(collection)
    }
}

impl<F, T> Matcher<&[T]> for PredicateMatcher<F, T>
    where
        F: Fn(&T) -> bool,
        T: Eq + Debug,
{
    fn test(&self, collection: &&[T]) -> MatcherResult {
        self.test(collection)
    }
}

/// Creates a PredicateMatcher that asserts whether any of the elements in a collection satisfy the given predicate.
pub fn satisfy_for_any<F, T>(predicate: F) -> PredicateMatcher<F, T>
    where
        F: Fn(&T) -> bool,
        T: Eq + Debug,
{
    PredicateMatcher::SatisfyAny(predicate, PhantomData)
}

/// Creates a PredicateMatcher that asserts whether all the elements in a collection satisfy the given predicate.
pub fn satisfy_for_all<F, T>(predicate: F) -> PredicateMatcher<F, T>
    where
        F: Fn(&T) -> bool,
        T: Eq + Debug,
{
    PredicateMatcher::SatisfyAll(predicate, PhantomData)
}

#[cfg(test)]
mod tests {
    use crate::assertions::bool::TrueFalseAssertion;
    use crate::matchers::collection::predicate::{satisfy_for_all, satisfy_for_any};

    #[test]
    fn should_satisfy_for_any() {
        let collection = vec!["junit", "testify", "xunit"];
        let matcher = satisfy_for_any(|element: &&str| element.len() > 6);
        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_satisfy_for_any_but_id_did_not() {
        let collection = vec!["unit4j", "testify"];
        let matcher = satisfy_for_any(|element: &&str| element.starts_with("clear"));
        matcher.test(&collection).passed.should_be_true();
    }
    #[test]
    fn should_satisfy_for_all() {
        let collection = vec!["junit", "testify"];
        let matcher = satisfy_for_all(|element: &&str| element.len() > 3);
        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_satisfy_for_all_but_id_did_not() {
        let collection = vec!["unit4j", "testify"];
        let matcher = satisfy_for_all(|element: &&str| element.starts_with("clear"));
        matcher.test(&collection).passed.should_be_true();
    }
}
