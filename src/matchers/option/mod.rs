use crate::matchers::{Matcher, MatcherResult};

pub enum SomeNoneBased {
    Some,
    None,
}

impl<T> Matcher<Option<T>> for SomeNoneBased {
    fn test(&self, value: &Option<T>) -> MatcherResult {
        match self {
            SomeNoneBased::Some => MatcherResult::new(
                value.is_some(),
                "Value should be Some",
                "Value should not be Some",
            ),
            SomeNoneBased::None => MatcherResult::new(
                value.is_none(),
                "Value should be None",
                "Value should not be None",
            ),
        }
    }
}

pub fn be_some() -> SomeNoneBased {
    SomeNoneBased::Some
}

pub fn be_none() -> SomeNoneBased {
    SomeNoneBased::None
}

#[cfg(test)]
mod tests {
    use crate::assertions::bool::TrueFalse;
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
