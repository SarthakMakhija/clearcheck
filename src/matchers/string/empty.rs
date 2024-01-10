use crate::matchers::empty::EmptyMatcher;
use crate::matchers::{Matcher, MatcherResult};

impl Matcher<&str> for EmptyMatcher {
    fn test(&self, value: &&str) -> MatcherResult {
        self.test_string(value)
    }
}
