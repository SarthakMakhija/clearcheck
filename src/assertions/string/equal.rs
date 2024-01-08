use crate::matchers::equal::be_equal_ignoring_case;
use crate::matchers::{Should, ShouldNot};

pub trait IgnoreCaseEqualityAssertion {
    fn should_be_equal_ignoring_case(&self, other: &str) -> &Self;
    fn should_not_be_equal_ignoring_case(&self, other: &str) -> &Self;
}

impl IgnoreCaseEqualityAssertion for String {
    fn should_be_equal_ignoring_case(&self, other: &str) -> &Self {
        (self as &str).should_be_equal_ignoring_case(other);
        self
    }

    fn should_not_be_equal_ignoring_case(&self, other: &str) -> &Self {
        (self as &str).should_not_be_equal_ignoring_case(other);
        self
    }
}

impl IgnoreCaseEqualityAssertion for &str {
    fn should_be_equal_ignoring_case(&self, other: &str) -> &Self {
        self.should(&be_equal_ignoring_case(&other));
        self
    }

    fn should_not_be_equal_ignoring_case(&self, other: &str) -> &Self {
        self.should_not(&be_equal_ignoring_case(&other));
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
