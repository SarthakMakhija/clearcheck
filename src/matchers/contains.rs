use crate::matchers::Matcher;

pub enum StringContains<'a> {
    OnlyDigits,
    ADigit,
    NoDigits,
    Char(char),
    Substr(&'a str),
    SubstrIgnoringCase(&'a str),
}

impl<'a> Matcher<&str> for StringContains<'a> {
    fn test(&self, value: &&str) -> bool {
        match self {
            StringContains::OnlyDigits => value.chars().all(|ch| ch.is_numeric()),
            StringContains::ADigit => value.chars().any(|ch| ch.is_numeric()),
            StringContains::NoDigits => !value.chars().any(|ch| ch.is_numeric()),
            StringContains::Char(ch) => value.chars().any(|source| &source == ch),
            StringContains::Substr(substr) => value.contains(substr),
            StringContains::SubstrIgnoringCase(substr) => {
                value.to_lowercase().contains(&substr.to_lowercase())
            }
        }
    }
}

pub fn contain_only_digits() -> StringContains<'static> {
    StringContains::OnlyDigits
}

pub fn contain_a_digit() -> StringContains<'static> {
    StringContains::ADigit
}

pub fn not_contain_digits() -> StringContains<'static> {
    StringContains::NoDigits
}

pub fn contain_character(ch: char) -> StringContains<'static> {
    StringContains::Char(ch)
}

pub fn contain(substr: &str) -> StringContains {
    StringContains::Substr(substr)
}

pub fn contain_ignoring_case(substr: &str) -> StringContains {
    StringContains::SubstrIgnoringCase(substr)
}
