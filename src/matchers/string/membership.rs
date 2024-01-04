use crate::matchers::Matcher;

pub enum MembershipBased<'a> {
    OnlyDigits,
    ADigit,
    NoDigits,
    Char(char),
    Substr(&'a str),
    SubstrIgnoringCase(&'a str),
}

impl<'a> Matcher<&str> for MembershipBased<'a> {
    fn test(&self, value: &&str) -> bool {
        match self {
            MembershipBased::OnlyDigits => value.chars().all(|ch| ch.is_numeric()),
            MembershipBased::ADigit => value.chars().any(|ch| ch.is_numeric()),
            MembershipBased::NoDigits => !value.chars().any(|ch| ch.is_numeric()),
            MembershipBased::Char(ch) => value.chars().any(|source| &source == ch),
            MembershipBased::Substr(substr) => value.contains(substr),
            MembershipBased::SubstrIgnoringCase(substr) => {
                value.to_lowercase().contains(&substr.to_lowercase())
            }
        }
    }
}

pub fn contain_only_digits() -> MembershipBased<'static> {
    MembershipBased::OnlyDigits
}

pub fn contain_a_digit() -> MembershipBased<'static> {
    MembershipBased::ADigit
}

pub fn not_contain_digits() -> MembershipBased<'static> {
    MembershipBased::NoDigits
}

pub fn contain_character(ch: char) -> MembershipBased<'static> {
    MembershipBased::Char(ch)
}

pub fn contain(substr: &str) -> MembershipBased {
    MembershipBased::Substr(substr)
}

pub fn contain_ignoring_case(substr: &str) -> MembershipBased {
    MembershipBased::SubstrIgnoringCase(substr)
}

#[cfg(test)]
mod tests {
    use crate::assertions::bool::TrueFalse;
    use crate::matchers::string::membership::{
        contain, contain_character, contain_ignoring_case, contain_only_digits, not_contain_digits,
    };
    use crate::matchers::Matcher;

    #[test]
    fn should_contains_only_digits() {
        let matcher = contain_only_digits();
        matcher.test(&"12345").should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_contains_only_digits_but_did_not() {
        let matcher = contain_only_digits();
        matcher.test(&"12345a").should_be_true();
    }

    #[test]
    fn should_contain_a_digit() {
        let matcher = contain_only_digits();
        matcher.test(&"12345").should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_contain_a_digit_but_did_not() {
        let matcher = contain_only_digits();
        matcher.test(&"goselect").should_be_true();
    }

    #[test]
    fn should_not_contain_any_digit() {
        let matcher = not_contain_digits();
        matcher.test(&"goselect").should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_not_contain_any_digit_but_it_did() {
        let matcher = not_contain_digits();
        matcher.test(&"goselect1").should_be_true();
    }

    #[test]
    fn should_contain_a_char() {
        let matcher = contain_character('g');
        matcher.test(&"goselect").should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_contain_a_char_but_it_did() {
        let matcher = contain_character('$');
        matcher.test(&"goselect").should_be_true();
    }

    #[test]
    fn should_contain_substring() {
        let matcher = contain("select");
        matcher.test(&"goselect").should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_contain_substring_but_it_did_not() {
        let matcher = contain("etcd");
        matcher.test(&"goselect").should_be_true();
    }

    #[test]
    fn should_contain_substring_ignoring_case() {
        let matcher = contain_ignoring_case("SELECT");
        matcher.test(&"goselect").should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_contain_substring_ignoring_case_but_it_did_not() {
        let matcher = contain_ignoring_case("ETCD");
        matcher.test(&"goselect").should_be_true();
    }
}
