use std::fmt::Debug;

use crate::matchers::{Matcher, MatcherResult};

/// OrderedMatcher offers a flexible way to assert ordering relationships between values.
///
/// Works with any data type that implements the PartialOrd trait.
///
/// # Example
///```
/// use clearcheck::matchers::Matcher;
/// use clearcheck::matchers::ordered::be_greater_than;
///
/// let value = 100;
/// let matcher = be_greater_than(90);
///
/// assert!(matcher.test(&value).passed());
/// ```
pub enum OrderedMatcher<T: PartialOrd> {
    Gt(T),
    Gte(T),
    Lt(T),
    Lte(T),
}

impl<T: Debug + PartialOrd> Matcher<T> for OrderedMatcher<T> {
    fn test(&self, value: &T) -> MatcherResult {
        match self {
            OrderedMatcher::Gt(other) => MatcherResult::formatted(
                value > other,
                format!("{:?} should be greater than {:?}", value, other),
                format!("{:?} should not be greater than {:?}", value, other),
            ),
            OrderedMatcher::Gte(other) => MatcherResult::formatted(
                value >= other,
                format!("{:?} should be greater than equals to {:?}", value, other),
                format!(
                    "{:?} should not be greater than equals to {:?}",
                    value, other
                ),
            ),
            OrderedMatcher::Lt(other) => MatcherResult::formatted(
                value < other,
                format!("{:?} should be less than {:?}", value, other),
                format!("{:?} should not be less than {:?}", value, other),
            ),
            OrderedMatcher::Lte(other) => MatcherResult::formatted(
                value <= other,
                format!("{:?} should be less than equals to {:?}", value, other),
                format!("{:?} should not be less than equals to {:?}", value, other),
            ),
        }
    }
}

/// Creates an OrderedMatcher that asserts whether a value is greater than the given value.
pub fn be_greater_than<T: PartialOrd>(other: T) -> OrderedMatcher<T> {
    OrderedMatcher::Gt(other)
}

/// Creates an OrderedMatcher that asserts whether a value is greater than or equal to the given value.
pub fn be_greater_than_equal_to<T: PartialOrd>(other: T) -> OrderedMatcher<T> {
    OrderedMatcher::Gte(other)
}

/// Creates an OrderedMatcher that asserts whether a value is less than the given value.
pub fn be_less_than<T: PartialOrd>(other: T) -> OrderedMatcher<T> {
    OrderedMatcher::Lt(other)
}

/// Creates an OrderedMatcher that asserts whether a value is less than or equal to the given value.
pub fn be_less_than_equal_to<T: PartialOrd>(other: T) -> OrderedMatcher<T> {
    OrderedMatcher::Lte(other)
}

#[cfg(test)]
mod tests {
    use crate::assertions::bool::TrueFalseAssertion;
    use crate::matchers::ordered::{
        be_greater_than, be_greater_than_equal_to, be_less_than, be_less_than_equal_to,
    };
    use crate::matchers::Matcher;

    #[test]
    fn should_be_greater_than() {
        let value = 100;
        let matcher = be_greater_than(90);
        matcher.test(&value).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_greater_than_but_was_not() {
        let value = 80;
        let matcher = be_greater_than(90);
        matcher.test(&value).passed.should_be_true();
    }

    #[test]
    fn should_be_greater_than_equal_to() {
        let value = 100;
        let matcher = be_greater_than_equal_to(100);
        matcher.test(&value).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_greater_than_equal_to_but_was_not() {
        let value = 80;
        let matcher = be_greater_than_equal_to(90);
        matcher.test(&value).passed.should_be_true();
    }

    #[test]
    fn should_be_less_than() {
        let value = 80;
        let matcher = be_less_than(90);
        matcher.test(&value).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_less_than_but_was_not() {
        let value = 100;
        let matcher = be_less_than(90);
        matcher.test(&value).passed.should_be_true();
    }

    #[test]
    fn should_be_less_than_equal_to() {
        let value = 100;
        let matcher = be_less_than_equal_to(100);
        matcher.test(&value).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_less_than_equal_to_but_was_not() {
        let value = 100;
        let matcher = be_less_than_equal_to(90);
        matcher.test(&value).passed.should_be_true();
    }
}
