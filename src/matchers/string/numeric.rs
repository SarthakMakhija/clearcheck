use std::marker::PhantomData;
use std::str::FromStr;

use crate::matchers::{Matcher, MatcherResult};

/// NumericMatcher offers a flexible way to assert whether a string is numeric.
///
/// # Example
///```
/// use clearcheck::matchers::string::numeric::be_numeric;
/// use clearcheck::matchers::Matcher;
///
/// let matcher = be_numeric::<i32>();
/// assert!(matcher.test(&"12345").passed());
/// ```
pub struct NumericMatcher<M: FromStr> {
    _inner: PhantomData<M>,
}

impl<T: AsRef<str>, M: FromStr> Matcher<T> for NumericMatcher<M> {
    fn test(&self, value: &T) -> MatcherResult {
        let parse_result = value.as_ref().parse::<M>();
        MatcherResult::formatted(
            parse_result.is_ok(),
            format!("{:?} should be numeric", value.as_ref()),
            format!("{:?} should not be numeric", value.as_ref()),
        )
    }
}

/// Creates a NumericMatcher that asserts whether a string is numeric.
pub fn be_numeric<M: FromStr>() -> NumericMatcher<M> {
    NumericMatcher {
        _inner: PhantomData,
    }
}

#[cfg(test)]
mod tests {
    use crate::assertions::bool::TrueFalseAssertion;
    use crate::matchers::Matcher;
    use crate::matchers::string::numeric::be_numeric;

    #[test]
    fn should_be_numeric_i32() {
        let matcher = be_numeric::<i32>();
        let value = "123";
        matcher.test(&value).passed.should_be_true();
    }

    #[test]
    fn should_be_numeric_f64() {
        let matcher = be_numeric::<f64>();
        let value = "123.45";
        matcher.test(&value).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_numeric_i32_but_was_not() {
        let matcher = be_numeric::<i32>();
        let value = "123a";
        matcher.test(&value).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_numeric_f64_but_was_not() {
        let matcher = be_numeric::<f64>();
        let value = "123.45a";
        matcher.test(&value).passed.should_be_true();
    }
}
