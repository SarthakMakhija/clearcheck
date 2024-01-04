use crate::matchers::Matcher;

pub enum BoundaryBased<'a> {
    Begin(&'a str),
    End(&'a str),
}

impl<'a> Matcher<&str> for BoundaryBased<'a> {
    fn test(&self, value: &&str) -> bool {
        match self {
            BoundaryBased::Begin(prefix) => value.starts_with(prefix),
            BoundaryBased::End(suffix) => value.ends_with(suffix),
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
        matcher.test(&"goselect").should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_begin_with_but_did_not() {
        let matcher = begin_with("go");
        matcher.test(&"select").should_be_true();
    }

    #[test]
    fn should_end_with() {
        let matcher = end_with("elect");
        matcher.test(&"goselect").should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_end_with_but_did_not() {
        let matcher = end_with("go");
        matcher.test(&"select").should_be_true();
    }
}
