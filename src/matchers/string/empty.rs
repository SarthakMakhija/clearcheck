use crate::matchers::empty::EmptyBased;
use crate::matchers::{Matcher, MatcherResult};

impl Matcher<&str> for EmptyBased {
    fn test(&self, value: &&str) -> MatcherResult {
        self.test_string(value)
    }
}
