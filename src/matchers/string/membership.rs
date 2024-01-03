use crate::matchers::Matcher;

pub enum MembershipBased<'a> {
    OnlyDigits,
    ADigit,
    NoDigits,
    Char(char),
    Substr(&'a str),
    SubstrIgnoringCase(&'a str),
    Empty,
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
            MembershipBased::Empty => value.is_empty(),
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

pub fn be_empty() -> MembershipBased<'static> {
    MembershipBased::Empty
}
