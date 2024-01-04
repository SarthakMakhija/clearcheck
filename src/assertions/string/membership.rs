use crate::matchers::string::membership::{
    be_empty, contain, contain_a_digit, contain_character, contain_ignoring_case,
    contain_only_digits, not_contain_digits,
};
use crate::matchers::{Should, ShouldNot};
use crate::panicking::{assert_failed_binary, assert_failed_unary, AssertKind};

pub trait Membership {
    fn should_only_contain_digits(&self) -> &Self;
    fn should_contain_a_digit(&self) -> &Self;
    fn should_not_contain_digits(&self) -> &Self;
    fn should_contain_character(&self, ch: char) -> &Self;
    fn should_not_contain_character(&self, ch: char) -> &Self;
    fn should_contain(&self, substr: &str) -> &Self;
    fn should_not_contain(&self, substr: &str) -> &Self;
    fn should_contain_ignoring_case(&self, substr: &str) -> &Self;
    fn should_not_contain_ignoring_case(&self, substr: &str) -> &Self;
    fn should_be_empty(&self) -> &Self;
    fn should_not_be_empty(&self) -> &Self;
}

impl Membership for String {
    fn should_only_contain_digits(&self) -> &Self {
        (self as &str).should_only_contain_digits();
        self
    }

    fn should_contain_a_digit(&self) -> &Self {
        (self as &str).should_contain_a_digit();
        self
    }

    fn should_not_contain_digits(&self) -> &Self {
        (self as &str).should_not_contain_digits();
        self
    }

    fn should_contain_character(&self, ch: char) -> &Self {
        (self as &str).should_contain_character(ch);
        self
    }

    fn should_not_contain_character(&self, ch: char) -> &Self {
        (self as &str).should_not_contain_character(ch);
        self
    }

    fn should_contain(&self, substr: &str) -> &Self {
        (self as &str).should_contain(substr);
        self
    }

    fn should_not_contain(&self, substr: &str) -> &Self {
        (self as &str).should_not_contain(substr);
        self
    }

    fn should_contain_ignoring_case(&self, substr: &str) -> &Self {
        (self as &str).should_contain_ignoring_case(substr);
        self
    }

    fn should_not_contain_ignoring_case(&self, substr: &str) -> &Self {
        (self as &str).should_not_contain_ignoring_case(substr);
        self
    }

    fn should_be_empty(&self) -> &Self {
        (self as &str).should_be_empty();
        self
    }

    fn should_not_be_empty(&self) -> &Self {
        (self as &str).should_not_be_empty();
        self
    }
}

impl Membership for &str {
    fn should_only_contain_digits(&self) -> &Self {
        if !self.should(&contain_only_digits()) {
            assert_failed_unary(AssertKind::ContainsOnlyDigits, self);
        }
        self
    }

    fn should_contain_a_digit(&self) -> &Self {
        if !self.should(&contain_a_digit()) {
            assert_failed_unary(AssertKind::ContainsADigit, self);
        }
        self
    }

    fn should_not_contain_digits(&self) -> &Self {
        if !self.should(&not_contain_digits()) {
            assert_failed_unary(AssertKind::NotContainsDigits, self);
        }
        self
    }

    fn should_contain_character(&self, ch: char) -> &Self {
        if !self.should(&contain_character(ch)) {
            assert_failed_binary(AssertKind::Contains, self, &ch);
        }
        self
    }

    fn should_not_contain_character(&self, ch: char) -> &Self {
        if !self.should_not(&contain_character(ch)) {
            assert_failed_binary(AssertKind::NotContains, self, &ch);
        }
        self
    }

    fn should_contain(&self, substr: &str) -> &Self {
        if !self.should(&contain(substr)) {
            assert_failed_binary(AssertKind::Contains, self, substr);
        }
        self
    }

    fn should_not_contain(&self, substr: &str) -> &Self {
        if !self.should_not(&contain(substr)) {
            assert_failed_binary(AssertKind::NotContains, self, substr);
        }
        self
    }

    fn should_contain_ignoring_case(&self, substr: &str) -> &Self {
        if !self.should(&contain_ignoring_case(substr)) {
            assert_failed_binary(AssertKind::Contains, self, substr);
        }
        self
    }

    fn should_not_contain_ignoring_case(&self, substr: &str) -> &Self {
        if !self.should_not(&contain_ignoring_case(substr)) {
            assert_failed_binary(AssertKind::NotContains, self, substr);
        }
        self
    }

    fn should_be_empty(&self) -> &Self {
        if !self.should(&be_empty()) {
            assert_failed_unary(AssertKind::Empty, self);
        }
        self
    }

    fn should_not_be_empty(&self) -> &Self {
        if !self.should_not(&be_empty()) {
            assert_failed_unary(AssertKind::NotEmpty, self);
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::assertions::string::membership::Membership;

    #[test]
    fn should_only_contain_digits() {
        let str = "12345";
        str.should_only_contain_digits();
    }

    #[test]
    #[should_panic]
    fn should_only_contain_digits_but_it_did_not() {
        let str = "12345a";
        str.should_only_contain_digits();
    }

    #[test]
    fn should_contain_a_digit() {
        let email = "john1@gmail.com";
        email.should_contain_a_digit();
    }

    #[test]
    #[should_panic]
    fn should_contain_a_digit_but_it_did_not() {
        let email = "john@gmail.com";
        email.should_contain_a_digit();
    }

    #[test]
    fn should_not_contain_digits() {
        let email = "john@gmail.com";
        email.should_not_contain_digits();
    }

    #[test]
    #[should_panic]
    fn should_not_contain_digits_but_it_did() {
        let email = "john1@gmail.com";
        email.should_not_contain_digits();
    }

    #[test]
    fn should_contain_the_character() {
        let email = "john@gmail.com";
        email.should_contain_character('@');
    }

    #[test]
    #[should_panic]
    fn should_contain_the_character_but_it_did_not() {
        let email = "john1@gmail.com";
        email.should_contain_character('#');
    }

    #[test]
    fn should_not_contain_the_character() {
        let email = "john@gmail.com";
        email.should_not_contain_character('#');
    }

    #[test]
    #[should_panic]
    fn should_not_contain_the_character_but_it_did() {
        let email = "john1@gmail.com";
        email.should_not_contain_character('@');
    }

    #[test]
    fn should_contain_substring() {
        let email = "john@gmail.com";
        email.should_contain("gmail");
    }

    #[test]
    #[should_panic]
    fn should_contain_substring_but_it_did_not() {
        let email = "john1@gmail.com";
        email.should_contain("yahoo");
    }

    #[test]
    fn should_not_contain_substring() {
        let email = "john@gmail.com";
        email.should_not_contain("yahoo");
    }

    #[test]
    #[should_panic]
    fn should_not_contain_substring_but_it_did() {
        let email = "john1@gmail.com";
        email.should_not_contain("gmail");
    }

    #[test]
    fn should_contain_substring_ignoring_case() {
        let email = "john@gmail.com";
        email.should_contain_ignoring_case("GMAIL");
    }

    #[test]
    #[should_panic]
    fn should_contain_substring_ignoring_case_but_it_did_not() {
        let email = "john1@gmail.com";
        email.should_contain_ignoring_case("YAHOO");
    }

    #[test]
    fn should_not_contain_substring_ignoring_case() {
        let email = "john@gmail.com";
        email.should_not_contain_ignoring_case("YAHOO");
    }

    #[test]
    #[should_panic]
    fn should_not_contain_substring_ignoring_case_but_it_did() {
        let email = "john1@gmail.com";
        email.should_not_contain_ignoring_case("GMAIL");
    }

    #[test]
    fn should_be_empty() {
        let name = "";
        name.should_be_empty();
    }

    #[test]
    #[should_panic]
    fn should_be_empty_but_was_not() {
        let name = "John";
        name.should_be_empty();
    }

    #[test]
    fn should_not_be_empty() {
        let name = "John";
        name.should_not_be_empty();
    }

    #[test]
    #[should_panic]
    fn should_not_be_empty_but_was() {
        let name = "";
        name.should_not_be_empty();
    }
}