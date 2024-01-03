use crate::matchers::Matcher;

pub enum EqualityBased<'a, T: Eq> {
    IgnoringCase(&'a T),
}

impl Matcher<&str> for EqualityBased<'_, &str> {
    fn test(&self, value: &&str) -> bool {
        match self {
            EqualityBased::IgnoringCase(other) => value.eq_ignore_ascii_case(other),
        }
    }
}

impl Matcher<char> for EqualityBased<'_, char> {
    fn test(&self, value: &char) -> bool {
        match self {
            EqualityBased::IgnoringCase(other) => value.eq_ignore_ascii_case(other),
        }
    }
}

pub fn be_equal_ignoring_case<T: Eq>(other: &T) -> EqualityBased<'_, T> {
    EqualityBased::IgnoringCase(other)
}
