use std::collections::HashSet;
use std::hash::Hash;

use crate::matchers::Matcher;

pub struct DuplicateMatcher;

impl<T: Hash + Eq> Matcher<&[T]> for DuplicateMatcher {
    fn test(&self, value: &&[T]) -> bool {
        let unique = value.iter().collect::<HashSet<_>>();
        unique.len() != value.len()
    }
}

pub fn contain_duplicates() -> DuplicateMatcher {
    DuplicateMatcher
}
