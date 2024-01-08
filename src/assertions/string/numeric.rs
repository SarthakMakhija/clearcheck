use std::str::FromStr;

use crate::matchers::string::numeric::be_numeric;
use crate::matchers::{Should, ShouldNot};

pub trait NumericAssertion {
    fn should_be_numeric<T: FromStr>(&self) -> &Self;
    fn should_not_be_numeric<T: FromStr>(&self) -> &Self;
}

impl NumericAssertion for String {
    fn should_be_numeric<T: FromStr>(&self) -> &Self {
        (self as &str).should_be_numeric::<T>();
        self
    }

    fn should_not_be_numeric<T: FromStr>(&self) -> &Self {
        (self as &str).should_not_be_numeric::<T>();
        self
    }
}

impl NumericAssertion for &str {
    fn should_be_numeric<T: FromStr>(&self) -> &Self {
        self.should(&be_numeric::<T>());
        self
    }

    fn should_not_be_numeric<T: FromStr>(&self) -> &Self {
        self.should_not(&be_numeric::<T>());
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::assertions::string::numeric::NumericAssertion;

    #[test]
    fn should_be_numeric() {
        let value = "1234";
        value.should_be_numeric::<i32>();
    }

    #[test]
    #[should_panic]
    fn should_be_numeric_but_was_not() {
        let value = "1234a";
        value.should_be_numeric::<i32>();
    }

    #[test]
    fn should_not_be_numeric() {
        let value = "1234a";
        value.should_not_be_numeric::<i32>();
    }

    #[test]
    #[should_panic]
    fn should_not_be_numeric_but_was() {
        let value = "1234";
        value.should_not_be_numeric::<i32>();
    }
}
