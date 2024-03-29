use std::fmt::Debug;

use crate::matchers::{Matcher, MatcherResult};

/// MembershipMatcher offers a flexible way to assert the presence or absence of specific elements within collections.
///
/// Works with any data type that implements the Eq and Debug trait.
///
/// clearcheck implements MembershipMatcher for collection types including vector, arrays and reference to slices.
///
/// # Example
///```
/// use clearcheck::matchers::collection::membership::contain_all;
/// use clearcheck::matchers::Matcher;
///
/// let collection = vec!["clearcheck", "testify", "assert4j", "xunit"];
/// let all_to_be_contained = vec!["testify", "assert4j", "xunit"];
///
/// let matcher = contain_all(all_to_be_contained);
///
/// assert!(matcher.test(&collection).passed());
/// ```
pub enum MembershipMatcher<T: Eq> {
    Contain(T),
    ContainAll(Vec<T>),
    ContainAny(Vec<T>),
}

impl<T: Eq + Debug> MembershipMatcher<T> {
    fn test(&self, collection: &[T]) -> MatcherResult {
        match self {
            MembershipMatcher::Contain(element) => MatcherResult::formatted(
                collection.contains(element),
                format!("{:?} should contain {:?}", collection, element),
                format!("{:?} should not contain {:?}", collection, element),
            ),
            MembershipMatcher::ContainAll(target) => {
                let missing = target
                    .iter()
                    .filter(|element| !collection.contains(element))
                    .collect::<Vec<_>>();

                MatcherResult::formatted(
                    missing.is_empty(),
                    format!(
                        "{:?} should contain {:?} but was missing {:?}",
                        collection, target, missing
                    ),
                    format!("{:?} should not contain {:?}", collection, target),
                )
            }
            MembershipMatcher::ContainAny(target) => MatcherResult::formatted(
                target.iter().any(|source| collection.contains(source)),
                format!("{:?} should contain any of {:?}", collection, target),
                format!("{:?} should not contain any of {:?}", collection, target),
            ),
        }
    }
}

impl<T> Matcher<Vec<T>> for MembershipMatcher<T>
    where
        T: Eq + Debug,
{
    fn test(&self, collection: &Vec<T>) -> MatcherResult {
        self.test(collection)
    }
}

impl<T, const N: usize> Matcher<[T; N]> for MembershipMatcher<T>
    where
        T: Eq + Debug,
{
    fn test(&self, collection: &[T; N]) -> MatcherResult {
        self.test(collection as &[T])
    }
}

impl<T> Matcher<&[T]> for MembershipMatcher<T>
    where
        T: Eq + Debug,
{
    fn test(&self, collection: &&[T]) -> MatcherResult {
        self.test(collection)
    }
}

/// Creates a MembershipMatcher that asserts whether a collection contains the given element.
pub fn contain<T>(element: T) -> MembershipMatcher<T>
    where
        T: Eq + Debug,
{
    MembershipMatcher::Contain(element)
}

/// Creates a MembershipMatcher that asserts whether a collection contains all the given elements.
pub fn contain_all<T>(elements: Vec<T>) -> MembershipMatcher<T>
    where
        T: Eq + Debug,
{
    MembershipMatcher::ContainAll(elements)
}

/// Creates a MembershipMatcher that asserts whether a collection contains any of the given elements.
pub fn contain_any<T>(elements: Vec<T>) -> MembershipMatcher<T>
    where
        T: Eq + Debug,
{
    MembershipMatcher::ContainAny(elements)
}

#[cfg(test)]
mod tests {
    use crate::assertions::bool::TrueFalseAssertion;
    use crate::matchers::collection::membership::{contain, contain_all, contain_any};

    #[test]
    fn should_contain() {
        let collection = vec!["junit", "testify"];
        let matcher = contain("junit");
        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_contain_but_id_did_not() {
        let collection = vec!["unit4j", "testify"];
        let matcher = contain("junit");
        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    fn should_contain_all_elements() {
        let collection = vec!["junit", "testify", "assert4j", "xunit"];
        let all_to_be_contained = vec!["testify", "assert4j", "xunit"];
        let matcher = contain_all(all_to_be_contained);
        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_contain_all_elements_but_it_did_not() {
        let collection = vec!["junit", "testify", "assert4j", "xunit"];
        let all_to_be_contained = vec!["testify", "assert", "xunit"];
        let matcher = contain_all(all_to_be_contained);
        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    fn should_contain_any_of_elements() {
        let collection = vec!["junit", "testify", "assert4j", "xunit"];
        let to_be_contained = vec!["testify", "catch", "xunit"];
        let matcher = contain_any(to_be_contained);
        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_contain_any_of_elements_but_it_did_not() {
        let collection = vec!["junit", "testify", "assert4j", "xunit"];
        let to_be_contained = vec!["catch", "catch2"];
        let matcher = contain_any(to_be_contained);
        matcher.test(&collection).passed.should_be_true();
    }
}
