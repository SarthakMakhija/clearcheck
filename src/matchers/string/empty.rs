use crate::matchers::{Matcher, MatcherResult};

/// StringEmptyMatcher offers a flexible way to assert whether a string is empty (no characters).
///
/// # Example
///```
/// use clearcheck::matchers::string::empty::be_empty;
/// use clearcheck::matchers::Matcher;
///
/// let matcher = be_empty();
/// assert!(matcher.test(&"").passed());
/// ```
pub enum StringEmptyMatcher {
    Empty,
    NotEmpty,
}

impl<T> Matcher<T> for StringEmptyMatcher
    where T: AsRef<str>
{
    fn test(&self, value: &T) -> MatcherResult {
        match self {
            StringEmptyMatcher::Empty => MatcherResult::new(
                value.as_ref().is_empty(),
                "Value should be empty",
                "Value should not be empty",
            ),
            StringEmptyMatcher::NotEmpty => MatcherResult::new(
                !value.as_ref().is_empty(),
                "Value should not be empty",
                "Value should be empty",
            ),
        }
    }
}

/// Creates a StringEmptyMatcher that asserts whether a string is empty.
pub fn be_empty() -> StringEmptyMatcher {
    StringEmptyMatcher::Empty
}

/// Creates a StringEmptyMatcher that asserts whether a string is not empty.
pub fn not_be_empty() -> StringEmptyMatcher {
    StringEmptyMatcher::NotEmpty
}

#[cfg(test)]
mod string_tests {
    use crate::assertions::bool::TrueFalseAssertion;
    use crate::matchers::Matcher;
    use crate::matchers::string::empty::{be_empty, not_be_empty};

    #[test]
    fn should_be_empty() {
        let matcher = be_empty();
        matcher.test(&"").passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_empty_but_was_not() {
        let matcher = be_empty();
        matcher.test(&"goselect").passed.should_be_true();
    }

    #[test]
    fn should_not_be_empty() {
        let matcher = not_be_empty();
        matcher.test(&"goselect").passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_not_be_empty_but_was() {
        let matcher = not_be_empty();
        matcher.test(&"").passed.should_be_true();
    }
}
