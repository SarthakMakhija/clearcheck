use crate::matchers::string::case::{be_lowercase, be_uppercase};
use crate::matchers::Should;
use crate::panicking::{assert_failed_unary, AssertKind};

pub trait Case {
    fn should_be_lower_case(&self) -> &Self;
    fn should_be_upper_case(&self) -> &Self;
}

impl Case for String {
    fn should_be_lower_case(&self) -> &Self {
        (self as &str).should_be_lower_case();
        self
    }

    fn should_be_upper_case(&self) -> &Self {
        (self as &str).should_be_upper_case();
        self
    }
}

impl Case for &str {
    fn should_be_lower_case(&self) -> &Self {
        if !self.should(&be_lowercase()) {
            assert_failed_unary(AssertKind::LowerCase, self);
        }
        self
    }

    fn should_be_upper_case(&self) -> &Self {
        if !self.should(&be_uppercase()) {
            assert_failed_unary(AssertKind::UpperCase, self);
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::assertions::string::case::Case;

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
