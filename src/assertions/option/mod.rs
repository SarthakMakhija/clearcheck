use std::fmt::Debug;

use crate::matchers::option::{be_none, be_some};
use crate::matchers::Should;

pub trait SomeNoneAssertions {
    fn should_be_some(&self) -> &Self;
    fn should_be_none(&self) -> &Self;
}

impl<T> SomeNoneAssertions for Option<T>
where
    T: Debug,
{
    fn should_be_some(&self) -> &Self {
        self.should(&be_some());
        self
    }

    fn should_be_none(&self) -> &Self {
        self.should(&be_none());
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::assertions::option::SomeNoneAssertions;

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
