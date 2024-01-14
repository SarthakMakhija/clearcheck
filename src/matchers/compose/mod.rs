use std::fmt::Debug;

use crate::matchers::{Matcher, MatcherResult};

enum Kind {
    And,
    Or,
}

/// MatcherBehavior encapsulates a matcher and an inversion flag, governing how it's applied in assertions.
pub struct MatcherBehavior<T: Debug> {
    matcher: Box<dyn Matcher<T>>,
    inverted: bool,
}

impl<T: Debug> MatcherBehavior<T> {
    /// Creates a new instance of MatcherBehavior encapsulating the given matcher.
    pub fn new(matcher: Box<dyn Matcher<T>>) -> Self {
        MatcherBehavior {
            matcher,
            inverted: false,
        }
    }

    /// Creates a new instance of MatcherBehavior encapsulating the given matcher with an inversion flag.
    pub fn inverted(matcher: Box<dyn Matcher<T>>) -> Self {
        MatcherBehavior {
            matcher,
            inverted: true,
        }
    }

    /// Runs the underlying matcher.
    pub fn run_matcher(&self, value: &T) -> MatcherResult {
        let matcher_result = self.matcher.test(value);
        if self.inverted {
            return MatcherResult::formatted(
                !matcher_result.passed,
                matcher_result.inverted_failure_message,
                matcher_result.failure_message,
            );
        }
        matcher_result
    }
}

/// MatchersBuilder provides an elegant way to compose various matchers.
///
/// # Example
///```
/// use clearcheck::matchers::{BoxWrap, Matcher};
/// use clearcheck::matchers::compose::MatchersBuilder;
/// use clearcheck::matchers::string::boundary::begin_with;
/// use clearcheck::matchers::string::empty::be_empty;
/// use clearcheck::matchers::string::length::have_atleast_same_length;
/// use clearcheck::matchers::string::membership::{contain_a_digit, contain_any_of_characters, contain_ignoring_case};
///
/// let matchers = MatchersBuilder::start_building_with_inverted(be_empty().boxed())
///    .push(have_atleast_same_length(10).boxed())
///    .push(contain_a_digit().boxed())
///    .push(contain_any_of_characters(vec!['@', '#']).boxed())
///    .push_inverted(begin_with("pass").boxed())
///    .push_inverted(contain_ignoring_case("pass").boxed())
///    .push_inverted(contain_ignoring_case("word").boxed())
///    .combine_as_and();
///
/// let password = "P@@sw0rd9082";
/// assert!(matchers.test(&password).passed());
/// ```
pub struct MatchersBuilder<T: Debug> {
    matchers_behaviors: Vec<MatcherBehavior<T>>,
}

impl<T: Debug> MatchersBuilder<T> {
    /// Creates an instance of MatchersBuilder with the given matcher.
    pub fn start_building(matcher: Box<dyn Matcher<T>>) -> Self {
        MatchersBuilder {
            matchers_behaviors: vec![MatcherBehavior::new(matcher)]
        }
    }

    /// Creates an instance of MatchersBuilder with the given matcher inverted.
    pub fn start_building_with_inverted(matcher: Box<dyn Matcher<T>>) -> Self {
        MatchersBuilder {
            matchers_behaviors: vec![MatcherBehavior::inverted(matcher)]
        }
    }

    /// Pushes the instance of the given matcher to the MatchersBuilder.
    pub fn push(mut self, matcher: Box<dyn Matcher<T>>) -> Self {
        self.matchers_behaviors.push(MatcherBehavior::new(matcher));
        self
    }

    /// Pushes the instance of the given matcher inverted to the MatchersBuilder.
    pub fn push_inverted(mut self, matcher: Box<dyn Matcher<T>>) -> Self {
        self.matchers_behaviors.push(MatcherBehavior::inverted(matcher));
        self
    }

    /// Combines all the matchers using AND operator.
    /// All the matchers must pass for Matchers to pass.
    pub fn combine_as_and(self) -> Matchers<T> {
        Matchers::and(self.matchers_behaviors)
    }

    /// Combines all the matchers using OR operator.
    /// Any of the matchers must pass for Matchers to pass.
    pub fn combine_as_or(self) -> Matchers<T> {
        Matchers::or(self.matchers_behaviors)
    }
}

/// Matchers provides a way to combine various matchers using AND or OR operators.
/// If an instance of Matchers is created using AND operator, all the underlying matchers MUST pass for Matchers to pass.
/// If an instance of Matchers is created using OR operator, any of the underlying matchers MUST pass for Matchers to pass.
pub struct Matchers<T: Debug> {
    matcher_behaviors: Vec<MatcherBehavior<T>>,
    kind: Kind,
}

impl<T: Debug> Matchers<T> {
    fn and(matchers: Vec<MatcherBehavior<T>>) -> Self {
        Matchers {
            matcher_behaviors: matchers,
            kind: Kind::And,
        }
    }

    fn or(matchers: Vec<MatcherBehavior<T>>) -> Self {
        Matchers {
            matcher_behaviors: matchers,
            kind: Kind::Or,
        }
    }
}

/// Matchers implement the [`crate::matchers::Matcher`] trait.
impl<T: Debug> Matcher<T> for Matchers<T> {
    fn test(&self, value: &T) -> MatcherResult {
        let results = self
            .matcher_behaviors
            .iter()
            .map(|matcher_behavior| matcher_behavior.run_matcher(value))
            .collect::<Vec<_>>();

        match self.kind {
            Kind::And => MatcherResult::formatted(
                results.iter().all(|result| result.passed),
                messages(
                    &results,
                    |result| !result.passed,
                    |result| result.failure_message.clone(),
                ),
                messages(
                    &results,
                    |result| result.passed,
                    |result| result.inverted_failure_message.clone(),
                ),
            ),
            Kind::Or => MatcherResult::formatted(
                results.iter().any(|result| result.passed),
                messages(&results, |_| true, |result| result.failure_message.clone()),
                messages(
                    &results,
                    |_| true,
                    |result| result.inverted_failure_message.clone(),
                ),
            ),
        }
    }
}

fn messages<P, M>(results: &[MatcherResult], predicate: P, mapper: M) -> String
    where
        P: FnMut(&&MatcherResult) -> bool,
        M: FnMut(&MatcherResult) -> String,
{
    results
        .iter()
        .filter(predicate)
        .map(mapper)
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod string_matchers {
    use crate::assertions::bool::TrueFalseAssertion;
    use crate::matchers::{BoxWrap, Matcher};
    use crate::matchers::compose::MatchersBuilder;
    use crate::matchers::string::boundary::{begin_with, end_with};
    use crate::matchers::string::empty::be_empty;
    use crate::matchers::string::length::have_atleast_same_length;

    #[test]
    fn should_run_all_matchers_successfully() {
        let begin_with = begin_with("go").boxed();
        let end_with = end_with("select").boxed();
        let atleast_length = have_atleast_same_length(4).boxed();

        let matchers = MatchersBuilder::start_building(begin_with).push(end_with).push(atleast_length).combine_as_and();

        let term = "goselect";
        matchers.test(&term).passed.should_be_true();
    }

    #[test]
    fn should_fail_one_of_matchers() {
        let begin_with = begin_with("go").boxed();
        let end_with = end_with("select").boxed();
        let atleast_length = have_atleast_same_length(10).boxed();

        let matchers = MatchersBuilder::start_building(begin_with).push(end_with).push(atleast_length).combine_as_and();

        let term = "goselect";
        matchers.test(&term).passed.should_be_false();
    }

    #[test]
    fn should_run_any_of_the_matchers_successfully() {
        let begin_with = begin_with("go").boxed();
        let end_with = end_with("ted").boxed();
        let atleast_length = have_atleast_same_length(8).boxed();

        let matchers = MatchersBuilder::start_building(begin_with).push(end_with).push(atleast_length).combine_as_or();

        let term = "goselect";
        matchers.test(&term).passed.should_be_true();
    }

    #[test]
    fn should_fail_all_matchers() {
        let begin_with = begin_with("go").boxed();
        let end_with = end_with("select").boxed();
        let atleast_length = have_atleast_same_length(10).boxed();

        let matchers = MatchersBuilder::start_building(begin_with).push(end_with).push(atleast_length).combine_as_or();

        let term = "testify";
        matchers.test(&term).passed.should_be_false();
    }

    #[test]
    fn should_run_inverted_matchers_successfully() {
        let begin_with = begin_with("go").boxed();
        let not_end_with = end_with("test").boxed();
        let not_be_empty = be_empty().boxed();

        let matchers = MatchersBuilder::start_building(begin_with).push_inverted(not_end_with).push_inverted(not_be_empty).combine_as_or();

        let term = "goselect";
        matchers.test(&term).passed.should_be_true();
    }
}


#[cfg(test)]
mod slice_matchers {
    use crate::assertions::bool::TrueFalseAssertion;
    use crate::matchers::{BoxWrap, Matcher};
    use crate::matchers::collection::duplicate::contain_duplicates;
    use crate::matchers::collection::length::{have_atleast_same_length, have_atmost_same_length};
    use crate::matchers::collection::membership::contain;
    use crate::matchers::compose::MatchersBuilder;

    #[test]
    fn should_run_all_matchers_successfully() {
        let contain = contain("assert4j").boxed();
        let atmost_length = have_atmost_same_length(4).boxed();
        let duplicates = contain_duplicates().boxed();

        let matchers = MatchersBuilder::start_building(contain).push(atmost_length).push(duplicates).combine_as_and();
        let collection = vec!["junit", "assert4j", "junit"];

        matchers.test(&collection).passed.should_be_true();
    }

    #[test]
    fn should_fail_one_matcher() {
        let contain = contain("assert4j").boxed();
        let atmost_length = have_atmost_same_length(1).boxed();
        let duplicates = contain_duplicates().boxed();

        let matchers = MatchersBuilder::start_building(contain).push(atmost_length).push(duplicates).combine_as_and();
        let collection = vec!["junit", "assert4j", "junit"];

        matchers.test(&collection).passed.should_be_false();
    }

    #[test]
    fn should_run_any_of_the_matchers_successfully() {
        let contain = contain("assert4j").boxed();
        let atleast_length = have_atleast_same_length(4).boxed();
        let duplicates = contain_duplicates().boxed();

        let matchers = MatchersBuilder::start_building(contain).push(atleast_length).push(duplicates).combine_as_or();
        let collection = vec!["junit", "assert4j", "testify"];

        matchers.test(&collection).passed.should_be_true();
    }

    #[test]
    fn should_fail_all_matchers() {
        let contain = contain("assert4j").boxed();
        let atleast_length = have_atleast_same_length(4).boxed();
        let duplicates = contain_duplicates().boxed();

        let matchers = MatchersBuilder::start_building(contain).push(atleast_length).push(duplicates).combine_as_or();
        let collection = vec!["junit", "assert", "testify1"];

        matchers.test(&collection).passed.should_be_false();
    }

    #[test]
    fn should_run_all_inverted_matchers_successfully() {
        let contain = contain("assert4j").boxed();
        let atmost_length = have_atmost_same_length(4).boxed();
        let duplicates = contain_duplicates().boxed();

        let matchers = MatchersBuilder::start_building(contain).push(atmost_length).push_inverted(duplicates).combine_as_and();
        let collection = vec!["junit", "assert4j", "clearcheck"];

        matchers.test(&collection).passed.should_be_true();
    }
}

#[cfg(test)]
mod custom_string_matchers_tests {
    use std::fmt::Debug;

    use crate::matchers::{BoxWrap, Should};
    use crate::matchers::compose::{Matchers, MatchersBuilder};
    use crate::matchers::string::boundary::begin_with;
    use crate::matchers::string::empty::be_empty;
    use crate::matchers::string::length::have_atleast_same_length;
    use crate::matchers::string::membership::{contain_a_digit, contain_any_of_characters, contain_ignoring_case};

    fn be_a_valid_password<T: AsRef<str> + Debug>() -> Matchers<T> {
        MatchersBuilder::start_building_with_inverted(be_empty().boxed())
            .push(have_atleast_same_length(10).boxed())
            .push(contain_a_digit().boxed())
            .push(contain_any_of_characters(vec!['@', '#']).boxed())
            .push_inverted(begin_with("pass").boxed())
            .push_inverted(contain_ignoring_case("pass").boxed())
            .push_inverted(contain_ignoring_case("word").boxed())
            .combine_as_and()
    }

    trait PasswordAssertion {
        fn should_be_a_valid_password(&self) -> &Self;
    }

    impl PasswordAssertion for &str {
        fn should_be_a_valid_password(&self) -> &Self {
            self.should(&be_a_valid_password());
            self
        }
    }

    #[test]
    fn should_be_a_valid_password() {
        let password = "P@@sw0rd9082";
        password.should_be_a_valid_password();
    }

    #[test]
    #[should_panic]
    fn should_not_be_a_valid_password() {
        let password = "P@@sword9082";
        password.should_be_a_valid_password();
    }
}

#[cfg(test)]
mod custom_collection_matchers_tests {
    use std::fmt::Debug;

    use crate::matchers::{BoxWrap, Should};
    use crate::matchers::collection::empty::be_empty;
    use crate::matchers::collection::length::have_atleast_same_length;
    use crate::matchers::collection::membership::contain_all;
    use crate::matchers::collection::sort::be_sorted_ascending;
    use crate::matchers::compose::{Matchers, MatchersBuilder};

    #[derive(Debug, Eq, PartialEq, PartialOrd)]
    enum LaptopBrands {
        Apple,
        Asus,
        Dell,
        Lenovo,
    }

    fn be_valid_laptop_brands(all: Vec<LaptopBrands>) -> Matchers<Vec<LaptopBrands>> {
        let empty = be_empty();
        let size = have_atleast_same_length(3);
        let contain_all = contain_all(all);
        let sorted = be_sorted_ascending();

        MatchersBuilder::<Vec<LaptopBrands>>::start_building_with_inverted(empty.boxed())
            .push(size.boxed())
            .push(contain_all.boxed())
            .push(sorted.boxed())
            .combine_as_and()
    }

    trait LaptopAssertion {
        fn should_be_valid_laptop_brands(&self) -> &Self;
    }

    impl LaptopAssertion for Vec<LaptopBrands> {
        fn should_be_valid_laptop_brands(&self) -> &Self {
            self.should(&be_valid_laptop_brands(vec![LaptopBrands::Dell]));
            self
        }
    }

    #[test]
    fn should_be_a_valid_collection_of_laptop_brands() {
        let brands = vec![LaptopBrands::Apple, LaptopBrands::Asus, LaptopBrands::Dell, LaptopBrands::Lenovo];
        brands.should_be_valid_laptop_brands();
    }

    #[test]
    #[should_panic]
    fn should_not_be_a_valid_collection_of_laptop_brands() {
        let brands = vec![LaptopBrands::Apple, LaptopBrands::Asus, LaptopBrands::Lenovo];
        brands.should_be_valid_laptop_brands();
    }
}