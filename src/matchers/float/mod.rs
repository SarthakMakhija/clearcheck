use std::fmt::Debug;

use num::Float;

use crate::matchers::{Matcher, MatcherResult};

pub enum FloatBased {
    NaN,
    Zero,
    Positive,
    Negative,
}

impl<T: Float + Debug + Default + PartialEq> Matcher<T> for FloatBased {
    fn test(&self, value: &T) -> MatcherResult {
        match self {
            FloatBased::NaN => MatcherResult::formatted(
                value.is_nan(),
                format!("{:?} should be NaN", value),
                format!("{:?} should not be NaN", value),
            ),
            FloatBased::Zero => MatcherResult::formatted(
                value.is_zero(),
                format!("{:?} should be zero", value),
                format!("{:?} should not be zero", value),
            ),
            FloatBased::Positive => MatcherResult::formatted(
                value.is_sign_positive(),
                format!("{:?} should be positive", value),
                format!("{:?} should not be positive", value),
            ),
            FloatBased::Negative => MatcherResult::formatted(
                value.is_sign_negative(),
                format!("{:?} should be negative", value),
                format!("{:?} should not be negative", value),
            ),
        }
    }
}

pub fn be_nan() -> FloatBased {
    FloatBased::NaN
}

pub fn be_zero() -> FloatBased {
    FloatBased::Zero
}

pub fn be_positive() -> FloatBased {
    FloatBased::Positive
}

pub fn be_negative() -> FloatBased {
    FloatBased::Negative
}

#[cfg(test)]
mod tests {
    use crate::assertions::bool::TrueFalse;
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
