use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

use crate::matchers::{Matcher, MatcherResult};

pub struct DuplicateItemBased;

impl DuplicateItemBased {
    fn test<T: Hash + Eq + Debug>(&self, collection: &[T]) -> MatcherResult {
        let unique = collection.iter().collect::<HashSet<_>>();
        MatcherResult::formatted(
            unique.len() != collection.len(),
            format!("{:?} should have duplicates", collection),
            format!("{:?} should not have duplicates", collection),
        )
    }
}

impl<T: Hash + Eq + Debug> Matcher<Vec<T>> for DuplicateItemBased {
    fn test(&self, collection: &Vec<T>) -> MatcherResult {
        self.test(&collection)
    }
}

impl<T: Hash + Eq + Debug, const N: usize> Matcher<[T; N]> for DuplicateItemBased {
    fn test(&self, collection: &[T; N]) -> MatcherResult {
        self.test(collection as &[T])
    }
}

impl<T: Hash + Eq + Debug> Matcher<&[T]> for DuplicateItemBased {
    fn test(&self, collection: &&[T]) -> MatcherResult {
        self.test(&collection)
    }
}

pub fn contain_duplicates() -> DuplicateItemBased {
    DuplicateItemBased
}

#[cfg(test)]
mod tests {
    use crate::assertions::bool::TrueFalse;
    use crate::matchers::collection::duplicate::contain_duplicates;

    #[test]
    fn should_contains_duplicates() {
        let matcher = contain_duplicates();
        let collection = vec!["junit", "assert4j", "junit"];
        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_contains_duplicates_but_it_did_not() {
        let matcher = contain_duplicates();
        let collection = vec!["junit", "assert4j", ""];
        matcher.test(&collection).passed.should_be_true();
    }
}
