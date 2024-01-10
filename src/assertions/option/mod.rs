use std::fmt::Debug;

use crate::matchers::option::{be_none, be_some};
use crate::matchers::Should;

/// SomeNoneAssertion enables assertions about whether an Option evaluates to Some or None.
pub trait SomeNoneAssertion {
    /// - Asserts that the Option evaluates to Some.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::option::SomeNoneAssertion;
    ///
    /// let value = Some(32);
    /// value.should_be_some();
    /// ```
    fn should_be_some(&self) -> &Self;

    /// - Asserts that the Option evaluates to None.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::option::SomeNoneAssertion;
    ///
    /// let value: Option<i32> = None;
    /// value.should_be_none();
    /// ```
    fn should_be_none(&self) -> &Self;
}

impl<T> SomeNoneAssertion for Option<T>
where
    T: Debug,
{
    fn should_be_some(&self) -> &Self {
        self.should(&be_some());
        self
    }

    fn should_be_none(&self) -> &Self {
        self.should(&be_none());
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::assertions::option::SomeNoneAssertion;

    #[test]
    fn should_be_none() {
        let option: Option<i32> = None;
        option.should_be_none();
    }

    #[test]
    #[should_panic]
    fn should_be_none_but_was_not() {
        let option = Some("junit");
        option.should_be_none();
    }

    #[test]
    fn should_be_some() {
        let option = Some("junit");
        option.should_be_some();
    }

    #[test]
    #[should_panic]
    fn should_be_some_but_was_not() {
        let option: Option<i32> = None;
        option.should_be_some();
    }
}
