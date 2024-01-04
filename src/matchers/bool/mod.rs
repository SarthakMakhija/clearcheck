use crate::matchers::Matcher;

pub enum TrueFalseBased {
    True,
    False,
}

impl Matcher<bool> for TrueFalseBased {
    fn test(&self, value: &bool) -> bool {
        match self {
            TrueFalseBased::True => *value == true,
            TrueFalseBased::False => *value == false,
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
        matcher.test(&true).should_be_true();
    }

    #[test]
    fn should_be_false() {
        let matcher = be_false();
        matcher.test(&false).should_be_true();
    }
}
