use crate::matchers::Matcher;

pub enum StringMembershipBased<'a> {
    OnlyDigits,
    ADigit,
    NoDigits,
    Char(char),
    Substr(&'a str),
    SubstrIgnoringCase(&'a str),
}

impl<'a> Matcher<&str> for StringMembershipBased<'a> {
    fn test(&self, value: &&str) -> bool {
        match self {
            StringMembershipBased::OnlyDigits => value.chars().all(|ch| ch.is_numeric()),
            StringMembershipBased::ADigit => value.chars().any(|ch| ch.is_numeric()),
            StringMembershipBased::NoDigits => !value.chars().any(|ch| ch.is_numeric()),
            StringMembershipBased::Char(ch) => value.chars().any(|source| &source == ch),
            StringMembershipBased::Substr(substr) => value.contains(substr),
            StringMembershipBased::SubstrIgnoringCase(substr) => {
                value.to_lowercase().contains(&substr.to_lowercase())
            }
        }
    }
}

pub fn contain_only_digits() -> StringMembershipBased<'static> {
    StringMembershipBased::OnlyDigits
}

pub fn contain_a_digit() -> StringMembershipBased<'static> {
    StringMembershipBased::ADigit
}

pub fn not_contain_digits() -> StringMembershipBased<'static> {
    StringMembershipBased::NoDigits
}

pub fn contain_character(ch: char) -> StringMembershipBased<'static> {
    StringMembershipBased::Char(ch)
}

pub fn contain(substr: &str) -> StringMembershipBased {
    StringMembershipBased::Substr(substr)
}

pub fn contain_ignoring_case(substr: &str) -> StringMembershipBased {
    StringMembershipBased::SubstrIgnoringCase(substr)
}
