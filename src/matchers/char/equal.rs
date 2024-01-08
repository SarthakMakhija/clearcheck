use crate::matchers::equal::IgnoreCaseEqualityMatcher;
use crate::matchers::{Matcher, MatcherResult};

impl Matcher<char> for IgnoreCaseEqualityMatcher<'_, char> {
    fn test(&self, value: &char) -> MatcherResult {
        match self {
            IgnoreCaseEqualityMatcher::IgnoringCase(other) => MatcherResult::formatted(
                value.eq_ignore_ascii_case(other),
                format!("{} should match {}", value, other),
                format!("{} should not match {}", value, other),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::assertions::bool::TrueFalseAssertion;
    use crate::matchers::equal::be_equal_ignoring_case;
    use crate::matchers::Matcher;

    #[test]
    fn should_be_equal() {
        let based = be_equal_ignoring_case(&'a');
        based.test(&'a').passed.should_be_true();
    }

    #[test]
    fn should_not_be_equal() {
        let based = be_equal_ignoring_case(&'b');
        based.test(&'a').passed.should_be_false();
    }
}
