use crate::matchers::length::LengthBased;
use crate::matchers::Matcher;

impl<T> Matcher<&[T]> for LengthBased {
    fn test(&self, collection: &&[T]) -> bool {
        match self {
            LengthBased::Same(length) => collection.len() == *length,
            LengthBased::Atleast(length) => collection.len() >= *length,
            LengthBased::Atmost(length) => collection.len() <= *length,
            LengthBased::Zero => collection.len() == 0,
        }
    }
}
