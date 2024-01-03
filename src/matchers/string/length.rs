use crate::matchers::length::LengthBased;
use crate::matchers::Matcher;

impl Matcher<&str> for LengthBased {
    fn test(&self, value: &&str) -> bool {
        match self {
            LengthBased::Same(length) => value.len() == *length,
            LengthBased::Atleast(length) => value.len() >= *length,
            LengthBased::Atmost(length) => value.len() <= *length,
        }
    }
}
