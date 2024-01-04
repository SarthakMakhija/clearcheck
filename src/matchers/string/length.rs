use crate::matchers::length::LengthBased;
use crate::matchers::Matcher;

impl Matcher<&str> for LengthBased {
    fn test(&self, value: &&str) -> bool {
        self.test_length(value.len())
    }
}
