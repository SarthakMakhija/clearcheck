use crate::matchers::equal::EqualityBased;
use crate::matchers::Matcher;

impl Matcher<char> for EqualityBased<'_, char> {
    fn test(&self, value: &char) -> bool {
        match self {
            EqualityBased::IgnoringCase(other) => value.eq_ignore_ascii_case(other),
        }
    }
}
