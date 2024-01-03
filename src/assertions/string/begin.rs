use crate::panicking::{assert_failed_binary, AssertKind};

pub trait Begin {
    fn should_begin_with(&self, prefix: &str) -> &Self;
    fn should_not_begin_with(&self, prefix: &str) -> &Self;
}

impl Begin for String {
    fn should_begin_with(&self, prefix: &str) -> &Self {
        (self as &str).should_begin_with(prefix);
        self
    }

    fn should_not_begin_with(&self, prefix: &str) -> &Self {
        (self as &str).should_not_begin_with(prefix);
        self
    }
}

impl Begin for &str {
    fn should_begin_with(&self, prefix: &str) -> &Self {
        if !self.starts_with(prefix) {
            assert_failed_binary(AssertKind::BeginWith, self, prefix);
        }
        self
    }

    fn should_not_begin_with(&self, prefix: &str) -> &Self {
        if self.starts_with(prefix) {
            assert_failed_binary(AssertKind::NotBeginWith, self, prefix);
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::assertions::string::begin::Begin;

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
}
