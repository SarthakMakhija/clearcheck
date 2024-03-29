//! provides [IgnoreCaseEqualityMatcher] for char.

use crate::matchers::equal::IgnoreCaseEqualityMatcher;
use crate::matchers::{Matcher, MatcherResult};

impl Matcher<char> for IgnoreCaseEqualityMatcher<char> {
    fn test(&self, value: &char) -> MatcherResult {
        MatcherResult::formatted(
            value.eq_ignore_ascii_case(&self.other),
            format!("{} should match {}", value, self.other),
            format!("{} should not match {}", value, self.other),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::assertions::bool::TrueFalseAssertion;
    use crate::matchers::equal::be_equal_ignoring_case;
    use crate::matchers::Matcher;

    #[test]
    fn should_be_equal() {
        let based = be_equal_ignoring_case('a');
        based.test(&'a').passed.should_be_true();
    }

    #[test]
    fn should_not_be_equal() {
        let based = be_equal_ignoring_case('b');
        based.test(&'a').passed.should_be_false();
    }
}
