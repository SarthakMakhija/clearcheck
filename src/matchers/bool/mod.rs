use crate::matchers::{Matcher, MatcherResult};

/// TrueFalseMatcher provides a way to assert whether boolean values evaluate to true or false.
///
/// # Example
///```
/// use clearcheck::matchers::bool::be_true;
/// use clearcheck::matchers::Matcher;
///
/// let matcher = be_true();
/// assert!(matcher.test(&true).passed());
/// ```
pub enum TrueFalseMatcher {
    True,
    False,
}

impl Matcher<bool> for TrueFalseMatcher {
    fn test(&self, value: &bool) -> MatcherResult {
        match self {
            TrueFalseMatcher::True => {
                MatcherResult::new(*value, "Value should be TRUE", "Value should not be TRUE")
            }
            TrueFalseMatcher::False => MatcherResult::new(
                !(*value),
                "Value should be FALSE",
                "Value should not be FALSE",
            ),
        }
    }
}

/// Creates a TrueFalseMatcher::True instance for asserting true.
pub fn be_true() -> TrueFalseMatcher {
    TrueFalseMatcher::True
}

/// Creates a TrueFalseMatcher::False instance for asserting false.
pub fn be_false() -> TrueFalseMatcher {
    TrueFalseMatcher::False
}

#[cfg(test)]
mod tests {
    use crate::assertions::bool::TrueFalseAssertion;
    use crate::matchers::bool::{be_false, be_true};
    use crate::matchers::Matcher;

    #[test]
    fn should_be_true() {
        let matcher = be_true();
        matcher.test(&true).passed.should_be_true();
    }

    #[test]
    fn should_be_false() {
        let matcher = be_false();
        matcher.test(&false).passed.should_be_true();
    }
}
