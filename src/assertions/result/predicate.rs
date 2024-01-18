use crate::matchers::{Should, ShouldNot};
use crate::matchers::result::predicate::satisfy;
use crate::matchers::result::be_ok;

/// OkPredicateAssertion enables assertions about whether the Result value is both Ok and that the contained value meets or doesn't meet certain conditions defined by a predicate.
pub trait OkPredicateAssertion<T> {
    /// - Asserts that the Result value is Ok and satisfies the given predicate.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::result::predicate::OkPredicateAssertion;
    ///
    /// let value: Result<i32, &str> = Ok(1000);
    /// value.should_be_ok_and_satisfy(|value| value > &50);
    /// ```
    fn should_be_ok_and_satisfy<F: Fn(&T) -> bool>(&self, predicate: F) -> &Self;

    /// - Asserts that the Result value is Ok and does not satisfy the given predicate.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::result::predicate::OkPredicateAssertion;
    ///
    /// let value: Result<i32, &str> = Ok(100);
    /// value.should_be_ok_and_not_satisfy(|value| value > &500);
    /// ```
    fn should_be_ok_and_not_satisfy<F: Fn(&T) -> bool>(&self, predicate: F) -> &Self;
}

impl<T, E> OkPredicateAssertion<T> for Result<T, E> {
    fn should_be_ok_and_satisfy<F: Fn(&T) -> bool>(&self, predicate: F) -> &Self {
        self.should(&be_ok());
        self.should(&satisfy(predicate));
        self
    }

    fn should_be_ok_and_not_satisfy<F: Fn(&T) -> bool>(&self, predicate: F) -> &Self {
        self.should(&be_ok());
        self.should_not(&satisfy(predicate));
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::assertions::result::predicate::OkPredicateAssertion;

    #[test]
    fn should_be_ok_and_satisfy_predicate() {
        let value: Result<i32, &str> = Ok(1000);
        value.should_be_ok_and_satisfy(|value| value > &50);
    }

    #[test]
    #[should_panic]
    fn should_be_ok_and_satisfy_predicate_but_it_did_not() {
        let value: Result<i32, &str> = Ok(10);
        value.should_be_ok_and_satisfy(|value| value > &100);
    }

    #[test]
    #[should_panic]
    fn should_be_ok_and_satisfy_predicate_but_it_was_err() {
        let value: Result<i32, &str> = Err("test error");
        value.should_be_ok_and_satisfy(|value| value > &100);
    }

    #[test]
    fn should_be_ok_and_not_satisfy_predicate() {
        let value: Result<i32, &str> = Ok(100);
        value.should_be_ok_and_not_satisfy(|value| value > &500);
    }

    #[test]
    #[should_panic]
    fn should_be_ok_and_not_satisfy_predicate_but_it_did() {
        let value: Result<i32, &str> = Ok(100);
        value.should_be_ok_and_not_satisfy(|value| value > &50);
    }

    #[test]
    #[should_panic]
    fn should_be_ok_and_not_satisfy_predicate_but_it_was_err() {
        let value: Result<i32, &str> = Err("test error");
        value.should_be_ok_and_not_satisfy(|value| value > &100);
    }
}
