use regex::Regex;

use crate::matchers::{Matcher, MatcherResult};

/// RegexMatcher offers a flexible way to assert whether a string matches a regular expression.
///
/// # Example
///```
/// use regex::Regex;
/// use clearcheck::matchers::string::regex::match_with;
/// use clearcheck::matchers::Matcher;
///
/// let matcher = match_with(Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap());
/// assert!(matcher.test(&"Started clearcheck on On 2024-01-02.").passed());
/// ```
pub struct RegexMatcher {
    regexp: Regex,
}

impl<T: AsRef<str>> Matcher<T> for RegexMatcher {
    fn test(&self, value: &T) -> MatcherResult {
        MatcherResult::formatted(
            self.regexp.is_match(value.as_ref()),
            format!(
                "{:?} should match the regular expression {:?}",
                value.as_ref(), self.regexp
            ),
            format!(
                "{:?} should not match the regular expression {:?}",
                value.as_ref(), self.regexp
            ),
        )
    }
}

/// Creates a RegexMatcher that asserts whether a string matches the given regular expression.
pub fn match_with(regular_expression: Regex) -> RegexMatcher {
    RegexMatcher {
        regexp: regular_expression,
    }
}

#[cfg(all(test, feature = "regex"))]
mod tests {
    use crate::assertions::bool::TrueFalseAssertion;
    use crate::matchers::string::regex::match_with;
    use crate::matchers::Matcher;
    use regex::Regex;

    #[test]
    fn should_match_regular_expression() {
        let regex = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
        let str = "Started clearcheck on On 2024-01-02.";

        let matcher = match_with(regex);
        matcher.test(&str).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_match_regular_expression_but_it_did_not() {
        let regex = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
        let str = "Started clearcheck on On 02nd January 2024";

        let matcher = match_with(regex);
        matcher.test(&str).passed.should_be_true();
    }
}
