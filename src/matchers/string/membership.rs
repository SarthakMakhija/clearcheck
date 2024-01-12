use crate::matchers::{Matcher, MatcherResult};

pub enum MembershipMatcher {
    OnlyDigits,
    ADigit,
    NoDigits,
    Char(char),
    AllChars(Vec<char>),
    AnyChars(Vec<char>),
}

pub enum SubstringMatcher<T: AsRef<str>> {
    Substr(T),
    SubstrIgnoringCase(T),
}

impl<T> Matcher<T> for MembershipMatcher
    where T: AsRef<str>
{
    fn test(&self, value: &T) -> MatcherResult {
        match self {
            MembershipMatcher::OnlyDigits => MatcherResult::formatted(
                value.as_ref().chars().all(|ch| ch.is_numeric()),
                format!("{:?} should contain only digits", value.as_ref()),
                format!("{:?} should not contain only digits", value.as_ref()),
            ),
            MembershipMatcher::ADigit => MatcherResult::formatted(
                value.as_ref().chars().any(|ch| ch.is_numeric()),
                format!("{:?} should contain a digit", value.as_ref()),
                format!("{:?} should not contain a digit", value.as_ref()),
            ),
            MembershipMatcher::NoDigits => MatcherResult::formatted(
                !value.as_ref().chars().any(|ch| ch.is_numeric()),
                format!("{:?} should contain no digits", value.as_ref()),
                format!("{:?} should contain digits", value.as_ref()),
            ),
            MembershipMatcher::Char(ch) => MatcherResult::formatted(
                value.as_ref().chars().any(|source| &source == ch),
                format!("{:?} should contain the character {:?}", value.as_ref(), ch),
                format!("{:?} should not contain the character {:?}", value.as_ref(), ch),
            ),
            MembershipMatcher::AllChars(chars) => MatcherResult::formatted(
                chars.iter().all(|ch| value.as_ref().contains(*ch)),
                format!("{:?} should contain all characters {:?}", value.as_ref(), chars),
                format!("{:?} should not contain all characters {:?}", value.as_ref(), chars),
            ),
            MembershipMatcher::AnyChars(chars) => MatcherResult::formatted(
                chars.iter().any(|ch| value.as_ref().contains(*ch)),
                format!("{:?} should contain any of the characters {:?}", value.as_ref(), chars),
                format!("{:?} should not contain any of the characters {:?}", value.as_ref(), chars),
            ),
        }
    }
}

impl<T> Matcher<T> for SubstringMatcher<T>
    where T: AsRef<str>
{
    fn test(&self, value: &T) -> MatcherResult {
        match self {
            SubstringMatcher::Substr(substr) => MatcherResult::formatted(
                value.as_ref().contains(substr.as_ref()),
                format!("{:?} should contain the substring {:?}", value.as_ref(), substr.as_ref()),
                format!("{:?} should not contain the substring {:?}", value.as_ref(), substr.as_ref()),
            ),
            SubstringMatcher::SubstrIgnoringCase(substr) => MatcherResult::formatted(
                value.as_ref().to_lowercase().contains(&substr.as_ref().to_lowercase()),
                format!(
                    "{:?} should contain the substring ignoring case {:?}",
                    value.as_ref(), substr.as_ref()
                ),
                format!(
                    "{:?} should not contain the substring ignoring case {:?}",
                    value.as_ref(), substr.as_ref()
                ),
            )
        }
    }
}


pub fn contain_only_digits() -> MembershipMatcher {
    MembershipMatcher::OnlyDigits
}

pub fn contain_a_digit() -> MembershipMatcher {
    MembershipMatcher::ADigit
}

pub fn not_contain_digits() -> MembershipMatcher {
    MembershipMatcher::NoDigits
}

pub fn contain_character(ch: char) -> MembershipMatcher {
    MembershipMatcher::Char(ch)
}

pub fn contain_all_characters(chars: Vec<char>) -> MembershipMatcher {
    MembershipMatcher::AllChars(chars)
}

pub fn contain_any_of_characters(chars: Vec<char>) -> MembershipMatcher {
    MembershipMatcher::AnyChars(chars)
}

pub fn contain<T: AsRef<str>>(substr: T) -> SubstringMatcher<T> {
    SubstringMatcher::Substr(substr)
}

pub fn contain_ignoring_case<T: AsRef<str>>(substr: T) -> SubstringMatcher<T> {
    SubstringMatcher::SubstrIgnoringCase(substr)
}

#[cfg(test)]
mod tests {
    use crate::assertions::bool::TrueFalseAssertion;
    use crate::matchers::Matcher;
    use crate::matchers::string::membership::{contain, contain_all_characters, contain_any_of_characters, contain_character, contain_ignoring_case, contain_only_digits, not_contain_digits};

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
    fn should_contain_all_chars() {
        let matcher = contain_all_characters(vec!['@', '#']);
        matcher.test(&"p@@sword#123").passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_contain_all_chars_but_it_did_not() {
        let matcher = contain_all_characters(vec!['@', '#']);
        matcher.test(&"p@@sword").passed.should_be_true();
    }

    #[test]
    fn should_contain_any_of_chars() {
        let matcher = contain_any_of_characters(vec!['@', '%']);
        matcher.test(&"p@@sword#123").passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_contain_any_of_chars_but_it_did_not() {
        let matcher = contain_any_of_characters(vec!['^', '%']);
        matcher.test(&"p@@sword").passed.should_be_true();
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
