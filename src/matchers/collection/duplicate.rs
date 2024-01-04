use std::collections::HashSet;
use std::hash::Hash;

use crate::matchers::Matcher;

pub struct DuplicateItemBased;

impl DuplicateItemBased {
    fn test<T: Hash + Eq>(&self, collection: &[T]) -> bool {
        let unique = collection.iter().collect::<HashSet<_>>();
        unique.len() != collection.len()
    }
}

impl<T: Hash + Eq> Matcher<Vec<T>> for DuplicateItemBased {
    fn test(&self, collection: &Vec<T>) -> bool {
        self.test(&collection)
    }
}

impl<T: Hash + Eq, const N: usize> Matcher<[T; N]> for DuplicateItemBased {
    fn test(&self, collection: &[T; N]) -> bool {
        self.test(collection as &[T])
    }
}

impl<T: Hash + Eq> Matcher<&[T]> for DuplicateItemBased {
    fn test(&self, collection: &&[T]) -> bool {
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
        matcher.test(&collection).should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_contains_duplicates_but_it_did_not() {
        let matcher = contain_duplicates();
        let collection = vec!["junit", "assert4j", ""];
        matcher.test(&collection).should_be_true();
    }
}
