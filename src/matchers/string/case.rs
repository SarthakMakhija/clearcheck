use crate::matchers::{Matcher, MatcherResult};

pub enum CaseBased {
    Lower,
    Upper,
}

impl Matcher<&str> for CaseBased {
    fn test(&self, value: &&str) -> MatcherResult {
        match self {
            CaseBased::Lower => MatcherResult::formatted(
                value == &value.to_lowercase(),
                format!("{:?} should be lowercase", value),
                format!("{:?} should not be lowercase", value),
            ),
            CaseBased::Upper => MatcherResult::formatted(
                value == &value.to_uppercase(),
                format!("{:?} should be uppercase", value),
                format!("{:?} should not be uppercase", value),
            ),
        }
    }
}

pub fn be_lowercase() -> CaseBased {
    CaseBased::Lower
}

pub fn be_uppercase() -> CaseBased {
    CaseBased::Upper
}

#[cfg(test)]
mod tests {
    use crate::assertions::bool::TrueFalseAssertions;
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
