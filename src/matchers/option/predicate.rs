use std::marker::PhantomData;

use crate::matchers::{Matcher, MatcherResult};

/// SomePredicateMatcher offers a flexible way to assert whether the Option value is both Some and that the contained value meets certain conditions defined by the predicate.
///
/// # Example
///```
/// use clearcheck::matchers::Matcher;
/// use clearcheck::matchers::option::predicate::satisfy;
///
/// let matcher = satisfy(|value| value > &&400);
/// assert!(matcher.test(&Some(1000)).passed());
/// ```
pub struct SomePredicateMatcher<F, T>
    where F: Fn(&&T) -> bool
{
    predicate: F,
    _inner: PhantomData<T>,
}

impl<F, T> Matcher<Option<T>> for SomePredicateMatcher<F, T>
    where F: Fn(&&T) -> bool
{
    fn test(&self, value: &Option<T>) -> MatcherResult {
        MatcherResult::new(
            value.as_ref().filter(&self.predicate).is_some(),
            "Option value should satisfy the given predicate",
            "Option value should not satisfy the given predicate",
        )
    }
}

/// Creates a SomePredicateMatcher that asserts whether the Option value is both Some and that the contained value meets certain conditions defined by the predicate.
pub fn satisfy<F, T>(predicate: F) -> SomePredicateMatcher<F, T>
    where F: Fn(&&T) -> bool
{
    SomePredicateMatcher {
        predicate,
        _inner: PhantomData,
    }
}

#[cfg(test)]
mod tests {
    use crate::assertions::bool::TrueFalseAssertion;
    use crate::matchers::Matcher;
    use crate::matchers::option::predicate::satisfy;

    #[test]
    fn should_be_some_and_satisfy_the_predicate() {
        let matcher = satisfy(|value| value > &&400);
        matcher.test(&Some(1000)).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_some_and_satisfy_the_predicate_but_did_not() {
        let matcher = satisfy(|value| value > &&400);
        matcher.test(&Some(100)).passed.should_be_true();
    }
}