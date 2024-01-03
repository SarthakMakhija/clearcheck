use crate::panicking::{assert_failed_unary, AssertKind};

trait Empty {
    fn should_be_empty(&self) -> &Self;
    fn should_not_be_empty(&self) -> &Self;
}

impl Empty for String {
    fn should_be_empty(&self) -> &Self {
        (self as &str).should_be_empty();
        self
    }

    fn should_not_be_empty(&self) -> &Self {
        (self as &str).should_not_be_empty();
        self
    }
}

impl Empty for &str {
    fn should_be_empty(&self) -> &Self {
        if !self.is_empty() {
            assert_failed_unary(AssertKind::Empty, self);
        }
        self
    }

    fn should_not_be_empty(&self) -> &Self {
        if self.is_empty() {
            assert_failed_unary(AssertKind::NotEmpty, self);
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::matchers::string::empty::Empty;

    #[test]
    fn should_be_empty() {
        let name = "";
        name.should_be_empty();
    }

    #[test]
    #[should_panic]
    fn should_be_empty_but_was_not() {
        let name = "John";
        name.should_be_empty();
    }

    #[test]
    fn should_not_be_empty() {
        let name = "John";
        name.should_not_be_empty();
    }

    #[test]
    #[should_panic]
    fn should_not_be_empty_but_was() {
        let name = "";
        name.should_not_be_empty();
    }
}