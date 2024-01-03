use crate::matchers::Matcher;

pub enum SomeNoneBased {
    Some,
    None,
}

impl<T> Matcher<Option<T>> for SomeNoneBased {
    fn test(&self, value: &Option<T>) -> bool {
        match self {
            SomeNoneBased::Some => value.is_some(),
            SomeNoneBased::None => value.is_none(),
        }
    }
}

pub fn be_some() -> SomeNoneBased {
    SomeNoneBased::Some
}

pub fn be_none() -> SomeNoneBased {
    SomeNoneBased::None
}
