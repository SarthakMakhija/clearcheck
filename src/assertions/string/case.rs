use crate::matchers::string::case::{be_lowercase, be_uppercase};
use crate::matchers::Should;

pub trait CaseAssertion {
    fn should_be_lower_case(&self) -> &Self;
    fn should_be_upper_case(&self) -> &Self;
}

impl CaseAssertion for String {
    fn should_be_lower_case(&self) -> &Self {
        (self as &str).should_be_lower_case();
        self
    }

    fn should_be_upper_case(&self) -> &Self {
        (self as &str).should_be_upper_case();
        self
    }
}

impl CaseAssertion for &str {
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
