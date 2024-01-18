use crate::matchers::{Should, ShouldNot};
use crate::matchers::equal::be_equal_ignoring_case;

/// IgnoreCaseEqualityAssertion enables assertions about whether a string (or str) equals other string, with case ignored.
pub trait IgnoreCaseEqualityAssertion {
    /// - Asserts that the string equals other string, with case ignored.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::string::equal::IgnoreCaseEqualityAssertion;
    ///
    /// let name = "clearcheck";
    /// name.should_be_equal_ignoring_case("CLearCheck");
    /// ```
    fn should_be_equal_ignoring_case(&self, other: &str) -> &Self;

    /// - Asserts that the string does not equal other string, with case ignored.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::string::equal::IgnoreCaseEqualityAssertion;
    ///
    /// let name = "clearcheck";
    /// name.should_not_be_equal_ignoring_case("CLEARCHECK-001");
    /// ```
    fn should_not_be_equal_ignoring_case(&self, other: &str) -> &Self;
}

impl<T> IgnoreCaseEqualityAssertion for T
    where T: AsRef<str> {
    fn should_be_equal_ignoring_case(&self, other: &str) -> &Self {
        self.should(&be_equal_ignoring_case(other));
        self
    }

    fn should_not_be_equal_ignoring_case(&self, other: &str) -> &Self {
        self.should_not(&be_equal_ignoring_case(other));
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::assertions::string::equal::IgnoreCaseEqualityAssertion;

    #[test]
    fn should_be_equal() {
        let name = "john";
        name.should_be_equal_ignoring_case("JOHN");
    }

    #[test]
    #[should_panic]
    fn should_be_equal_but_was_not() {
        let name = "johnR";
        name.should_be_equal_ignoring_case("JOHN");
    }

    #[test]
    fn should_not_be_equal() {
        let name = "john";
        name.should_not_be_equal_ignoring_case("JOHN-R");
    }

    #[test]
    #[should_panic]
    fn should_not_be_equal_but_was() {
        let name = "john";
        name.should_not_be_equal_ignoring_case("JOHN");
    }
}

#[cfg(test)]
mod string_tests {
    use crate::assertions::string::equal::IgnoreCaseEqualityAssertion;

    #[test]
    fn should_be_equal() {
        let name = String::from("john");
        name.should_be_equal_ignoring_case("JOHN");
    }

    #[test]
    #[should_panic]
    fn should_be_equal_but_was_not() {
        let name = String::from("johnR");
        name.should_be_equal_ignoring_case("JOHN");
    }

    #[test]
    fn should_not_be_equal() {
        let name = String::from("john");
        name.should_not_be_equal_ignoring_case("JOHN-R");
    }

    #[test]
    #[should_panic]
    fn should_not_be_equal_but_was() {
        let name = String::from("john");
        name.should_not_be_equal_ignoring_case("JOHN");
    }
}
