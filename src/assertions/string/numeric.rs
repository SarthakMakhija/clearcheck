use std::str::FromStr;

use crate::matchers::{Should, ShouldNot};
use crate::matchers::string::numeric::be_numeric;

/// NumericAssertion enables assertions about whether a string (or str) is numeric.
pub trait NumericAssertion {
    /// - Asserts that the string is numeric.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::string::numeric::NumericAssertion;
    ///
    /// let value = "12345";
    /// value.should_be_numeric::<i32>();
    /// ```
    fn should_be_numeric<T: FromStr>(&self) -> &Self;

    /// - Asserts that the string is not numeric.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::string::numeric::NumericAssertion;
    ///
    /// let name = "assert4j";
    /// name.should_not_be_numeric::<i32>();
    /// ```
    fn should_not_be_numeric<T: FromStr>(&self) -> &Self;
}

impl<S> NumericAssertion for S
    where S: AsRef<str>
{
    fn should_be_numeric<T: FromStr>(&self) -> &Self {
        self.should(&be_numeric::<T>());
        self
    }

    fn should_not_be_numeric<T: FromStr>(&self) -> &Self {
        self.should_not(&be_numeric::<T>());
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::assertions::string::numeric::NumericAssertion;

    #[test]
    fn should_be_numeric() {
        let value = "1234";
        value.should_be_numeric::<i32>();
    }

    #[test]
    #[should_panic]
    fn should_be_numeric_but_was_not() {
        let value = "1234a";
        value.should_be_numeric::<i32>();
    }

    #[test]
    fn should_not_be_numeric() {
        let value = "1234a";
        value.should_not_be_numeric::<i32>();
    }

    #[test]
    #[should_panic]
    fn should_not_be_numeric_but_was() {
        let value = "1234";
        value.should_not_be_numeric::<i32>();
    }
}

#[cfg(test)]
mod string_tests {
    use crate::assertions::string::numeric::NumericAssertion;

    #[test]
    fn should_be_numeric() {
        let value = String::from("1234");
        value.should_be_numeric::<i32>();
    }

    #[test]
    #[should_panic]
    fn should_be_numeric_but_was_not() {
        let value = String::from("1234a");
        value.should_be_numeric::<i32>();
    }

    #[test]
    fn should_not_be_numeric() {
        let value = String::from("1234a");
        value.should_not_be_numeric::<i32>();
    }

    #[test]
    #[should_panic]
    fn should_not_be_numeric_but_was() {
        let value = String::from("1234");
        value.should_not_be_numeric::<i32>();
    }
}
