use regex::Regex;

use crate::matchers::string::regex::match_with;
use crate::matchers::{Should, ShouldNot};

pub trait RegularExpressionAssertion {
    fn should_match(&self, regex: Regex) -> &Self;
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
        self.should(&match_with(&regex));
        self
    }

    fn should_not_match(&self, regex: Regex) -> &Self {
        self.should_not(&match_with(&regex));
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
        let str = "Started assert4rs on On 2024-01-02.";
        str.should_match(regex);
    }

    #[test]
    #[should_panic]
    fn should_match_regular_expression_but_it_did_not() {
        let regex = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
        let str = "Started assert4rs on On 02nd January 2024";
        str.should_match(regex);
    }

    #[test]
    fn should_not_match_regular_expression() {
        let regex = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
        let str = "Started assert4rs on On 02nd January 2024";
        str.should_not_match(regex);
    }

    #[test]
    #[should_panic]
    fn should_not_match_regular_expression_but_it_did() {
        let regex = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
        let str = "Started assert4rs on On 2024-01-02.";
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
        let str = String::from("Started assert4rs on On 2024-01-02.");
        str.should_match(regex);
    }

    #[test]
    #[should_panic]
    fn should_match_regular_expression_but_it_did_not() {
        let regex = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
        let str = String::from("Started assert4rs on On 02nd January 2024");
        str.should_match(regex);
    }

    #[test]
    fn should_not_match_regular_expression() {
        let regex = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
        let str = String::from("Started assert4rs on On 02nd January 2024");
        str.should_not_match(regex);
    }

    #[test]
    #[should_panic]
    fn should_not_match_regular_expression_but_it_did() {
        let regex = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
        let str = String::from("Started assert4rs on On 2024-01-02.");
        str.should_not_match(regex);
    }
}
