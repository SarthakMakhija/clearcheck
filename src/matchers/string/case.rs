use crate::matchers::{Matcher, MatcherResult};

pub enum CaseMatcher {
    Lower,
    Upper,
}

impl Matcher<&str> for CaseMatcher {
    fn test(&self, value: &&str) -> MatcherResult {
        match self {
            CaseMatcher::Lower => MatcherResult::formatted(
                value == &value.to_lowercase(),
                format!("{:?} should be lowercase", value),
                format!("{:?} should not be lowercase", value),
            ),
            CaseMatcher::Upper => MatcherResult::formatted(
                value == &value.to_uppercase(),
                format!("{:?} should be uppercase", value),
                format!("{:?} should not be uppercase", value),
            ),
        }
    }
}

pub fn be_lowercase() -> CaseMatcher {
    CaseMatcher::Lower
}

pub fn be_uppercase() -> CaseMatcher {
    CaseMatcher::Upper
}

#[cfg(test)]
mod tests {
    use crate::assertions::bool::TrueFalseAssertion;
    use crate::matchers::string::case::{be_lowercase, be_uppercase};
    use crate::matchers::Matcher;

    #[test]
    fn should_be_lowercase() {
        let matcher = be_lowercase();
        matcher.test(&"goselect").passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_lowercase_but_was_not() {
        let matcher = be_lowercase();
        matcher.test(&"GoSelect").passed.should_be_true();
    }

    #[test]
    fn should_be_uppercase() {
        let matcher = be_uppercase();
        matcher.test(&"GOSELECT").passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_uppercase_but_was_not() {
        let matcher = be_uppercase();
        matcher.test(&"GoSelect").passed.should_be_true();
    }
}
