use regex::Regex;

use crate::matchers::string::regex::match_with;
use crate::matchers::{Should, ShouldNot};

/// RegularExpressionAssertion enables assertions about whether a string (or str) matches a regular expression.
pub trait RegularExpressionAssertion {
    /// - Asserts that the string matches the given regular expression.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use regex::Regex;
    /// use clearcheck::assertions::string::regex::RegularExpressionAssertion;
    ///
    /// let regex = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    /// let phrase = "Started clearcheck on 2024-01-02.";
    /// phrase.should_match(regex);
    /// ```
    fn should_match(&self, regex: Regex) -> &Self;

    /// - Asserts that the string does not match the given regular expression.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use regex::Regex;
    /// use clearcheck::assertions::string::regex::RegularExpressionAssertion;
    ///
    /// let regex = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    /// let phrase = String::from("Started clearcheck on 02nd January 2024");
    /// phrase.should_not_match(regex);
    /// ```
    fn should_not_match(&self, regex: Regex) -> &Self;
}

impl RegularExpressionAssertion for String {
    fn should_match(&self, regex: Regex) -> &Self {
        (self as &str).should_match(regex);
        self
    }

    fn should_not_match(&self, regex: Regex) -> &Self {
        (self as &str).should_not_match(regex);
        self
    }
}

impl RegularExpressionAssertion for &str {
    fn should_match(&self, regex: Regex) -> &Self {
        self.should(&match_with(regex));
        self
    }

    fn should_not_match(&self, regex: Regex) -> &Self {
        self.should_not(&match_with(regex));
        self
    }
}

#[cfg(all(test, feature = "regex"))]
mod tests {
    use crate::assertions::string::regex::RegularExpressionAssertion;
    use regex::Regex;

    #[test]
    fn should_match_regular_expression() {
        let regex = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
        let str = "Started clearcheck on On 2024-01-02.";
        str.should_match(regex);
    }

    #[test]
    #[should_panic]
    fn should_match_regular_expression_but_it_did_not() {
        let regex = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
        let str = "Started clearcheck on On 02nd January 2024";
        str.should_match(regex);
    }

    #[test]
    fn should_not_match_regular_expression() {
        let regex = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
        let str = "Started clearcheck on On 02nd January 2024";
        str.should_not_match(regex);
    }

    #[test]
    #[should_panic]
    fn should_not_match_regular_expression_but_it_did() {
        let regex = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
        let str = "Started clearcheck on On 2024-01-02.";
        str.should_not_match(regex);
    }
}

#[cfg(all(test, feature = "regex"))]
mod string_tests {
    use crate::assertions::string::regex::RegularExpressionAssertion;
    use regex::Regex;

    #[test]
    fn should_match_regular_expression() {
        let regex = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
        let str = String::from("Started clearcheck on 2024-01-02.");
        str.should_match(regex);
    }

    #[test]
    #[should_panic]
    fn should_match_regular_expression_but_it_did_not() {
        let regex = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
        let str = String::from("Started clearcheck on 02nd January 2024");
        str.should_match(regex);
    }

    #[test]
    fn should_not_match_regular_expression() {
        let regex = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
        let str = String::from("Started clearcheck on 02nd January 2024");
        str.should_not_match(regex);
    }

    #[test]
    #[should_panic]
    fn should_not_match_regular_expression_but_it_did() {
        let regex = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
        let str = String::from("Started clearcheck on 2024-01-02.");
        str.should_not_match(regex);
    }
}
