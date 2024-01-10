use crate::matchers::bool::{be_false, be_true};
use crate::matchers::Should;

/// TrueFalseAssertion enables assertions about whether a boolean evaluates to true or false.
pub trait TrueFalseAssertion {
    /// - Asserts that the boolean evaluates to true.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::bool::TrueFalseAssertion;
    ///
    /// let value = true;
    /// value.should_be_true();
    /// ```
    fn should_be_true(&self) -> &Self;

    /// - Asserts that the boolean evaluates to false.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::bool::TrueFalseAssertion;
    ///
    /// let value = false;
    /// value.should_be_false();
    /// ```
    fn should_be_false(&self) -> &Self;
}

impl TrueFalseAssertion for bool {
    fn should_be_true(&self) -> &Self {
        self.should(&be_true());
        self
    }

    fn should_be_false(&self) -> &Self {
        self.should(&be_false());
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::assertions::bool::TrueFalseAssertion;

    #[test]
    fn should_be_true() {
        let value = true;
        value.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_true_but_was_not() {
        let value = false;
        value.should_be_true();
    }

    #[test]
    fn should_be_false() {
        let value = false;
        value.should_be_false();
    }

    #[test]
    #[should_panic]
    fn should_be_false_but_was_not() {
        let value = true;
        value.should_be_false();
    }
}
