use std::fmt::Debug;

use num::Integer;

use crate::matchers::int::{be_even, be_negative, be_odd, be_positive, be_zero};
use crate::matchers::{Should, ShouldNot};

pub trait IntAssertion<T: Integer + Debug + PartialEq + Default> {
    fn should_be_positive(&self) -> &Self;
    fn should_be_negative(&self) -> &Self;
    fn should_be_even(&self) -> &Self;
    fn should_be_odd(&self) -> &Self;
    fn should_be_zero(&self) -> &Self;
    fn should_not_be_zero(&self) -> &Self;
}

impl<T: Integer + Debug + PartialEq + Default> IntAssertion<T> for T {
    fn should_be_positive(&self) -> &Self {
        self.should(&be_positive());
        self
    }

    fn should_be_negative(&self) -> &Self {
        self.should(&be_negative());
        self
    }

    fn should_be_even(&self) -> &Self {
        self.should(&be_even());
        self
    }

    fn should_be_odd(&self) -> &Self {
        self.should(&be_odd());
        self
    }

    fn should_be_zero(&self) -> &Self {
        self.should(&be_zero());
        self
    }

    fn should_not_be_zero(&self) -> &Self {
        self.should_not(&be_zero());
        self
    }
}

#[cfg(all(test, feature = "num"))]
mod tests {
    use crate::assertions::int::IntAssertion;

    #[test]
    fn should_be_positive() {
        let value = 10;
        value.should_be_positive();
    }

    #[test]
    #[should_panic]
    fn should_be_positive_but_was_not() {
        let value = -10;
        value.should_be_positive();
    }

    #[test]
    fn should_be_negative() {
        let value = -10;
        value.should_be_negative();
    }

    #[test]
    #[should_panic]
    fn should_be_negative_but_was_not() {
        let value = 10;
        value.should_be_negative();
    }

    #[test]
    fn should_be_even() {
        let value = 10;
        value.should_be_even();
    }

    #[test]
    #[should_panic]
    fn should_be_even_but_was_not() {
        let value = -11;
        value.should_be_even();
    }

    #[test]
    fn should_be_odd() {
        let value = 11;
        value.should_be_odd();
    }

    #[test]
    #[should_panic]
    fn should_be_odd_but_was_not() {
        let value = -10;
        value.should_be_odd();
    }

    #[test]
    fn should_be_zero() {
        let value = 0;
        value.should_be_zero();
    }

    #[test]
    #[should_panic]
    fn should_be_zero_but_was_not() {
        let value = -10;
        value.should_be_zero();
    }

    #[test]
    fn should_not_be_zero() {
        let value = 1;
        value.should_not_be_zero();
    }

    #[test]
    #[should_panic]
    fn should_not_be_zero_but_was() {
        let value = 0;
        value.should_not_be_zero();
    }
}
