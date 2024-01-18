use std::marker::PhantomData;
use crate::matchers::{Matcher, MatcherResult};

/// OkPredicateMatcher offers a flexible way to assert whether the Result value is both Ok and that the contained value meets certain conditions defined by the predicate.
///
/// # Example
///```
/// use clearcheck::matchers::Matcher;
/// use clearcheck::matchers::result::predicate::satisfy;
///
/// let matcher = satisfy(|value| value > &400);
/// let value: Result<i32, &str> = Ok(1000);
///
/// assert!(matcher.test(&value).passed());
/// ```
pub struct OkPredicateMatcher<F, T>
    where F: Fn(&T) -> bool
{
    predicate: F,
    _inner: PhantomData<T>,
}

impl<F, T, E> Matcher<Result<T, E>> for OkPredicateMatcher<F, T>
    where F: Fn(&T) -> bool
{
    fn test(&self, value: &Result<T, E>) -> MatcherResult {
        MatcherResult::new(
            value.as_ref().is_ok_and(&self.predicate),
            "Result value should satisfy the given predicate",
            "Result value should not satisfy the given predicate",
        )
    }
}

/// Creates an OkPredicateMatcher that asserts whether the Result value is both Ok and that the contained value meets certain conditions defined by the predicate.
pub fn satisfy<F, T>(predicate: F) -> OkPredicateMatcher<F, T>
    where F: Fn(&T) -> bool
{
    OkPredicateMatcher {
        predicate,
        _inner: PhantomData,
    }
}

#[cfg(test)]
mod tests {
    use crate::assertions::bool::TrueFalseAssertion;
    use crate::matchers::Matcher;
    use crate::matchers::result::predicate::satisfy;

    #[test]
    fn should_be_ok_and_satisfy_the_predicate() {
        let matcher = satisfy(|value| value > &400);
        let value: Result<i32, &str> = Ok(1000);

        matcher.test(&value).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_ok_and_satisfy_the_predicate_but_did_not() {
        let matcher = satisfy(|value| value > &400);
        let value: Result<i32, &str> = Ok(100);

        matcher.test(&value).passed.should_be_true();
    }
}