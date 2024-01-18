use crate::matchers::{Should, ShouldNot};
use crate::matchers::option::be_some;
use crate::matchers::option::predicate::satisfy;

/// SomePredicateAssertion enables assertions about whether the Option value is both Some and that the contained value meets or doesn't meet certain conditions defined by a predicate.
pub trait SomePredicateAssertion<T> {
    /// - Asserts that the Option value is Some and satisfies the given predicate.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::option::predicate::SomePredicateAssertion;
    ///
    /// let option = Some(100);
    /// option.should_be_some_and_satisfy(|value| value > &&50);
    /// ```
    fn should_be_some_and_satisfy<F: Fn(&&T) -> bool>(&self, predicate: F) -> &Self;

    /// - Asserts that the Option value is Some and does not satisfy the given predicate.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::option::predicate::SomePredicateAssertion;
    ///
    /// let option = Some(100);
    /// option.should_be_some_and_not_satisfy(|value| value > &&500);
    /// ```
    fn should_be_some_and_not_satisfy<F: Fn(&&T) -> bool>(&self, predicate: F) -> &Self;
}

impl<T> SomePredicateAssertion<T> for Option<T> {
    fn should_be_some_and_satisfy<F: Fn(&&T) -> bool>(&self, predicate: F) -> &Self {
        self.should(&be_some());
        self.should(&satisfy(predicate));
        self
    }

    fn should_be_some_and_not_satisfy<F: Fn(&&T) -> bool>(&self, predicate: F) -> &Self {
        self.should(&be_some());
        self.should_not(&satisfy(predicate));
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::assertions::option::predicate::SomePredicateAssertion;

    #[test]
    fn should_be_some_and_satisfy_predicate() {
        let option = Some(100);
        option.should_be_some_and_satisfy(|value| value > &&50);
    }

    #[test]
    #[should_panic]
    fn should_be_some_and_satisfy_predicate_but_it_did_not() {
        let option = Some(100);
        option.should_be_some_and_satisfy(|value| value > &&100);
    }

    #[test]
    #[should_panic]
    fn should_be_some_and_satisfy_predicate_but_it_was_none() {
        let option: Option<i32> = None;
        option.should_be_some_and_satisfy(|value| value > &&100);
    }

    #[test]
    fn should_be_some_and_not_satisfy_predicate() {
        let option = Some(100);
        option.should_be_some_and_not_satisfy(|value| value > &&500);
    }

    #[test]
    #[should_panic]
    fn should_be_some_and_not_satisfy_predicate_but_it_did() {
        let option = Some(100);
        option.should_be_some_and_not_satisfy(|value| value > &&50);
    }

    #[test]
    #[should_panic]
    fn should_be_some_and_not_satisfy_predicate_but_it_was_none() {
        let option: Option<i32> = None;
        option.should_be_some_and_not_satisfy(|value| value > &&100);
    }
}
