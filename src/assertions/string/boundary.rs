use crate::matchers::string::boundary::{begin_with, end_with};
use crate::matchers::{Should, ShouldNot};

pub trait BoundaryAssertion {
    fn should_begin_with(&self, prefix: &str) -> &Self;
    fn should_not_begin_with(&self, prefix: &str) -> &Self;
    fn should_end_with(&self, suffix: &str) -> &Self;
    fn should_not_end_with(&self, suffix: &str) -> &Self;
}

impl BoundaryAssertion for String {
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

impl BoundaryAssertion for &str {
    fn should_begin_with(&self, prefix: &str) -> &Self {
        self.should(&begin_with(prefix));
        self
    }

    fn should_not_begin_with(&self, prefix: &str) -> &Self {
        self.should_not(&begin_with(prefix));
        self
    }

    fn should_end_with(&self, suffix: &str) -> &Self {
        self.should(&end_with(suffix));
        self
    }

    fn should_not_end_with(&self, suffix: &str) -> &Self {
        self.should_not(&end_with(suffix));
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::assertions::string::boundary::BoundaryAssertion;

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

#[cfg(test)]
mod string_tests {
    use crate::assertions::string::boundary::BoundaryAssertion;

    #[test]
    fn should_begin_with() {
        let library = String::from("cacheD");
        library.should_begin_with("cache");
    }

    #[test]
    #[should_panic]
    fn should_begin_with_but_it_did_not() {
        let library = String::from("junit");
        library.should_begin_with("unit");
    }

    #[test]
    fn should_not_begin_with() {
        let library = String::from("junit");
        library.should_not_begin_with("cache");
    }

    #[test]
    #[should_panic]
    fn should_not_begin_with_but_it_did() {
        let library = String::from("junit");
        library.should_not_begin_with("jun");
    }

    #[test]
    fn should_end_with() {
        let library = String::from("goselect");
        library.should_end_with("select");
    }

    #[test]
    #[should_panic]
    fn should_end_with_but_it_did_not() {
        let library = String::from("junit");
        library.should_end_with("et");
    }

    #[test]
    fn should_not_end_with() {
        let library = String::from("junit");
        library.should_not_end_with("cache");
    }

    #[test]
    #[should_panic]
    fn should_not_end_with_but_it_did() {
        let library = String::from("junit");
        library.should_not_end_with("unit");
    }
}
