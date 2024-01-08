use crate::matchers::{Matcher, MatcherResult};

pub enum SomeNoneMatcher {
    Some,
    None,
}

impl<T> Matcher<Option<T>> for SomeNoneMatcher {
    fn test(&self, value: &Option<T>) -> MatcherResult {
        match self {
            SomeNoneMatcher::Some => MatcherResult::new(
                value.is_some(),
                "Value should be Some",
                "Value should not be Some",
            ),
            SomeNoneMatcher::None => MatcherResult::new(
                value.is_none(),
                "Value should be None",
                "Value should not be None",
            ),
        }
    }
}

pub fn be_some() -> SomeNoneMatcher {
    SomeNoneMatcher::Some
}

pub fn be_none() -> SomeNoneMatcher {
    SomeNoneMatcher::None
}

#[cfg(test)]
mod tests {
    use crate::assertions::bool::TrueFalseAssertion;
    use crate::matchers::option::{be_none, be_some};
    use crate::matchers::Matcher;

    #[test]
    fn should_be_some() {
        let matcher = be_some();
        matcher.test(&Some(10)).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_some_but_was_not() {
        let matcher = be_some();
        matcher.test(&None::<()>).passed.should_be_true();
    }

    #[test]
    fn should_be_none() {
        let matcher = be_none();
        matcher.test(&None::<()>).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_none_but_was_not() {
        let matcher = be_none();
        matcher.test(&Some(10)).passed.should_be_true();
    }
}
