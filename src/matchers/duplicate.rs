use std::collections::HashSet;
use std::hash::Hash;

use crate::matchers::Matcher;

pub struct DuplicateItemMatcher;

impl<T: Hash + Eq> Matcher<&[T]> for DuplicateItemMatcher {
    fn test(&self, value: &&[T]) -> bool {
        let unique = value.iter().collect::<HashSet<_>>();
        unique.len() != value.len()
    }
}

pub fn contain_duplicates() -> DuplicateItemMatcher {
    DuplicateItemMatcher
}
