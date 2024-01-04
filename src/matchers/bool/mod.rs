use crate::matchers::{Matcher, MatcherResult};

pub enum TrueFalseBased {
    True,
    False,
}

impl Matcher<bool> for TrueFalseBased {
    fn test(&self, value: &bool) -> MatcherResult {
        match self {
            TrueFalseBased::True => MatcherResult::new(
                *value == true,
                "Value should be TRUE",
                "Value should not be TRUE",
            ),
            TrueFalseBased::False => MatcherResult::new(
                *value == false,
                "Value should be FALSE",
                "Value should not be FALSE",
            ),
        }
    }
}

pub fn be_true() -> TrueFalseBased {
    TrueFalseBased::True
}

pub fn be_false() -> TrueFalseBased {
    TrueFalseBased::False
}

#[cfg(test)]
mod tests {
    use crate::assertions::bool::TrueFalse;
    use crate::matchers::bool::{be_false, be_true};
    use crate::matchers::Matcher;

    #[test]
    fn should_be_true() {
        let matcher = be_true();
        matcher.test(&true).passed.should_be_true();
    }

    #[test]
    fn should_be_false() {
        let matcher = be_false();
        matcher.test(&false).passed.should_be_true();
    }
}
