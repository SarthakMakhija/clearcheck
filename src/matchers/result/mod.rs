use crate::matchers::{Matcher, MatcherResult};

pub enum OkErrBased {
    Ok,
    Err,
}

impl<T, E> Matcher<Result<T, E>> for OkErrBased {
    fn test(&self, value: &Result<T, E>) -> MatcherResult {
        match self {
            OkErrBased::Ok => MatcherResult::new(
                value.is_ok(),
                "Value should be Ok",
                "Value should not be Ok",
            ),
            OkErrBased::Err => MatcherResult::new(
                value.is_err(),
                "Value should be Err",
                "Value should not be Err",
            ),
        }
    }
}

pub fn be_ok() -> OkErrBased {
    OkErrBased::Ok
}

pub fn be_err() -> OkErrBased {
    OkErrBased::Err
}

#[cfg(test)]
mod tests {
    use crate::assertions::bool::TrueFalseAssertions;
    use crate::matchers::result::{be_err, be_ok};
    use crate::matchers::Matcher;

    #[test]
    fn should_be_ok() {
        let matcher = be_ok();
        matcher.test(&Ok::<i32, String>(12)).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_ok_but_was_not() {
        let matcher = be_ok();
        matcher
            .test(&Err::<i32, &str>("test error"))
            .passed
            .should_be_true();
    }

    #[test]
    fn should_be_err() {
        let matcher = be_err();
        matcher
            .test(&Err::<i32, &str>("test error"))
            .passed
            .should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_err_but_was_not() {
        let matcher = be_err();
        matcher.test(&Ok::<i32, String>(12)).passed.should_be_true();
    }
}
