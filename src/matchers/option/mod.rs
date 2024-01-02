use std::fmt::Debug;

use crate::panicking::{assert_failed_unary, AssertKind};

trait SomeNone {
    fn should_be_some(&self) -> &Self;
    fn should_be_none(&self) -> &Self;
}

impl<T> SomeNone for Option<T>
where
    T: Debug,
{
    fn should_be_some(&self) -> &Self {
        if self.is_none() {
            assert_failed_unary(AssertKind::Some, self);
        }
        self
    }

    fn should_be_none(&self) -> &Self {
        if self.is_some() {
            assert_failed_unary(AssertKind::None, self);
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::matchers::option::SomeNone;

    #[test]
    fn should_be_none() {
        let option: Option<i32> = None;
        option.should_be_none();
    }

    #[test]
    #[should_panic]
    fn should_be_none_but_was_not() {
        let option = Some("junit");
        option.should_be_none();
    }

    #[test]
    fn should_be_some() {
        let option = Some("junit");
        option.should_be_some();
    }

    #[test]
    #[should_panic]
    fn should_be_some_but_was_not() {
        let option: Option<i32> = None;
        option.should_be_some();
    }
}
