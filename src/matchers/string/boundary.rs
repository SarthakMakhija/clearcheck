use crate::matchers::{Matcher, MatcherResult};

pub enum BoundaryMatcher<T: AsRef<str>> {
    Begin(T),
    End(T),
}

impl<T> Matcher<T> for BoundaryMatcher<T>
    where T: AsRef<str>
{
    fn test(&self, value: &T) -> MatcherResult {
        match self {
            BoundaryMatcher::Begin(prefix) => MatcherResult::formatted(
                value.as_ref().starts_with(prefix.as_ref()),
                format!("{:?} should begin with {:?}", value.as_ref(), prefix.as_ref()),
                format!("{:?} should not begin with {:?}", value.as_ref(), prefix.as_ref()),
            ),
            BoundaryMatcher::End(suffix) => MatcherResult::formatted(
                value.as_ref().ends_with(suffix.as_ref()),
                format!("{:?} should end with {:?}", value.as_ref(), suffix.as_ref()),
                format!("{:?} should not end with {:?}", value.as_ref(), suffix.as_ref()),
            ),
        }
    }
}

pub fn begin_with<T: AsRef<str>>(prefix: T) -> BoundaryMatcher<T> {
    BoundaryMatcher::Begin(prefix)
}

pub fn end_with<T: AsRef<str>>(suffix: T) -> BoundaryMatcher<T> {
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
