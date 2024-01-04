use crate::matchers::{Matcher, MatcherResult};
use std::fmt::Debug;

pub enum MembershipBased<'a, T: Eq + Debug> {
    Contain(&'a T),
}

impl<'a, T: Eq + Debug> MembershipBased<'a, T> {
    fn test(&self, collection: &[T]) -> MatcherResult {
        match self {
            MembershipBased::Contain(element) => MatcherResult::formatted(
                collection.contains(element),
                format!("{:?} should contain {:?}", collection, element),
                format!("{:?} should not contain {:?}", collection, element),
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

#[cfg(test)]
mod tests {
    use crate::assertions::bool::TrueFalse;
    use crate::matchers::collection::membership::contain;

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
}
