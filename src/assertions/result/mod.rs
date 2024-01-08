use std::fmt::Debug;

use crate::matchers::result::{be_err, be_ok};
use crate::matchers::Should;

pub trait OkErrAssertion {
    fn should_be_ok(&self) -> &Self;
    fn should_be_err(&self) -> &Self;
}

impl<T, E> OkErrAssertion for Result<T, E>
where
    T: Debug,
    E: Debug,
{
    fn should_be_ok(&self) -> &Self {
        self.should(&be_ok());
        self
    }

    fn should_be_err(&self) -> &Self {
        self.should(&be_err());
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::assertions::result::OkErrAssertion;

    #[test]
    fn should_be_ok() {
        let result: Result<i32, &str> = Ok(100);
        result.should_be_ok();
    }

    #[test]
    #[should_panic]
    fn should_be_ok_but_was_not() {
        let result: Result<i32, &str> = Err("test error");
        result.should_be_ok();
    }

    #[test]
    fn should_be_err() {
        let result: Result<i32, &str> = Err("test error");
        result.should_be_err();
    }

    #[test]
    #[should_panic]
    fn should_be_err_but_was_not() {
        let result: Result<i32, &str> = Ok(100);
        result.should_be_err();
    }
}
