use crate::matchers::Matcher;

pub enum EqualMatcher<'a, T: Eq> {
    IgnoringCase(&'a T),
}

impl Matcher<&str> for EqualMatcher<'_, &str> {
    fn test(&self, value: &&str) -> bool {
        match self {
            EqualMatcher::IgnoringCase(other) => value.eq_ignore_ascii_case(other),
        }
    }
}

impl Matcher<char> for EqualMatcher<'_, char> {
    fn test(&self, value: &char) -> bool {
        match self {
            EqualMatcher::IgnoringCase(other) => value.eq_ignore_ascii_case(other),
        }
    }
}

pub fn be_equal_ignoring_case<T: Eq>(other: &T) -> EqualMatcher<'_, T> {
    EqualMatcher::IgnoringCase(other)
}
