use crate::matchers::Matcher;

pub enum SomeNoneBased {
    Some,
    None,
}

impl<T> Matcher<Option<T>> for SomeNoneBased {
    fn test(&self, value: &Option<T>) -> bool {
        match self {
            SomeNoneBased::Some => value.is_some(),
            SomeNoneBased::None => value.is_none(),
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
        matcher.test(&Some(10)).should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_some_but_was_not() {
        let matcher = be_some();
        matcher.test(&None::<()>).should_be_true();
    }

    #[test]
    fn should_be_none() {
        let matcher = be_none();
        matcher.test(&None::<()>).should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_none_but_was_not() {
        let matcher = be_none();
        matcher.test(&Some(10)).should_be_true();
    }
}
