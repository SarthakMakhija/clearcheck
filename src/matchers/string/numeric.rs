use std::marker::PhantomData;
use std::str::FromStr;

use crate::matchers::{Matcher, MatcherResult};

pub struct NumericBased<T: FromStr> {
    _inner: PhantomData<T>,
}

impl<T: FromStr> Matcher<&str> for NumericBased<T> {
    fn test(&self, value: &&str) -> MatcherResult {
        let parse_result = value.parse::<T>();
        MatcherResult::formatted(
            parse_result.is_ok(),
            format!("{:?} should be numeric", value),
            format!("{:?} should not be numeric", value),
        )
    }
}

pub fn be_numeric<T: FromStr>() -> NumericBased<T> {
    NumericBased {
        _inner: PhantomData,
    }
}

#[cfg(test)]
mod tests {
    use crate::assertions::bool::TrueFalseAssertion;
    use crate::matchers::string::numeric::be_numeric;
    use crate::matchers::Matcher;

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
