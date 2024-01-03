use std::collections::HashMap;
use std::hash::Hash;

use crate::matchers::Matcher;

pub enum LengthBasedMatcher {
    Same(usize),
    Atleast(usize),
    Atmost(usize),
}

impl<T> Matcher<&[T]> for LengthBasedMatcher {
    fn test(&self, collection: &&[T]) -> bool {
        match self {
            LengthBasedMatcher::Same(length) => collection.len() == *length,
            LengthBasedMatcher::Atleast(length) => collection.len() >= *length,
            LengthBasedMatcher::Atmost(length) => collection.len() <= *length,
        }
    }
}

impl<K: Eq + Hash, V> Matcher<HashMap<K, V>> for LengthBasedMatcher {
    fn test(&self, collection: &HashMap<K, V>) -> bool {
        match self {
            LengthBasedMatcher::Same(length) => collection.len() == *length,
            LengthBasedMatcher::Atleast(length) => collection.len() >= *length,
            LengthBasedMatcher::Atmost(length) => collection.len() <= *length,
        }
    }
}

impl Matcher<&str> for LengthBasedMatcher {
    fn test(&self, value: &&str) -> bool {
        match self {
            LengthBasedMatcher::Same(length) => value.len() == *length,
            LengthBasedMatcher::Atleast(length) => value.len() >= *length,
            LengthBasedMatcher::Atmost(length) => value.len() <= *length,
        }
    }
}

pub fn have_same_length(length: usize) -> LengthBasedMatcher {
    LengthBasedMatcher::Same(length)
}

pub fn have_atleast_same_length(length: usize) -> LengthBasedMatcher {
    LengthBasedMatcher::Atleast(length)
}

pub fn have_atmost_same_length(length: usize) -> LengthBasedMatcher {
    LengthBasedMatcher::Atmost(length)
}
