use crate::matchers::{Matcher, MatcherResult};

pub enum BoundaryMatcher<'a> {
    Begin(&'a str),
    End(&'a str),
}

impl<'a, T> Matcher<T> for BoundaryMatcher<'a>
    where T: AsRef<str>
{
    fn test(&self, value: &T) -> MatcherResult {
        match self {
            BoundaryMatcher::Begin(prefix) => MatcherResult::formatted(
                value.as_ref().starts_with(prefix),
                format!("{:?} should begin with {:?}", value.as_ref(), prefix),
                format!("{:?} should not begin with {:?}", value.as_ref(), prefix),
            ),
            BoundaryMatcher::End(suffix) => MatcherResult::formatted(
                value.as_ref().ends_with(suffix),
                format!("{:?} should end with {:?}", value.as_ref(), suffix),
                format!("{:?} should not end with {:?}", value.as_ref(), suffix),
            ),
        }
    }
}

pub fn begin_with(prefix: &str) -> BoundaryMatcher<'_> {
    BoundaryMatcher::Begin(prefix)
}

pub fn end_with(suffix: &str) -> BoundaryMatcher<'_> {
    BoundaryMatcher::End(suffix)
}

#[cfg(test)]
mod tests {
    use crate::assertions::bool::TrueFalseAssertion;
    use crate::matchers::Matcher;
    use crate::matchers::string::boundary::{begin_with, end_with};

    #[test]
    fn should_begin_with() {
        let matcher = begin_with("go");
        matcher.test(&"goselect").passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_begin_with_but_did_not() {
        let matcher = begin_with("go");
        matcher.test(&"select").passed.should_be_true();
    }

    #[test]
    fn should_end_with() {
        let matcher = end_with("elect");
        matcher.test(&"goselect").passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_end_with_but_did_not() {
        let matcher = end_with("go");
        matcher.test(&"select").passed.should_be_true();
    }
}
