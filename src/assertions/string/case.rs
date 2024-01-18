use crate::matchers::Should;
use crate::matchers::string::case::{be_lowercase, be_uppercase};

/// CaseAssertion enables assertions about whether a string (or str) is lowercase or uppercase.
pub trait CaseAssertion {
    /// - Asserts that the string is lowercase.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::string::case::CaseAssertion;
    ///
    /// let name = "clearcheck";
    /// name.should_be_lower_case();
    /// ```
    fn should_be_lower_case(&self) -> &Self;

    /// - Asserts that the string is uppercase.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::string::case::CaseAssertion;
    ///
    /// let name = "CLEARCHECK";
    /// name.should_be_upper_case();
    /// ```
    fn should_be_upper_case(&self) -> &Self;
}

impl<T> CaseAssertion for T
    where T: AsRef<str> + PartialEq {
    fn should_be_lower_case(&self) -> &Self {
        self.should(&be_lowercase());
        self
    }

    fn should_be_upper_case(&self) -> &Self {
        self.should(&be_uppercase());
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::assertions::string::case::CaseAssertion;

    #[test]
    fn should_be_lower_case() {
        let name = "assert4j";
        name.should_be_lower_case();
    }

    #[test]
    #[should_panic]
    fn should_be_lower_case_but_was_not() {
        let name = "assert4J";
        name.should_be_lower_case();
    }

    #[test]
    fn should_be_upper_case() {
        let name = "ASSERT4J";
        name.should_be_upper_case();
    }

    #[test]
    #[should_panic]
    fn should_be_upper_case_but_was_not() {
        let name = "assert4J";
        name.should_be_upper_case();
    }
}

#[cfg(test)]
mod string_tests {
    use crate::assertions::string::case::CaseAssertion;

    #[test]
    fn should_be_lower_case() {
        let name = String::from("assert4j");
        name.should_be_lower_case();
    }

    #[test]
    #[should_panic]
    fn should_be_lower_case_but_was_not() {
        let name = String::from("ASSERT4J");
        name.should_be_lower_case();
    }

    #[test]
    fn should_be_upper_case() {
        let name = String::from("ASSERT4J");
        name.should_be_upper_case();
    }

    #[test]
    #[should_panic]
    fn should_be_upper_case_but_was_not() {
        let name = String::from("assert4J");
        name.should_be_upper_case();
    }
}
