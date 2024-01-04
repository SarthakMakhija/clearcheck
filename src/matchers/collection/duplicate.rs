use std::collections::HashSet;
use std::hash::Hash;

use crate::matchers::Matcher;

pub struct DuplicateItemBased;

impl<T: Hash + Eq> Matcher<&[T]> for DuplicateItemBased {
    fn test(&self, collection: &&[T]) -> bool {
        let unique = collection.iter().collect::<HashSet<_>>();
        unique.len() != collection.len()
    }
}

impl<T: Hash + Eq> Matcher<Vec<T>> for DuplicateItemBased {
    fn test(&self, collection: &Vec<T>) -> bool {
        let unique = collection.iter().collect::<HashSet<_>>();
        unique.len() != collection.len()
    }
}

pub fn contain_duplicates() -> DuplicateItemBased {
    DuplicateItemBased
}
