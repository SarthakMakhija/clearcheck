use crate::matchers::equal::EqualityBased;
use crate::matchers::Matcher;

impl Matcher<&str> for EqualityBased<'_, &str> {
    fn test(&self, value: &&str) -> bool {
        match self {
            EqualityBased::IgnoringCase(other) => value.eq_ignore_ascii_case(other),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::assertions::bool::TrueFalse;
    use crate::matchers::equal::be_equal_ignoring_case;
    use crate::matchers::Matcher;

    #[test]
    fn should_equal() {
        let matcher = be_equal_ignoring_case(&"ASSERT");
        matcher.test(&"assert").should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_equal_but_was_not() {
        let matcher = be_equal_ignoring_case(&"assert");
        matcher.test(&"assert4J").should_be_true();
    }
}
