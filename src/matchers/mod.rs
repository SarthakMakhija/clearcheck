//! Matchers provide the granular tools for carrying out the assertions.
//! They examine the data and verify that the data conforms to specific criteria.
//!
//! Let's take an example of a collection matcher.
//!
//! ```
//! use clearcheck::matchers::collection::membership::contain_all;
//! use clearcheck::matchers::Matcher;
//!
//! let collection = vec!["clearcheck", "testify", "assert4j", "xunit"];
//! let all_to_be_contained = vec!["testify", "assert4j", "xunit"];
//!
//! let matcher = contain_all(all_to_be_contained);
//! assert!(matcher.test(&collection).passed());
//! ```

pub mod bool;
pub mod char;
pub mod collection;
pub mod compose;
#[cfg(feature = "date")]
pub mod date;
pub mod equal;
#[cfg(feature = "file")]
pub mod file;
#[cfg(feature = "num")]
pub mod float;
#[cfg(feature = "num")]
pub mod int;
pub mod map;
pub mod option;
pub mod ordered;
pub mod range;
pub mod result;
pub mod string;

/// Should provides a convenient way to express positive assertions within tests, indicating that a value should meet a certain condition.
pub trait Should<T> {
    /// - Takes a matcher as input and performs an assertion against the value itself.
    /// - Panics if the assertion fails, indicating that the value did not match the matcher's expectations.
    fn should(&self, matcher: &dyn Matcher<T>);
}

/// ShouldNot provides a convenient way to express negative assertions within tests, indicating that a value should not meet a certain condition.
pub trait ShouldNot<T> {
    /// - Takes a matcher as input and performs an inverted assertion against the value itself.
    /// - Inverts the result of the matcher's test method, ensuring the value does not match.
    /// - Panics if the inverted assertion fails, indicating that the value unexpectedly matched the matcher.
    fn should_not(&self, matcher: &dyn Matcher<T>);
}

impl<T> Should<T> for T {
    fn should(&self, matcher: &dyn Matcher<T>) {
        let matcher_result = matcher.test(self);
        if !matcher_result.passed {
            panic!("assertion failed: {}", matcher_result.failure_message);
        }
    }
}

impl<T> ShouldNot<T> for T {
    fn should_not(&self, matcher: &dyn Matcher<T>) {
        let matcher_result = matcher.test(self);
        let passed = !matcher_result.passed;
        if !passed {
            panic!(
                "assertion failed: {}",
                matcher_result.inverted_failure_message
            );
        }
    }
}

/// Matcher defines the core functionality of matchers. All the matchers implement `Matcher<T>` trait.
pub trait Matcher<T> {
    fn test(&self, value: &T) -> MatcherResult;
}

/// BoxWrap provides a `boxed` method to wrap a Matcher into Box object.
///
/// It is used to compose matchers in [`crate::matchers::compose::Matchers`].
///
/// BoxWrap is implemented for any `T: Matcher<M>`.
pub trait BoxWrap<W> {
    fn boxed(self) -> Box<dyn Matcher<W>>;
}

impl<M, T: Matcher<M> + 'static> BoxWrap<M> for T {
    fn boxed(self) -> Box<dyn Matcher<M>> {
        Box::new(self)
    }
}

/// MatcherResult defines the result of a matcher execution.
pub struct MatcherResult {
    passed: bool,
    failure_message: String,
    inverted_failure_message: String,
}

impl MatcherResult {
    /// Creates a new instance of MatcherResult using failure_message and inverted_failure_message of type &'static str.
    pub fn new(
        passed: bool,
        failure_message: &'static str,
        inverted_failure_message: &'static str,
    ) -> Self {
        MatcherResult::formatted(
            passed,
            failure_message.to_string(),
            inverted_failure_message.to_string(),
        )
    }

    /// Creates a new instance of MatcherResult using failure_message and inverted_failure_message of type String.
    pub fn formatted(
        passed: bool,
        failure_message: String,
        inverted_failure_message: String,
    ) -> Self {
        MatcherResult {
            passed,
            failure_message,
            inverted_failure_message,
        }
    }

    /// Returns true if the result of a matcher execution was successful, false otherwise.
    pub fn passed(&self) -> bool {
        self.passed
    }
}
