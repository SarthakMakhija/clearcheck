use crate::matchers::equal::EqualityBased;
use crate::matchers::Matcher;

impl Matcher<char> for EqualityBased<'_, char> {
    fn test(&self, value: &char) -> bool {
        match self {
            EqualityBased::IgnoringCase(other) => value.eq_ignore_ascii_case(other),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::assertions::bool::TrueFalse;
    use crate::matchers::equal::be_equal_ignoring_case;
    use crate::matchers::Matcher;

    #[test]
    fn should_be_equal() {
        let based = be_equal_ignoring_case(&'a');
        based.test(&'a').should_be_true();
    }

    #[test]
    fn should_not_be_equal() {
        let based = be_equal_ignoring_case(&'b');
        based.test(&'a').should_be_false();
    }
}
