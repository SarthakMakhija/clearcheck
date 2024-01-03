use crate::panicking::{assert_failed_binary, AssertKind};

pub trait Equal {
    fn should_be_equal_ignoring_case(&self, other: &str) -> &Self;
    fn should_not_be_equal_ignoring_case(&self, other: &str) -> &Self;
}

impl Equal for String {
    fn should_be_equal_ignoring_case(&self, other: &str) -> &Self {
        (self as &str).should_be_equal_ignoring_case(other);
        self
    }

    fn should_not_be_equal_ignoring_case(&self, other: &str) -> &Self {
        (self as &str).should_not_be_equal_ignoring_case(other);
        self
    }
}

impl Equal for &str {
    fn should_be_equal_ignoring_case(&self, other: &str) -> &Self {
        if !self.to_lowercase().eq(&other.to_lowercase()) {
            assert_failed_binary(AssertKind::Equal, self, other);
        }
        self
    }

    fn should_not_be_equal_ignoring_case(&self, other: &str) -> &Self {
        if self.to_lowercase().eq(&other.to_lowercase()) {
            assert_failed_binary(AssertKind::NotEqual, self, other);
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::matchers::string::equal::Equal;

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