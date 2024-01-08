use crate::matchers::bool::{be_false, be_true};
use crate::matchers::Should;

pub trait TrueFalseAssertion {
    fn should_be_true(&self) -> &Self;
    fn should_be_false(&self) -> &Self;
}

impl TrueFalseAssertion for bool {
    fn should_be_true(&self) -> &Self {
        self.should(&be_true());
        self
    }

    fn should_be_false(&self) -> &Self {
        self.should(&be_false());
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::assertions::bool::TrueFalseAssertion;

    #[test]
    fn should_be_true() {
        let value = true;
        value.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_true_but_was_not() {
        let value = false;
        value.should_be_true();
    }

    #[test]
    fn should_be_false() {
        let value = false;
        value.should_be_false();
    }

    #[test]
    #[should_panic]
    fn should_be_false_but_was_not() {
        let value = true;
        value.should_be_false();
    }
}
