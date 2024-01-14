use crate::matchers::{Matcher, MatcherResult};

/// StringLengthMatcher offers a flexible way to assert various length properties of string.
///
/// # Example
///```
/// use clearcheck::matchers::string::length::have_atleast_same_length;
/// use clearcheck::matchers::Matcher;
///
/// let matcher = have_atleast_same_length(3);
/// assert!(matcher.test(&"clearcheck").passed());
/// ```
pub enum StringLengthMatcher {
    Same(usize),
    Atleast(usize),
    Atmost(usize),
}

impl<T> Matcher<T> for StringLengthMatcher
    where T: AsRef<str>
{
    fn test(&self, value: &T) -> MatcherResult {
        match self {
            StringLengthMatcher::Same(input_length) => MatcherResult::formatted(
                value.as_ref().len() == *input_length,
                format!(
                    "{:?} length {:?} should be {:?}",
                    value.as_ref(), input_length, input_length,
                ),
                format!(
                    "{:?} length {:?} should not be {:?}",
                    value.as_ref(), input_length, input_length,
                ),
            ),
            StringLengthMatcher::Atleast(input_length) => MatcherResult::formatted(
                value.as_ref().len() >= *input_length,
                format!(
                    "{:?} length {:?} should be atleast {:?}",
                    value.as_ref(), input_length, input_length,
                ),
                format!(
                    "{:?} length {:?} should not be atleast {:?}",
                    value.as_ref(), input_length, input_length,
                ),
            ),
            StringLengthMatcher::Atmost(input_length) => MatcherResult::formatted(
                value.as_ref().len() <= *input_length,
                format!(
                    "{:?} length {:?} should be atmost {:?}",
                    value.as_ref(), input_length, input_length,
                ),
                format!(
                    "{:?} length {:?} should not be atmost {:?}",
                    value.as_ref(), input_length, input_length,
                ),
            ),
        }
    }
}

/// Creates a StringLengthMatcher that asserts whether the length of a string is same as the given length.
pub fn have_same_length(length: usize) -> StringLengthMatcher {
    StringLengthMatcher::Same(length)
}

/// Creates a StringLengthMatcher that asserts whether the length of a string is greater than or equal to the given length.
pub fn have_atleast_same_length(length: usize) -> StringLengthMatcher {
    StringLengthMatcher::Atleast(length)
}

/// Creates a StringLengthMatcher that asserts whether the length of a string is less than or equal to the given length.
pub fn have_atmost_same_length(length: usize) -> StringLengthMatcher {
    StringLengthMatcher::Atmost(length)
}

#[cfg(test)]
mod tests {
    use crate::assertions::bool::TrueFalseAssertion;
    use crate::matchers::Matcher;
    use crate::matchers::string::length::{have_atleast_same_length, have_atmost_same_length, have_same_length};

    #[test]
    fn should_have_same_length() {
        let matcher = have_same_length(4);
        matcher.test(&"test").passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_have_same_length_but_was_not() {
        let matcher = have_same_length(4);
        matcher.test(&"clearcheck").passed.should_be_true();
    }

    #[test]
    fn should_have_atleast_same_length() {
        let matcher = have_atleast_same_length(4);
        matcher.test(&"clearcheck").passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_have_atleast_same_length_but_was_not() {
        let matcher = have_atleast_same_length(20);
        matcher.test(&"clearcheck").passed.should_be_true();
    }

    #[test]
    fn should_have_atmost_length() {
        let matcher = have_atmost_same_length(5);
        matcher.test(&"junit").passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_have_atmost_length_but_was_not() {
        let matcher = have_atmost_same_length(3);
        matcher.test(&"junit").passed.should_be_true();
    }
}
