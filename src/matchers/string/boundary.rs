use crate::matchers::{Matcher, MatcherResult};

pub enum BoundaryBased<'a> {
    Begin(&'a str),
    End(&'a str),
}

impl<'a> Matcher<&str> for BoundaryBased<'a> {
    fn test(&self, value: &&str) -> MatcherResult {
        match self {
            BoundaryBased::Begin(prefix) => MatcherResult::formatted(
                value.starts_with(prefix),
                format!("{:?} should begin with {:?}", value, prefix),
                format!("{:?} should not begin with {:?}", value, prefix),
            ),
            BoundaryBased::End(suffix) => MatcherResult::formatted(
                value.ends_with(suffix),
                format!("{:?} should end with {:?}", value, suffix),
                format!("{:?} should not end with {:?}", value, suffix),
            ),
        }
    }
}

pub fn begin_with(prefix: &str) -> BoundaryBased<'_> {
    BoundaryBased::Begin(prefix)
}

pub fn end_with(suffix: &str) -> BoundaryBased<'_> {
    BoundaryBased::End(suffix)
}

#[cfg(test)]
mod tests {
    use crate::assertions::bool::TrueFalse;
    use crate::matchers::string::boundary::{begin_with, end_with};
    use crate::matchers::Matcher;

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
