use crate::panicking::{assert_failed_unary, AssertKind};

trait TrueFalse {
    fn should_be_true(&self) -> &Self;
    fn should_be_false(&self) -> &Self;
}

impl TrueFalse for bool {
    fn should_be_true(&self) -> &Self {
        if !*self {
            assert_failed_unary(AssertKind::True, self)
        }
        self
    }

    fn should_be_false(&self) -> &Self {
        if *self {
            assert_failed_unary(AssertKind::False, self)
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::matchers::bool::TrueFalse;

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
