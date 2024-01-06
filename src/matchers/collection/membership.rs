use std::fmt::Debug;

use crate::matchers::{Matcher, MatcherResult};

pub enum MembershipBased<'a, T: Eq + Debug> {
    Contain(&'a T),
    ContainAll(&'a [T]),
    ContainAny(&'a [T]),
}

impl<'a, T: Eq + Debug> MembershipBased<'a, T> {
    fn test(&self, collection: &[T]) -> MatcherResult {
        match self {
            MembershipBased::Contain(element) => MatcherResult::formatted(
                collection.contains(element),
                format!("{:?} should contain {:?}", collection, element),
                format!("{:?} should not contain {:?}", collection, element),
            ),
            MembershipBased::ContainAll(target) => {
                let missing = target
                    .iter()
                    .filter(|element| !collection.contains(element))
                    .collect::<Vec<_>>();

                MatcherResult::formatted(
                    missing.len() == 0,
                    format!(
                        "{:?} should contain {:?} but was missing {:?}",
                        collection, target, missing
                    ),
                    format!("{:?} should not contain {:?}", collection, target),
                )
            }
            MembershipBased::ContainAny(target) => MatcherResult::formatted(
                target.iter().any(|source| collection.contains(source)),
                format!("{:?} should contain any of {:?}", collection, target),
                format!("{:?} should not contain any of {:?}", collection, target),
            ),
        }
    }
}

impl<T> Matcher<Vec<T>> for MembershipBased<'_, T>
where
    T: Eq + Debug,
{
    fn test(&self, collection: &Vec<T>) -> MatcherResult {
        self.test(&collection)
    }
}

impl<T, const N: usize> Matcher<[T; N]> for MembershipBased<'_, T>
where
    T: Eq + Debug,
{
    fn test(&self, collection: &[T; N]) -> MatcherResult {
        self.test(collection as &[T])
    }
}

impl<T> Matcher<&[T]> for MembershipBased<'_, T>
where
    T: Eq + Debug,
{
    fn test(&self, collection: &&[T]) -> MatcherResult {
        self.test(&collection)
    }
}

pub fn contain<T>(element: &T) -> MembershipBased<'_, T>
where
    T: Eq + Debug,
{
    MembershipBased::Contain(element)
}

pub fn contain_all<T>(elements: &[T]) -> MembershipBased<'_, T>
where
    T: Eq + Debug,
{
    MembershipBased::ContainAll(elements)
}

pub fn contain_any<T>(elements: &[T]) -> MembershipBased<'_, T>
where
    T: Eq + Debug,
{
    MembershipBased::ContainAny(elements)
}

#[cfg(test)]
mod tests {
    use crate::assertions::bool::TrueFalse;
    use crate::matchers::collection::membership::{contain, contain_all, contain_any};

    #[test]
    fn should_contain() {
        let collection = vec!["junit", "testify"];
        let matcher = contain(&"junit");
        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_contain_but_id_did_not() {
        let collection = vec!["unit4j", "testify"];
        let matcher = contain(&"junit");
        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    fn should_contain_all_elements() {
        let collection = vec!["junit", "testify", "assert4j", "xunit"];
        let all_to_be_contained = vec!["testify", "assert4j", "xunit"];
        let matcher = contain_all(&all_to_be_contained);
        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_contain_all_elements_but_it_did_not() {
        let collection = vec!["junit", "testify", "assert4j", "xunit"];
        let all_to_be_contained = vec!["testify", "assert", "xunit"];
        let matcher = contain_all(&all_to_be_contained);
        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    fn should_contain_any_of_elements() {
        let collection = vec!["junit", "testify", "assert4j", "xunit"];
        let to_be_contained = vec!["testify", "catch", "xunit"];
        let matcher = contain_any(&to_be_contained);
        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_contain_any_of_elements_but_it_did_not() {
        let collection = vec!["junit", "testify", "assert4j", "xunit"];
        let to_be_contained = vec!["catch", "catch2"];
        let matcher = contain_any(&to_be_contained);
        matcher.test(&collection).passed.should_be_true();
    }
}
