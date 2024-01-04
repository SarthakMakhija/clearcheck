use crate::matchers::length::LengthBased;
use crate::matchers::{Matcher, MatcherResult};

impl Matcher<&str> for LengthBased {
    fn test(&self, value: &&str) -> MatcherResult {
        self.test(value.len())
    }
}
