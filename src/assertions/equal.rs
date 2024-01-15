use std::borrow::Borrow;
use std::fmt::Debug;

use crate::matchers::{Should, ShouldNot};
use crate::matchers::equal::equal;

/// EqualityAssertion enables assertions about the equality of two values of type T: Eq.
pub trait EqualityAssertion<T: Eq> {
    /// - Asserts that the value held by self is equal to other.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::equal::EqualityAssertion;
    ///
    /// #[derive(Debug, Eq, PartialEq)]
    /// struct Book {
    ///  name: &'static str,
    /// }
    ///
    /// let books = vec![
    ///     Book {name: "Database internals"},
    ///     Book {name: "Rust in action"}
    /// ];
    /// let other = vec![
    ///     Book {name: "Database internals"},
    ///     Book {name: "Rust in action"}
    /// ];
    /// books.should_equal(&other);
    /// ```
    fn should_equal<Q>(&self, other: &Q) -> &Self
        where
            T: Borrow<Q>,
            Q: Eq + Debug + ?Sized;

    /// - Asserts that the value held by self is not equal to other.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::equal::EqualityAssertion;
    ///
    /// #[derive(Debug, Eq, PartialEq)]
    /// struct Book {
    ///  name: &'static str,
    /// }
    ///
    /// let books = vec![
    ///     Book {name: "Database internals"},
    /// ];
    /// let other = vec![
    ///     Book {name: "Database internals"},
    ///     Book {name: "Rust in action"},
    /// ];
    /// books.should_not_equal(&other);
    /// ```
    fn should_not_equal<Q>(&self, other: &Q) -> &Self
        where
            T: Borrow<Q>,
            Q: Eq + Debug + ?Sized;
}

impl<T: Eq + Debug> EqualityAssertion<T> for T {
    fn should_equal<Q>(&self, other: &Q) -> &Self
        where
            T: Borrow<Q>,
            Q: Eq + Debug + ?Sized,
    {
        self.borrow().should(&equal(other));
        self
    }

    fn should_not_equal<Q>(&self, other: &Q) -> &Self
        where
            T: Borrow<Q>,
            Q: Eq + Debug + ?Sized,
    {
        self.borrow().should_not(&equal(other));
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::assertions::equal::EqualityAssertion;

    #[derive(Debug, Eq, PartialEq)]
    struct Book {
        name: &'static str,
    }

    #[test]
    fn should_equal() {
        let books = &vec![
            Book {
                name: "Database internals",
            },
            Book {
                name: "Rust in action",
            },
        ];
        let target = vec![
            Book {
                name: "Database internals",
            },
            Book {
                name: "Rust in action",
            },
        ];
        books.should_equal(&target);
    }

    #[test]
    #[should_panic]
    fn should_equal_but_was_not() {
        let books = &vec![
            Book {
                name: "Database internals",
            },
            Book {
                name: "Rust in action",
            },
        ];
        let target = vec![
            Book {
                name: "Database internals",
            },
            Book {
                name: "Learning Rust",
            },
        ];
        books.should_equal(&target);
    }

    #[test]
    fn should_not_equal() {
        let books = &vec![
            Book {
                name: "Database internals",
            },
            Book {
                name: "Rust in action",
            },
        ];
        let target = vec![Book {
            name: "Database internals",
        }];
        books.should_not_equal(&target);
    }

    #[test]
    #[should_panic]
    fn should_not_equal_but_was() {
        let books = &vec![
            Book {
                name: "Database internals",
            },
            Book {
                name: "Rust in action",
            },
        ];
        let target = vec![
            Book {
                name: "Database internals",
            },
            Book {
                name: "Rust in action",
            },
        ];
        books.should_not_equal(&target);
    }

    #[test]
    fn should_equal_strings() {
        let name = "junit";
        name.should_equal("junit");
    }
}
