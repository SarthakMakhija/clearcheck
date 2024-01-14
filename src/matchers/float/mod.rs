use std::fmt::Debug;

use num::Float;

use crate::matchers::{Matcher, MatcherResult};

/// FloatMatcher offers a flexible way to make assertions about specific float attributes.
///
/// # Example
///```
/// use clearcheck::matchers::float::be_zero;
/// use clearcheck::matchers::Matcher;
///
/// let value: f64 = 0.0;
/// let matcher = be_zero();
///
/// assert!(matcher.test(&value).passed());
/// ```
pub enum FloatMatcher {
    NaN,
    Zero,
    Positive,
    Negative,
}

impl<T: Float + Debug + Default + PartialEq> Matcher<T> for FloatMatcher {
    fn test(&self, value: &T) -> MatcherResult {
        match self {
            FloatMatcher::NaN => MatcherResult::formatted(
                value.is_nan(),
                format!("{:?} should be NaN", value),
                format!("{:?} should not be NaN", value),
            ),
            FloatMatcher::Zero => MatcherResult::formatted(
                value.is_zero(),
                format!("{:?} should be zero", value),
                format!("{:?} should not be zero", value),
            ),
            FloatMatcher::Positive => MatcherResult::formatted(
                value.is_sign_positive(),
                format!("{:?} should be positive", value),
                format!("{:?} should not be positive", value),
            ),
            FloatMatcher::Negative => MatcherResult::formatted(
                value.is_sign_negative(),
                format!("{:?} should be negative", value),
                format!("{:?} should not be negative", value),
            ),
        }
    }
}

/// Creates a FloatMatcher that asserts whether a floating value is NaN (not a number).
pub fn be_nan() -> FloatMatcher {
    FloatMatcher::NaN
}

/// Creates a FloatMatcher that asserts whether a floating value is zero.
pub fn be_zero() -> FloatMatcher {
    FloatMatcher::Zero
}

/// Creates a FloatMatcher that asserts whether a floating value is positive.
pub fn be_positive() -> FloatMatcher {
    FloatMatcher::Positive
}

/// Creates a FloatMatcher that asserts whether a floating value is negative.
pub fn be_negative() -> FloatMatcher {
    FloatMatcher::Negative
}

#[cfg(all(test, feature = "num"))]
mod tests {
    use crate::assertions::bool::TrueFalseAssertion;
    use crate::matchers::float::{be_nan, be_negative, be_positive, be_zero};
    use crate::matchers::Matcher;
    use num::Float;

    #[test]
    fn should_be_nan() {
        let value: f64 = Float::nan();
        let matcher = be_nan();
        matcher.test(&value).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_nan_but_was_not() {
        let value: f64 = 1.10;
        let matcher = be_nan();
        matcher.test(&value).passed.should_be_true();
    }

    #[test]
    fn should_be_zero() {
        let value: f64 = 0.0;
        let matcher = be_zero();
        matcher.test(&value).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_zero_but_was_not() {
        let value: f64 = 1.10;
        let matcher = be_zero();
        matcher.test(&value).passed.should_be_true();
    }

    #[test]
    fn should_be_positive() {
        let value: f64 = 1.0;
        let matcher = be_positive();
        matcher.test(&value).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_positive_but_was_not() {
        let value: f64 = -1.10;
        let matcher = be_positive();
        matcher.test(&value).passed.should_be_true();
    }

    #[test]
    fn should_be_negative() {
        let value: f64 = -1.0;
        let matcher = be_negative();
        matcher.test(&value).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_negative_but_was_not() {
        let value: f64 = 1.10;
        let matcher = be_negative();
        matcher.test(&value).passed.should_be_true();
    }
}
