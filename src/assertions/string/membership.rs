use crate::matchers::{Should, ShouldNot};
use crate::matchers::string::empty::be_empty;
use crate::matchers::string::membership::{contain, contain_a_digit, contain_all_characters, contain_any_of_characters, contain_character, contain_ignoring_case, contain_only_digits, not_contain_digits};

/// MembershipAssertion enables assertions about the presence or absence of characters, substrings, or digits within string (or str) values.
///
/// It offers a fluent interface for chaining multiple assertions.
///
/// # Example
/// ```
/// use clearcheck::assertions::string::membership::MembershipAssertion;
///
/// let pass_phrase = "P@ssw0rd123 phrase alpha#";
/// pass_phrase
///     .should_contain_a_digit()
///     .should_not_be_empty()
///     .should_contain_any_characters(vec!['@', '#'])
///     .should_not_contain_ignoring_case("pass");
/// ```
pub trait MembershipAssertion {
    /// - Asserts that the string contains only digits.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::string::membership::MembershipAssertion;
    ///
    /// let value = "12345";
    /// value.should_only_contain_digits();
    /// ```
    fn should_only_contain_digits(&self) -> &Self;

    /// - Asserts that the string contains a digit.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::string::membership::MembershipAssertion;
    ///
    /// let value = "assert4j";
    /// value.should_contain_a_digit();
    /// ```
    fn should_contain_a_digit(&self) -> &Self;

    /// - Asserts that the string does not contain any digits.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::string::membership::MembershipAssertion;
    ///
    /// let value = "assert";
    /// value.should_not_contain_digits();
    /// ```
    fn should_not_contain_digits(&self) -> &Self;

    /// - Asserts that the string contains the given character.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::string::membership::MembershipAssertion;
    ///
    /// let email = "john@gmail.com";
    /// email.should_contain_character('@');
    /// ```
    fn should_contain_character(&self, ch: char) -> &Self;

    /// - Asserts that the string does not contain the given character.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::string::membership::MembershipAssertion;
    ///
    /// let email = "john@gmail.com";
    /// email.should_not_contain_character('#');
    /// ```
    fn should_not_contain_character(&self, ch: char) -> &Self;

    /// - Asserts that the string contains all the given characters.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::string::membership::MembershipAssertion;
    ///
    /// let email = "john@gmail.com";
    /// email.should_contain_all_characters(vec!['@', '.']);
    /// ```
    fn should_contain_all_characters(&self, chars: Vec<char>) -> &Self;

    /// - Asserts that the string does not contain all the given characters.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::string::membership::MembershipAssertion;
    ///
    /// let email = "john@gmail.com";
    /// email.should_not_contain_all_characters(vec!['@', '.', '#']);
    /// ```
    fn should_not_contain_all_characters(&self, chars: Vec<char>) -> &Self;

    /// - Asserts that the string contains any of the given characters.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::string::membership::MembershipAssertion;
    ///
    /// let email = "john@gmail.com";
    /// email.should_contain_any_characters(vec!['@', '.', '#']);
    /// ```
    fn should_contain_any_characters(&self, chars: Vec<char>) -> &Self;

    /// - Asserts that the string does not contain any of the given characters.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::string::membership::MembershipAssertion;
    ///
    /// let email = "john@gmail.com";
    /// email.should_not_contain_any_characters(vec!['#', '%', '?']);
    /// ```
    fn should_not_contain_any_characters(&self, chars: Vec<char>) -> &Self;

    /// - Asserts that the string contains the given substring.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::string::membership::MembershipAssertion;
    ///
    /// let email = "john@gmail.com";
    /// email.should_contain("gmail");
    /// ```
    fn should_contain(&self, substr: &'static str) -> &Self;

    /// - Asserts that the string does not contain the given substring.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::string::membership::MembershipAssertion;
    ///
    /// let email = "john@gmail.com";
    /// email.should_not_contain("yahoo");
    /// ```
    fn should_not_contain(&self, substr: &'static str) -> &Self;

    /// - Asserts that the string contains the substring, ignoring case differences.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::string::membership::MembershipAssertion;
    ///
    /// let email = "john@gmail.com";
    /// email.should_contain_ignoring_case("GMAIL");
    /// ```
    fn should_contain_ignoring_case(&self, substr: &'static str) -> &Self;

    /// - Asserts that the string does not contain the substring, ignoring case differences.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::string::membership::MembershipAssertion;
    ///
    /// let email = "john@gmail.com";
    /// email.should_not_contain_ignoring_case("YaHoo");
    /// ```
    fn should_not_contain_ignoring_case(&self, substr: &'static str) -> &Self;

    /// - Asserts that the string is empty (has zero characters).
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::string::membership::MembershipAssertion;
    ///
    /// let email = "";
    /// email.should_be_empty();
    /// ```
    fn should_be_empty(&self) -> &Self;

    /// - Asserts that the string is not empty.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::string::membership::MembershipAssertion;
    ///
    /// let email = "john@gmail.com";
    /// email.should_not_be_empty();
    /// ```
    fn should_not_be_empty(&self) -> &Self;
}

impl<T> MembershipAssertion for T
    where T: AsRef<str> {
    fn should_only_contain_digits(&self) -> &Self {
        self.should(&contain_only_digits());
        self
    }

    fn should_contain_a_digit(&self) -> &Self {
        self.should(&contain_a_digit());
        self
    }

    fn should_not_contain_digits(&self) -> &Self {
        self.should(&not_contain_digits());
        self
    }

    fn should_contain_character(&self, ch: char) -> &Self {
        self.should(&contain_character(ch));
        self
    }

    fn should_not_contain_character(&self, ch: char) -> &Self {
        self.should_not(&contain_character(ch));
        self
    }

    fn should_contain_all_characters(&self, chars: Vec<char>) -> &Self {
        self.should(&contain_all_characters(chars));
        self
    }

    fn should_not_contain_all_characters(&self, chars: Vec<char>) -> &Self {
        self.should_not(&contain_all_characters(chars));
        self
    }

    fn should_contain_any_characters(&self, chars: Vec<char>) -> &Self {
        self.should(&contain_any_of_characters(chars));
        self
    }

    fn should_not_contain_any_characters(&self, chars: Vec<char>) -> &Self {
        self.should_not(&contain_any_of_characters(chars));
        self
    }

    fn should_contain(&self, substr: &'static str) -> &Self {
        self.should(&contain(substr));
        self
    }

    fn should_not_contain(&self, substr: &'static str) -> &Self {
        self.should_not(&contain(substr));
        self
    }

    fn should_contain_ignoring_case(&self, substr: &'static str) -> &Self {
        self.should(&contain_ignoring_case(substr));
        self
    }

    fn should_not_contain_ignoring_case(&self, substr: &'static str) -> &Self {
        self.should_not(&contain_ignoring_case(substr));
        self
    }

    fn should_be_empty(&self) -> &Self {
        self.should(&be_empty());
        self
    }

    fn should_not_be_empty(&self) -> &Self {
        self.should_not(&be_empty());
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::assertions::string::membership::MembershipAssertion;

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
    fn should_contain_all_characters() {
        let email = "john@gmail.com";
        email.should_contain_all_characters(vec!['@', '.']);
    }

    #[test]
    #[should_panic]
    fn should_contain_all_characters_but_it_did_not() {
        let email = "john1@gmail.com";
        email.should_contain_all_characters(vec!['@', '.', '#']);
    }

    #[test]
    fn should_not_contain_all_characters() {
        let email = "john@gmail.com";
        email.should_not_contain_all_characters(vec!['@', '.', '#']);
    }

    #[test]
    #[should_panic]
    fn should_not_contain_all_characters_but_it_did() {
        let email = "john1@gmail.com";
        email.should_not_contain_all_characters(vec!['@', '.']);
    }

    #[test]
    fn should_contain_any_characters() {
        let email = "john@gmail.com";
        email.should_contain_any_characters(vec!['@', '#']);
    }

    #[test]
    #[should_panic]
    fn should_contain_any_characters_but_it_did_not() {
        let email = "john1@gmail.com";
        email.should_contain_any_characters(vec!['%', '^', '#']);
    }

    #[test]
    fn should_not_contain_any_characters() {
        let email = "john@gmail.com";
        email.should_not_contain_any_characters(vec!['#', '%', '&']);
    }

    #[test]
    #[should_panic]
    fn should_not_contain_any_characters_but_it_did() {
        let email = "john1@gmail.com";
        email.should_not_contain_any_characters(vec!['@', '#']);
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

#[cfg(test)]
mod string_tests {
    use crate::assertions::string::membership::MembershipAssertion;

    #[test]
    fn should_only_contain_digits() {
        let str = String::from("12345");
        str.should_only_contain_digits();
    }

    #[test]
    #[should_panic]
    fn should_only_contain_digits_but_it_did_not() {
        let str = String::from("12345a");
        str.should_only_contain_digits();
    }

    #[test]
    fn should_contain_a_digit() {
        let email = String::from("john1@gmail.com");
        email.should_contain_a_digit();
    }

    #[test]
    #[should_panic]
    fn should_contain_a_digit_but_it_did_not() {
        let email = String::from("john@gmail.com");
        email.should_contain_a_digit();
    }

    #[test]
    fn should_not_contain_digits() {
        let email = String::from("john@gmail.com");
        email.should_not_contain_digits();
    }

    #[test]
    #[should_panic]
    fn should_not_contain_digits_but_it_did() {
        let email = String::from("john1@gmail.com");
        email.should_not_contain_digits();
    }

    #[test]
    fn should_contain_the_character() {
        let email = String::from("john@gmail.com");
        email.should_contain_character('@');
    }

    #[test]
    #[should_panic]
    fn should_contain_the_character_but_it_did_not() {
        let email = String::from("john1@gmail.com");
        email.should_contain_character('#');
    }

    #[test]
    fn should_not_contain_the_character() {
        let email = String::from("john@gmail.com");
        email.should_not_contain_character('#');
    }

    #[test]
    #[should_panic]
    fn should_not_contain_the_character_but_it_did() {
        let email = String::from("john1@gmail.com");
        email.should_not_contain_character('@');
    }

    #[test]
    fn should_contain_all_characters() {
        let email = String::from("john@gmail.com");
        email.should_contain_all_characters(vec!['@', '.']);
    }

    #[test]
    #[should_panic]
    fn should_contain_all_characters_but_it_did_not() {
        let email = String::from("john1@gmail.com");
        email.should_contain_all_characters(vec!['@', '.', '#']);
    }

    #[test]
    fn should_not_contain_all_characters() {
        let email = String::from("john@gmail.com");
        email.should_not_contain_all_characters(vec!['@', '.', '#']);
    }

    #[test]
    #[should_panic]
    fn should_not_contain_all_characters_but_it_did() {
        let email = String::from("john1@gmail.com");
        email.should_not_contain_all_characters(vec!['@', '.']);
    }

    #[test]
    fn should_contain_any_characters() {
        let email = String::from("john@gmail.com");
        email.should_contain_any_characters(vec!['@', '#']);
    }

    #[test]
    #[should_panic]
    fn should_contain_any_characters_but_it_did_not() {
        let email = String::from("john1@gmail.com");
        email.should_contain_any_characters(vec!['%', '^', '#']);
    }

    #[test]
    fn should_not_contain_any_characters() {
        let email = String::from("john@gmail.com");
        email.should_not_contain_any_characters(vec!['#', '%', '&']);
    }

    #[test]
    #[should_panic]
    fn should_not_contain_any_characters_but_it_did() {
        let email = String::from("john1@gmail.com");
        email.should_not_contain_any_characters(vec!['@', '#']);
    }

    #[test]
    fn should_contain_substring() {
        let email = String::from("john@gmail.com");
        email.should_contain("gmail");
    }

    #[test]
    #[should_panic]
    fn should_contain_substring_but_it_did_not() {
        let email = String::from("john1@gmail.com");
        email.should_contain("yahoo");
    }

    #[test]
    fn should_not_contain_substring() {
        let email = String::from("john@gmail.com");
        email.should_not_contain("yahoo");
    }

    #[test]
    #[should_panic]
    fn should_not_contain_substring_but_it_did() {
        let email = String::from("john1@gmail.com");
        email.should_not_contain("gmail");
    }

    #[test]
    fn should_contain_substring_ignoring_case() {
        let email = String::from("john@gmail.com");
        email.should_contain_ignoring_case("GMAIL");
    }

    #[test]
    #[should_panic]
    fn should_contain_substring_ignoring_case_but_it_did_not() {
        let email = String::from("john1@gmail.com");
        email.should_contain_ignoring_case("YAHOO");
    }

    #[test]
    fn should_not_contain_substring_ignoring_case() {
        let email = String::from("john@gmail.com");
        email.should_not_contain_ignoring_case("YAHOO");
    }

    #[test]
    #[should_panic]
    fn should_not_contain_substring_ignoring_case_but_it_did() {
        let email = String::from("john1@gmail.com");
        email.should_not_contain_ignoring_case("GMAIL");
    }

    #[test]
    fn should_be_empty() {
        let name = String::from("");
        name.should_be_empty();
    }

    #[test]
    #[should_panic]
    fn should_be_empty_but_was_not() {
        let name = String::from("John");
        name.should_be_empty();
    }

    #[test]
    fn should_not_be_empty() {
        let name = String::from("John");
        name.should_not_be_empty();
    }

    #[test]
    #[should_panic]
    fn should_not_be_empty_but_was() {
        let name = String::from("");
        name.should_not_be_empty();
    }
}
