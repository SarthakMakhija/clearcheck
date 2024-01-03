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

pub fn have_same_length(length: usize) -> LengthBasedMatcher {
    LengthBasedMatcher::Same(length)
}

pub fn have_atleast_same_length(length: usize) -> LengthBasedMatcher {
    LengthBasedMatcher::Atleast(length)
}

pub fn have_atmost_same_length(length: usize) -> LengthBasedMatcher {
    LengthBasedMatcher::Atmost(length)
}
