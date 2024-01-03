use std::collections::HashMap;
use std::hash::Hash;

use crate::matchers::Matcher;

pub enum LengthBased {
    Same(usize),
    Atleast(usize),
    Atmost(usize),
}

impl<T> Matcher<&[T]> for LengthBased {
    fn test(&self, collection: &&[T]) -> bool {
        match self {
            LengthBased::Same(length) => collection.len() == *length,
            LengthBased::Atleast(length) => collection.len() >= *length,
            LengthBased::Atmost(length) => collection.len() <= *length,
        }
    }
}

impl<K: Eq + Hash, V> Matcher<HashMap<K, V>> for LengthBased {
    fn test(&self, collection: &HashMap<K, V>) -> bool {
        match self {
            LengthBased::Same(length) => collection.len() == *length,
            LengthBased::Atleast(length) => collection.len() >= *length,
            LengthBased::Atmost(length) => collection.len() <= *length,
        }
    }
}

impl Matcher<&str> for LengthBased {
    fn test(&self, value: &&str) -> bool {
        match self {
            LengthBased::Same(length) => value.len() == *length,
            LengthBased::Atleast(length) => value.len() >= *length,
            LengthBased::Atmost(length) => value.len() <= *length,
        }
    }
}

pub fn have_same_length(length: usize) -> LengthBased {
    LengthBased::Same(length)
}

pub fn have_atleast_same_length(length: usize) -> LengthBased {
    LengthBased::Atleast(length)
}

pub fn have_atmost_same_length(length: usize) -> LengthBased {
    LengthBased::Atmost(length)
}
