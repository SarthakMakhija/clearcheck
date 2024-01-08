use std::fmt::Debug;

use crate::matchers::{Matcher, MatcherResult};

pub enum OrderedBased<'a, T: PartialOrd> {
    Gt(&'a T),
    Gte(&'a T),
    Lt(&'a T),
    Lte(&'a T),
}

impl<T: Debug + PartialOrd> Matcher<T> for OrderedBased<'_, T> {
    fn test(&self, value: &T) -> MatcherResult {
        match self {
            OrderedBased::Gt(other) => MatcherResult::formatted(
                value > other,
                format!("{:?} should be greater than {:?}", value, other),
                format!("{:?} should not be greater than {:?}", value, other),
            ),
            OrderedBased::Gte(other) => MatcherResult::formatted(
                value >= other,
                format!("{:?} should be greater than equals to {:?}", value, other),
                format!(
                    "{:?} should not be greater than equals to {:?}",
                    value, other
                ),
            ),
            OrderedBased::Lt(other) => MatcherResult::formatted(
                value < other,
                format!("{:?} should be less than {:?}", value, other),
                format!("{:?} should not be less than {:?}", value, other),
            ),
            OrderedBased::Lte(other) => MatcherResult::formatted(
                value <= other,
                format!("{:?} should be less than equals to {:?}", value, other),
                format!("{:?} should not be less than equals to {:?}", value, other),
            ),
        }
    }
}

pub fn be_greater_than<T: PartialOrd>(other: &T) -> OrderedBased<'_, T> {
    OrderedBased::Gt(other)
}

pub fn be_greater_than_equal_to<T: PartialOrd>(other: &T) -> OrderedBased<'_, T> {
    OrderedBased::Gte(other)
}

pub fn be_less_than<T: PartialOrd>(other: &T) -> OrderedBased<'_, T> {
    OrderedBased::Lt(other)
}

pub fn be_less_than_equal_to<T: PartialOrd>(other: &T) -> OrderedBased<'_, T> {
    OrderedBased::Lte(other)
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
        let matcher = be_greater_than(&90);
        matcher.test(&value).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_greater_than_but_was_not() {
        let value = 80;
        let matcher = be_greater_than(&90);
        matcher.test(&value).passed.should_be_true();
    }

    #[test]
    fn should_be_greater_than_equal_to() {
        let value = 100;
        let matcher = be_greater_than_equal_to(&100);
        matcher.test(&value).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_greater_than_equal_to_but_was_not() {
        let value = 80;
        let matcher = be_greater_than_equal_to(&90);
        matcher.test(&value).passed.should_be_true();
    }

    #[test]
    fn should_be_less_than() {
        let value = 80;
        let matcher = be_less_than(&90);
        matcher.test(&value).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_less_than_but_was_not() {
        let value = 100;
        let matcher = be_less_than(&90);
        matcher.test(&value).passed.should_be_true();
    }

    #[test]
    fn should_be_less_than_equal_to() {
        let value = 100;
        let matcher = be_less_than_equal_to(&100);
        matcher.test(&value).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_less_than_equal_to_but_was_not() {
        let value = 100;
        let matcher = be_less_than_equal_to(&90);
        matcher.test(&value).passed.should_be_true();
    }
}
