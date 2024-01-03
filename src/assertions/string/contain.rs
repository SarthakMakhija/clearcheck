use crate::panicking::{assert_failed_binary, assert_failed_unary, AssertKind};

pub trait Contains {
    fn should_only_contain_digits(&self) -> &Self;
    fn should_contain_a_digit(&self) -> &Self;
    fn should_not_contain_digits(&self) -> &Self;
    fn should_contain_character(&self, ch: char) -> &Self;
    fn should_not_contain_character(&self, ch: char) -> &Self;
    fn should_contain(&self, substr: &str) -> &Self;
    fn should_not_contain(&self, substr: &str) -> &Self;
    fn should_contain_ignoring_case(&self, substr: &str) -> &Self;
    fn should_not_contain_ignoring_case(&self, substr: &str) -> &Self;
}

impl Contains for String {
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
}

impl Contains for &str {
    fn should_only_contain_digits(&self) -> &Self {
        let contains = self.chars().all(|ch| ch.is_numeric());
        if !contains {
            assert_failed_unary(AssertKind::ContainsOnlyDigits, self);
        }
        self
    }

    fn should_contain_a_digit(&self) -> &Self {
        let contains = self.chars().any(|ch| ch.is_numeric());
        if !contains {
            assert_failed_unary(AssertKind::ContainsADigit, self);
        }
        self
    }

    fn should_not_contain_digits(&self) -> &Self {
        let contains = self.chars().any(|ch| ch.is_numeric());
        if contains {
            assert_failed_unary(AssertKind::NotContainsDigits, self);
        }
        self
    }

    fn should_contain_character(&self, ch: char) -> &Self {
        let contains = self.chars().any(|source| source == ch);
        if !contains {
            assert_failed_binary(AssertKind::Contains, self, &ch);
        }
        self
    }

    fn should_not_contain_character(&self, ch: char) -> &Self {
        let contains = self.chars().any(|source| source == ch);
        if contains {
            assert_failed_binary(AssertKind::NotContains, self, &ch);
        }
        self
    }

    fn should_contain(&self, substr: &str) -> &Self {
        if !self.contains(substr) {
            assert_failed_binary(AssertKind::Contains, self, substr);
        }
        self
    }

    fn should_not_contain(&self, substr: &str) -> &Self {
        if self.contains(substr) {
            assert_failed_binary(AssertKind::NotContains, self, substr);
        }
        self
    }

    fn should_contain_ignoring_case(&self, substr: &str) -> &Self {
        if !self.to_lowercase().contains(&substr.to_lowercase()) {
            assert_failed_binary(AssertKind::Contains, self, substr);
        }
        self
    }

    fn should_not_contain_ignoring_case(&self, substr: &str) -> &Self {
        if self.to_lowercase().contains(&substr.to_lowercase()) {
            assert_failed_binary(AssertKind::NotContains, self, substr);
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::assertions::string::contain::Contains;

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
}
