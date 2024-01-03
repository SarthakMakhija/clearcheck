use crate::matchers::equal::EqualityBased;
use crate::matchers::Matcher;

impl Matcher<&str> for EqualityBased<'_, &str> {
    fn test(&self, value: &&str) -> bool {
        match self {
            EqualityBased::IgnoringCase(other) => value.eq_ignore_ascii_case(other),
        }
    }
}
