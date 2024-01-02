use crate::panicking::{assert_failed_unary, AssertKind};

trait Case {
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
        if self != &self.to_lowercase() {
            assert_failed_unary(AssertKind::LowerCase, self);
        }
        self
    }

    fn should_be_upper_case(&self) -> &Self {
        if self != &self.to_uppercase() {
            assert_failed_unary(AssertKind::UpperCase, self);
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::matchers::string::case::Case;

    #[test]
    fn should_be_lower_case() {
        let name = "john";
        name.should_be_lower_case();
    }

    #[test]
    #[should_panic]
    fn should_be_lower_case_but_was_not() {
        let name = "John";
        name.should_be_lower_case();
    }

    #[test]
    fn should_be_upper_case() {
        let name = "JOHN";
        name.should_be_upper_case();
    }

    #[test]
    #[should_panic]
    fn should_be_upper_case_but_was_not() {
        let name = "John";
        name.should_be_upper_case();
    }
}