use crate::matchers::string::boundary::end_with;
use crate::matchers::{Should, ShouldNot};
use crate::panicking::{assert_failed_binary, AssertKind};

pub trait End {
    fn should_end_with(&self, suffix: &str) -> &Self;
    fn should_not_end_with(&self, suffix: &str) -> &Self;
}

impl End for String {
    fn should_end_with(&self, suffix: &str) -> &Self {
        (self as &str).should_end_with(suffix);
        self
    }

    fn should_not_end_with(&self, suffix: &str) -> &Self {
        (self as &str).should_not_end_with(suffix);
        self
    }
}

impl End for &str {
    fn should_end_with(&self, suffix: &str) -> &Self {
        if !self.should(&end_with(suffix)) {
            assert_failed_binary(AssertKind::EndWith, self, suffix);
        }
        self
    }

    fn should_not_end_with(&self, suffix: &str) -> &Self {
        if !self.should_not(&end_with(suffix)) {
            assert_failed_binary(AssertKind::NotEndWith, self, suffix);
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::assertions::string::end::End;

    #[test]
    fn should_end_with() {
        let library = "goselect";
        library.should_end_with("select");
    }

    #[test]
    #[should_panic]
    fn should_end_with_but_it_did_not() {
        let library = "junit";
        library.should_end_with("et");
    }

    #[test]
    fn should_not_end_with() {
        let library = "junit";
        library.should_not_end_with("cache");
    }

    #[test]
    #[should_panic]
    fn should_not_end_with_but_it_did() {
        let library = "junit";
        library.should_not_end_with("unit");
    }
}
