use std::fmt::Debug;

use num::Integer;

use crate::matchers::{Matcher, MatcherResult};

pub enum IntMatcher {
    Positive,
    Negative,
    Even,
    Odd,
    Zero,
}

impl<T: Integer + Debug + PartialEq + Default> Matcher<T> for IntMatcher {
    fn test(&self, value: &T) -> MatcherResult {
        match self {
            IntMatcher::Positive => MatcherResult::formatted(
                *value > T::default(),
                format!("{:?} should be positive", value),
                format!("{:?} should not be positive", value),
            ),
            IntMatcher::Negative => MatcherResult::formatted(
                *value < T::default(),
                format!("{:?} should be negative", value),
                format!("{:?} should not be negative", value),
            ),
            IntMatcher::Even => MatcherResult::formatted(
                value.is_even(),
                format!("{:?} should be even", value),
                format!("{:?} should not be even", value),
            ),
            IntMatcher::Odd => MatcherResult::formatted(
                value.is_odd(),
                format!("{:?} should be odd", value),
                format!("{:?} should not be odd", value),
            ),
            IntMatcher::Zero => MatcherResult::formatted(
                *value == T::default(),
                format!("{:?} should be zero", value),
                format!("{:?} should not be zero", value),
            ),
        }
    }
}

pub fn be_positive() -> IntMatcher {
    IntMatcher::Positive
}

pub fn be_negative() -> IntMatcher {
    IntMatcher::Negative
}

pub fn be_even() -> IntMatcher {
    IntMatcher::Even
}

pub fn be_odd() -> IntMatcher {
    IntMatcher::Odd
}

pub fn be_zero() -> IntMatcher {
    IntMatcher::Zero
}

#[cfg(test)]
mod tests {
    use crate::assertions::bool::TrueFalseAssertion;
    use crate::matchers::int::{be_even, be_negative, be_odd, be_positive, be_zero};
    use crate::matchers::Matcher;

    #[test]
    fn should_be_positive() {
        let value = 10;
        let matcher = be_positive();
        matcher.test(&value).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_positive_but_was_not() {
        let value = -1;
        let matcher = be_positive();
        matcher.test(&value).passed.should_be_true();
    }

    #[test]
    fn should_be_negative() {
        let value = -10;
        let matcher = be_negative();
        matcher.test(&value).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_negative_but_was_not() {
        let value = 1;
        let matcher = be_negative();
        matcher.test(&value).passed.should_be_true();
    }

    #[test]
    fn should_be_even() {
        let value = -10;
        let matcher = be_even();
        matcher.test(&value).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_even_but_was_not() {
        let value = -1;
        let matcher = be_even();
        matcher.test(&value).passed.should_be_true();
    }

    #[test]
    fn should_be_odd() {
        let value = -11;
        let matcher = be_odd();
        matcher.test(&value).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_odd_but_was_not() {
        let value = -10;
        let matcher = be_odd();
        matcher.test(&value).passed.should_be_true();
    }

    #[test]
    fn should_be_zero() {
        let value = 0;
        let matcher = be_zero();
        matcher.test(&value).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_zero_but_was_not() {
        let value = -10;
        let matcher = be_zero();
        matcher.test(&value).passed.should_be_true();
    }
}
