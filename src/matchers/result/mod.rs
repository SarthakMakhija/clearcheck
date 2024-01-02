use std::fmt::Debug;

use crate::panicking::{assert_failed_unary, AssertKind};

trait OkErr {
    fn should_be_ok(&self) -> &Self;
    fn should_be_err(&self) -> &Self;
}

impl<T, E> OkErr for Result<T, E>
where
    T: Debug,
    E: Debug,
{
    fn should_be_ok(&self) -> &Self {
        if self.is_err() {
            assert_failed_unary(AssertKind::Ok, self);
        }
        self
    }

    fn should_be_err(&self) -> &Self {
        if self.is_ok() {
            assert_failed_unary(AssertKind::Err, self);
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::matchers::result::OkErr;

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
