use crate::matchers::{Matcher, MatcherResult};

/// BoundaryMatcher offers a flexible way to assert that a string begins or ends with specific values.
///
/// # Example
///```
/// use clearcheck::matchers::Matcher;
/// use clearcheck::matchers::string::boundary::begin_with;
///
/// let matcher = begin_with("clear");
/// assert!(matcher.test(&"clearcheck").passed());
/// ```
pub enum BoundaryMatcher {
    Begin(&'static str),
    End(&'static str),
}

impl<T> Matcher<T> for BoundaryMatcher
    where T: AsRef<str>
{
    fn test(&self, value: &T) -> MatcherResult {
        match self {
            BoundaryMatcher::Begin(prefix) => MatcherResult::formatted(
                value.as_ref().starts_with(prefix),
                format!("{:?} should begin with {:?}", value.as_ref(), prefix),
                format!("{:?} should not begin with {:?}", value.as_ref(), prefix),
            ),
            BoundaryMatcher::End(suffix) => MatcherResult::formatted(
                value.as_ref().ends_with(suffix),
                format!("{:?} should end with {:?}", value.as_ref(), suffix),
                format!("{:?} should not end with {:?}", value.as_ref(), suffix),
            ),
        }
    }
}

/// Creates a BoundaryMatcher that asserts whether a string value begins with the given prefix.
pub fn begin_with(prefix: &'static str) -> BoundaryMatcher {
    BoundaryMatcher::Begin(prefix)
}

/// Creates a BoundaryMatcher that asserts whether a string value ends with the given suffix.
pub fn end_with(suffix: &'static str) -> BoundaryMatcher {
    BoundaryMatcher::End(suffix)
}

#[cfg(test)]
mod tests {
    use crate::assertions::bool::TrueFalseAssertion;
    use crate::matchers::Matcher;
    use crate::matchers::string::boundary::{begin_with, end_with};

    #[test]
    fn should_begin_with() {
        let matcher = begin_with("go");
        matcher.test(&"goselect").passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_begin_with_but_did_not() {
        let matcher = begin_with("go");
        matcher.test(&"select").passed.should_be_true();
    }

    #[test]
    fn should_end_with() {
        let matcher = end_with("elect");
        matcher.test(&"goselect").passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_end_with_but_did_not() {
        let matcher = end_with("go");
        matcher.test(&"select").passed.should_be_true();
    }
}
