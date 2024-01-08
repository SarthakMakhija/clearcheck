use std::fmt::Debug;

use crate::matchers::equal::equal;
use crate::matchers::{Should, ShouldNot};

pub trait EqualityAssertion<T: Eq> {
    fn should_equal(&self, other: &T) -> &Self;
    fn should_not_equal(&self, other: &T) -> &Self;
}

impl<T: Eq + Debug> EqualityAssertion<T> for T {
    fn should_equal(&self, other: &T) -> &Self {
        self.should(&equal(other));
        self
    }

    fn should_not_equal(&self, other: &T) -> &Self {
        self.should_not(&equal(other));
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
}
