use crate::matchers::Matcher;

pub enum CaseBased {
    Lower,
    Upper,
}

impl Matcher<&str> for CaseBased {
    fn test(&self, value: &&str) -> bool {
        match self {
            CaseBased::Lower => value == &value.to_lowercase(),
            CaseBased::Upper => value == &value.to_uppercase(),
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
    use crate::assertions::bool::TrueFalse;
    use crate::matchers::string::case::{be_lowercase, be_uppercase};
    use crate::matchers::Matcher;

    #[test]
    fn should_be_lowercase() {
        let matcher = be_lowercase();
        matcher.test(&"goselect").should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_lowercase_but_was_not() {
        let matcher = be_lowercase();
        matcher.test(&"GoSelect").should_be_true();
    }

    #[test]
    fn should_be_uppercase() {
        let matcher = be_uppercase();
        matcher.test(&"GOSELECT").should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_uppercase_but_was_not() {
        let matcher = be_uppercase();
        matcher.test(&"GoSelect").should_be_true();
    }
}
