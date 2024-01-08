use crate::matchers::{Matcher, MatcherResult};

pub enum MembershipMatcher<'a> {
    OnlyDigits,
    ADigit,
    NoDigits,
    Char(char),
    Substr(&'a str),
    SubstrIgnoringCase(&'a str),
}

impl<'a> Matcher<&str> for MembershipMatcher<'a> {
    fn test(&self, value: &&str) -> MatcherResult {
        match self {
            MembershipMatcher::OnlyDigits => MatcherResult::formatted(
                value.chars().all(|ch| ch.is_numeric()),
                format!("{:?} should contain only digits", value),
                format!("{:?} should not contain only digits", value),
            ),
            MembershipMatcher::ADigit => MatcherResult::formatted(
                value.chars().any(|ch| ch.is_numeric()),
                format!("{:?} should contain a digit", value),
                format!("{:?} should not contain a digit", value),
            ),
            MembershipMatcher::NoDigits => MatcherResult::formatted(
                !value.chars().any(|ch| ch.is_numeric()),
                format!("{:?} should contain no digits", value),
                format!("{:?} should contain digits", value),
            ),
            MembershipMatcher::Char(ch) => MatcherResult::formatted(
                value.chars().any(|source| &source == ch),
                format!("{:?} should contain the character {:?}", value, ch),
                format!("{:?} should not contain the character {:?}", value, ch),
            ),
            MembershipMatcher::Substr(substr) => MatcherResult::formatted(
                value.contains(substr),
                format!("{:?} should contain the substring {:?}", value, substr),
                format!("{:?} should not contain the substring {:?}", value, substr),
            ),
            MembershipMatcher::SubstrIgnoringCase(substr) => MatcherResult::formatted(
                value.to_lowercase().contains(&substr.to_lowercase()),
                format!(
                    "{:?} should contain the substring ignoring case {:?}",
                    value, substr
                ),
                format!(
                    "{:?} should not contain the substring ignoring case {:?}",
                    value, substr
                ),
            ),
        }
    }
}

pub fn contain_only_digits() -> MembershipMatcher<'static> {
    MembershipMatcher::OnlyDigits
}

pub fn contain_a_digit() -> MembershipMatcher<'static> {
    MembershipMatcher::ADigit
}

pub fn not_contain_digits() -> MembershipMatcher<'static> {
    MembershipMatcher::NoDigits
}

pub fn contain_character(ch: char) -> MembershipMatcher<'static> {
    MembershipMatcher::Char(ch)
}

pub fn contain(substr: &str) -> MembershipMatcher {
    MembershipMatcher::Substr(substr)
}

pub fn contain_ignoring_case(substr: &str) -> MembershipMatcher {
    MembershipMatcher::SubstrIgnoringCase(substr)
}

#[cfg(test)]
mod tests {
    use crate::assertions::bool::TrueFalseAssertion;
    use crate::matchers::string::membership::{
        contain, contain_character, contain_ignoring_case, contain_only_digits, not_contain_digits,
    };
    use crate::matchers::Matcher;

    #[test]
    fn should_contains_only_digits() {
        let matcher = contain_only_digits();
        matcher.test(&"12345").passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_contains_only_digits_but_did_not() {
        let matcher = contain_only_digits();
        matcher.test(&"12345a").passed.should_be_true();
    }

    #[test]
    fn should_contain_a_digit() {
        let matcher = contain_only_digits();
        matcher.test(&"12345").passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_contain_a_digit_but_did_not() {
        let matcher = contain_only_digits();
        matcher.test(&"goselect").passed.should_be_true();
    }

    #[test]
    fn should_not_contain_any_digit() {
        let matcher = not_contain_digits();
        matcher.test(&"goselect").passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_not_contain_any_digit_but_it_did() {
        let matcher = not_contain_digits();
        matcher.test(&"goselect1").passed.should_be_true();
    }

    #[test]
    fn should_contain_a_char() {
        let matcher = contain_character('g');
        matcher.test(&"goselect").passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_contain_a_char_but_it_did() {
        let matcher = contain_character('$');
        matcher.test(&"goselect").passed.should_be_true();
    }

    #[test]
    fn should_contain_substring() {
        let matcher = contain("select");
        matcher.test(&"goselect").passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_contain_substring_but_it_did_not() {
        let matcher = contain("etcd");
        matcher.test(&"goselect").passed.should_be_true();
    }

    #[test]
    fn should_contain_substring_ignoring_case() {
        let matcher = contain_ignoring_case("SELECT");
        matcher.test(&"goselect").passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_contain_substring_ignoring_case_but_it_did_not() {
        let matcher = contain_ignoring_case("ETCD");
        matcher.test(&"goselect").passed.should_be_true();
    }
}
