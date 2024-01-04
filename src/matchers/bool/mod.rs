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
