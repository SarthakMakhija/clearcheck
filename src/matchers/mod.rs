pub mod bool;
pub mod char;
pub mod collection;
pub mod compose;
pub mod empty;
pub mod equal;
pub mod length;
pub mod map;
pub mod option;
pub mod range;
pub mod result;
pub mod string;

pub trait Should<T> {
    fn should(&self, matcher: &dyn Matcher<T>);
}

pub trait ShouldNot<T> {
    fn should_not(&self, matcher: &dyn Matcher<T>);
}

impl<T> Should<T> for T {
    fn should(&self, matcher: &dyn Matcher<T>) {
        let matcher_result = matcher.test(&self);
        if !matcher_result.passed {
            panic!("assertion failed: {}", matcher_result.failure_message);
        }
    }
}

impl<T> ShouldNot<T> for T {
    fn should_not(&self, matcher: &dyn Matcher<T>) {
        let matcher_result = matcher.test(&self);
        let passed = !matcher_result.passed;
        if !passed {
            panic!(
                "assertion failed: {}",
                matcher_result.negated_failure_message
            );
        }
    }
}

pub trait Matcher<T> {
    fn test(&self, value: &T) -> MatcherResult;
}

pub struct MatcherResult {
    passed: bool,
    failure_message: String,
    negated_failure_message: String,
}

impl MatcherResult {
    pub fn new(
        passed: bool,
        failure_message: &'static str,
        negated_failure_message: &'static str,
    ) -> Self {
        MatcherResult::formatted(
            passed,
            failure_message.to_string(),
            negated_failure_message.to_string(),
        )
    }

    pub fn formatted(
        passed: bool,
        failure_message: String,
        negated_failure_message: String,
    ) -> Self {
        MatcherResult {
            passed,
            failure_message,
            negated_failure_message,
        }
    }
}
