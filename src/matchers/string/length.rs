use crate::matchers::length::LengthMatcher;
use crate::matchers::{Matcher, MatcherResult};

impl Matcher<&str> for LengthMatcher {
    fn test(&self, value: &&str) -> MatcherResult {
        self.test_string(value)
    }
}
