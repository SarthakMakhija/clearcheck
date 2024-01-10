use std::fmt::Debug;

use crate::matchers::result::{be_err, be_ok};
use crate::matchers::Should;

/// OkErrAssertion enables assertions about whether a Result evaluates to Ok or Err.
pub trait OkErrAssertion {
    /// - Asserts that the Result evaluates to Ok.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::result::OkErrAssertion;
    ///
    /// let value: Result<i32, &str> = Ok(32);
    /// value.should_be_ok();
    /// ```
    fn should_be_ok(&self) -> &Self;

    /// - Asserts that the Result evaluates to Err.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::result::OkErrAssertion;
    ///
    /// let value: Result<i32, &str> = Err("example error");
    /// value.should_be_err();
    /// ```
    fn should_be_err(&self) -> &Self;
}

impl<T, E> OkErrAssertion for Result<T, E>
where
    T: Debug,
    E: Debug,
{
    fn should_be_ok(&self) -> &Self {
        self.should(&be_ok());
        self
    }

    fn should_be_err(&self) -> &Self {
        self.should(&be_err());
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::assertions::result::OkErrAssertion;

    #[test]
    fn should_be_ok() {
        let result: Result<i32, &str> = Ok(100);
        result.should_be_ok();
    }

    #[test]
    #[should_panic]
    fn should_be_ok_but_was_not() {
        let result: Result<i32, &str> = Err("test error");
        result.should_be_ok();
    }

    #[test]
    fn should_be_err() {
        let result: Result<i32, &str> = Err("test error");
        result.should_be_err();
    }

    #[test]
    #[should_panic]
    fn should_be_err_but_was_not() {
        let result: Result<i32, &str> = Ok(100);
        result.should_be_err();
    }
}
