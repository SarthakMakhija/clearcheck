use crate::matchers::equal::EqualityBased;
use crate::matchers::{Matcher, MatcherResult};

impl Matcher<&str> for EqualityBased<'_, &str> {
    fn test(&self, value: &&str) -> MatcherResult {
        match self {
            EqualityBased::IgnoringCase(other) => MatcherResult::formatted(
                value.eq_ignore_ascii_case(other),
                format!("{:?} should equal {:?}", value, other),
                format!("{:?} should not equal {:?}", value, other),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::assertions::bool::TrueFalseAssertions;
    use crate::matchers::equal::be_equal_ignoring_case;
    use crate::matchers::Matcher;

    #[test]
    fn should_equal() {
        let matcher = be_equal_ignoring_case(&"ASSERT");
        matcher.test(&"assert").passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_equal_but_was_not() {
        let matcher = be_equal_ignoring_case(&"assert");
        matcher.test(&"assert4J").passed.should_be_true();
    }
}
