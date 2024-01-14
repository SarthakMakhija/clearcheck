use std::fmt::Debug;

use crate::matchers::{Matcher, MatcherResult};

/// EqualityMatcher offers a flexible way to assert the equality between two values of the same type.
///
/// Works with any data type that implements the Eq trait.
///
/// clearcheck implements EqualityMatcher for any T: Eq + Debug.
pub struct EqualityMatcher<T: Eq> {
    pub other: T,
}

/// IgnoreCaseEqualityMatcher offers a flexible way to assert the equality between two values of same type, ignoring case differences.
///
/// clearcheck implements IgnoreCaseEqualityMatcher for the following:
/// - char
/// - &str
/// - Vec<T> where T: AsRef<str> + Debug + Eq,
/// - &[T]   where T: AsRef<str> + Debug + Eq,
/// - [String; N] and [&str; N]
pub struct IgnoreCaseEqualityMatcher<T: Eq> {
    pub other: T,
}

/// Creates an EqualityMatcher that asserts whether a value equals the given value.
pub fn equal<T: Eq>(other: T) -> EqualityMatcher<T> {
    EqualityMatcher { other }
}

/// Creates an IgnoreCaseEqualityMatcher that asserts whether a value equals the given value, ignoring case differences.
pub fn be_equal_ignoring_case<T: Eq>(other: T) -> IgnoreCaseEqualityMatcher<T> {
    IgnoreCaseEqualityMatcher { other }
}

impl<T: Eq + Debug> Matcher<T> for EqualityMatcher<T> {
    fn test(&self, value: &T) -> MatcherResult {
        MatcherResult::formatted(
            value == &self.other,
            format!("{:?} should equal {:?}", value, self.other),
            format!("{:?} should not equal {:?}", value, self.other),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::assertions::bool::TrueFalseAssertion;
    use crate::matchers::equal::equal;
    use crate::matchers::Matcher;

    #[derive(Debug, Eq, PartialEq)]
    struct Book {
        name: &'static str,
    }

    #[test]
    fn should_equal() {
        let books = [
            Book {
                name: "Database internals",
            },
            Book {
                name: "Rust in action",
            },
        ];
        let matcher = equal([
            Book {
                name: "Database internals",
            },
            Book {
                name: "Rust in action",
            },
        ]);
        matcher.test(&books).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_not_equal() {
        let books = vec![
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

        let matcher = equal(target);
        matcher.test(&books).passed.should_be_true();
    }
}
