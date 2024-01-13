use crate::matchers::string::boundary::{begin_with, end_with};
use crate::matchers::{Should, ShouldNot};

/// BoundaryAssertion enables assertions about the beginning and the ending boundaries of string (or str) values.
///
/// It offers a fluent interface for chaining multiple assertions.
///
/// # Example
/// ```
/// use clearcheck::assertions::string::boundary::BoundaryAssertion;
///
/// let value = "clearcheck";
/// value
///     .should_begin_with("clear")
///     .should_end_with("check");
/// ```
pub trait BoundaryAssertion {
    /// - Asserts that the string begins with the given prefix.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::string::boundary::BoundaryAssertion;
    ///
    /// let value = "clearcheck";
    /// value
    ///     .should_begin_with("clear");
    /// ```
    fn should_begin_with(&self, prefix: &'static str) -> &Self;

    /// - Asserts that the string does not begin with the given prefix
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::string::boundary::BoundaryAssertion;
    ///
    /// let value = "clearcheck";
    /// value
    ///     .should_not_begin_with("rust");
    /// ```
    fn should_not_begin_with(&self, prefix: &'static str) -> &Self;

    /// - Asserts that the string ends with the given suffix.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::string::boundary::BoundaryAssertion;
    ///
    /// let value = "clearcheck";
    /// value
    ///     .should_end_with("check");
    /// ```
    fn should_end_with(&self, suffix: &'static str) -> &Self;

    /// - Asserts that the string does not end with the given suffix.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::string::boundary::BoundaryAssertion;
    ///
    /// let value = "clearcheck";
    /// value
    ///     .should_not_end_with("test");
    /// ```
    fn should_not_end_with(&self, suffix: &'static str) -> &Self;
}

impl BoundaryAssertion for String {
    fn should_begin_with(&self, prefix: &'static str) -> &Self {
        (self as &str).should_begin_with(prefix);
        self
    }

    fn should_not_begin_with(&self, prefix: &'static str) -> &Self {
        (self as &str).should_not_begin_with(prefix);
        self
    }

    fn should_end_with(&self, suffix: &'static str) -> &Self {
        (self as &str).should_end_with(suffix);
        self
    }

    fn should_not_end_with(&self, suffix: &'static str) -> &Self {
        (self as &str).should_not_end_with(suffix);
        self
    }
}

impl BoundaryAssertion for &str {
    fn should_begin_with(&self, prefix: &'static str) -> &Self {
        self.should(&begin_with(prefix));
        self
    }

    fn should_not_begin_with(&self, prefix: &'static str) -> &Self {
        self.should_not(&begin_with(prefix));
        self
    }

    fn should_end_with(&self, suffix: &'static str) -> &Self {
        self.should(&end_with(suffix));
        self
    }

    fn should_not_end_with(&self, suffix: &'static str) -> &Self {
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
