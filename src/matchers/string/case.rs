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
