use crate::matchers::Matcher;

pub enum OkErrBased {
    Ok,
    Err,
}

impl<T, E> Matcher<Result<T, E>> for OkErrBased {
    fn test(&self, value: &Result<T, E>) -> bool {
        match self {
            OkErrBased::Ok => value.is_ok(),
            OkErrBased::Err => value.is_err(),
        }
    }
}

pub fn be_ok() -> OkErrBased {
    OkErrBased::Ok
}

pub fn be_err() -> OkErrBased {
    OkErrBased::Err
}
