use std::fmt::Debug;

use crate::matchers::{Matcher, MatcherResult};

pub struct EqualityMatcher<'a, T: Eq> {
    pub other: &'a T,
}

pub struct IgnoreCaseEqualityMatcher<'a, T: Eq> {
    pub other: &'a T,
}

pub fn equal<T: Eq>(other: &T) -> EqualityMatcher<'_, T> {
    EqualityMatcher { other }
}

pub fn be_equal_ignoring_case<T: Eq>(other: &T) -> IgnoreCaseEqualityMatcher<'_, T> {
    IgnoreCaseEqualityMatcher { other }
}

impl<T: Eq + Debug> Matcher<T> for EqualityMatcher<'_, T> {
    fn test(&self, value: &T) -> MatcherResult {
        MatcherResult::formatted(
            value == self.other,
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
        let books = &[
            Book {
                name: "Database internals",
            },
            Book {
                name: "Rust in action",
            },
        ];
        let matcher = equal(&[
            Book {
                name: "Database internals",
            },
            Book {
                name: "Rust in action",
            },
        ]);
        matcher.test(books).passed.should_be_true();
    }

    #[test]
    #[should_panic]
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

        let matcher = equal(&target);
        matcher.test(books).passed.should_be_true();
    }
}
