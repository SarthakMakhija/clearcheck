use crate::matchers::equal::IgnoreCaseEqualityMatcher;
use crate::matchers::{Matcher, MatcherResult};

impl<T> Matcher<T> for IgnoreCaseEqualityMatcher<&str>
where T: AsRef<str>
{
    fn test(&self, value: &T) -> MatcherResult {
        MatcherResult::formatted(
            value.as_ref().eq_ignore_ascii_case(self.other),
            format!("{:?} should equal {:?}", value.as_ref(), self.other),
            format!("{:?} should not equal {:?}", value.as_ref(), self.other),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::assertions::bool::TrueFalseAssertion;
    use crate::matchers::equal::be_equal_ignoring_case;
    use crate::matchers::Matcher;

    #[test]
    fn should_equal() {
        let matcher = be_equal_ignoring_case("ASSERT");
        matcher.test(&"assert").passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_equal_but_was_not() {
        let matcher = be_equal_ignoring_case("assert");
        matcher.test(&"assert4J").passed.should_be_true();
    }
}
