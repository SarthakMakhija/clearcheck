use crate::matchers::string::boundary::{begin_with, end_with};
use crate::matchers::{Should, ShouldNot};
use crate::panicking::{assert_failed_binary, AssertKind};

pub trait Boundary {
    fn should_begin_with(&self, prefix: &str) -> &Self;
    fn should_not_begin_with(&self, prefix: &str) -> &Self;
    fn should_end_with(&self, suffix: &str) -> &Self;
    fn should_not_end_with(&self, suffix: &str) -> &Self;
}

impl Boundary for String {
    fn should_begin_with(&self, prefix: &str) -> &Self {
        (self as &str).should_begin_with(prefix);
        self
    }

    fn should_not_begin_with(&self, prefix: &str) -> &Self {
        (self as &str).should_not_begin_with(prefix);
        self
    }

    fn should_end_with(&self, suffix: &str) -> &Self {
        (self as &str).should_end_with(suffix);
        self
    }

    fn should_not_end_with(&self, suffix: &str) -> &Self {
        (self as &str).should_not_end_with(suffix);
        self
    }
}

impl Boundary for &str {
    fn should_begin_with(&self, prefix: &str) -> &Self {
        if !self.should(&begin_with(prefix)) {
            assert_failed_binary(AssertKind::BeginWith, self, prefix);
        }
        self
    }

    fn should_not_begin_with(&self, prefix: &str) -> &Self {
        if !self.should_not(&begin_with(prefix)) {
            assert_failed_binary(AssertKind::NotBeginWith, self, prefix);
        }
        self
    }

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
    use crate::assertions::string::boundary::Boundary;

    #[test]
    fn should_begin_with() {
        let library = "cacheD";
        library.should_begin_with("cache");
    }

    #[test]
    #[should_panic]
    fn should_begin_with_but_it_did_not() {
        let library = "junit";
        library.should_begin_with("unit");
    }

    #[test]
    fn should_not_begin_with() {
        let library = "junit";
        library.should_not_begin_with("cache");
    }

    #[test]
    #[should_panic]
    fn should_not_begin_with_but_it_did() {
        let library = "junit";
        library.should_not_begin_with("jun");
    }

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
