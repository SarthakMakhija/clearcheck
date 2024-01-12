use std::fmt::Debug;

use crate::matchers::{Matcher, MatcherResult};

enum Kind {
    And,
    Or,
}

pub struct MatcherBehavior<T: Debug> {
    matcher: Box<dyn Matcher<T>>,
    inverted: bool,
}

impl<T: Debug> MatcherBehavior<T> {
    fn new(matcher: Box<dyn Matcher<T>>) -> Self {
        MatcherBehavior {
            matcher,
            inverted: false,
        }
    }

    fn inverted(matcher: Box<dyn Matcher<T>>) -> Self {
        MatcherBehavior {
            matcher,
            inverted: true,
        }
    }

    fn run_matcher(&self, value: &T) -> MatcherResult {
        let matcher_result = self.matcher.test(value);
        if self.inverted {
            return MatcherResult::formatted(
                !matcher_result.passed,
                matcher_result.negated_failure_message,
                matcher_result.failure_message,
            );
        }
        matcher_result
    }
}

pub struct MatchersBuilder<T: Debug> {
    matchers_behaviors: Vec<MatcherBehavior<T>>,
}

impl<T: Debug> MatchersBuilder<T> {
    pub fn start_building(matcher: Box<dyn Matcher<T>>) -> Self {
        MatchersBuilder {
            matchers_behaviors: vec![MatcherBehavior::new(matcher)]
        }
    }

    pub fn start_building_with_negated(matcher: Box<dyn Matcher<T>>) -> Self {
        MatchersBuilder {
            matchers_behaviors: vec![MatcherBehavior::inverted(matcher)]
        }
    }

    pub fn push(mut self, matcher: Box<dyn Matcher<T>>) -> Self {
        self.matchers_behaviors.push(MatcherBehavior::new(matcher));
        self
    }

    pub fn push_inverted(mut self, matcher: Box<dyn Matcher<T>>) -> Self {
        self.matchers_behaviors.push(MatcherBehavior::inverted(matcher));
        self
    }

    pub fn combine_as_and(self) -> Matchers<T> {
        Matchers::and(self.matchers_behaviors)
    }

    pub fn combine_as_or(self) -> Matchers<T> {
        Matchers::or(self.matchers_behaviors)
    }
}

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
                    |result| result.negated_failure_message.clone(),
                ),
            ),
            Kind::Or => MatcherResult::formatted(
                results.iter().any(|result| result.passed),
                messages(&results, |_| true, |result| result.failure_message.clone()),
                messages(
                    &results,
                    |_| true,
                    |result| result.negated_failure_message.clone(),
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
    fn should_run_negated_matchers_successfully() {
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
        let contain = contain(&"assert4j").boxed();
        let atmost_length = have_atmost_same_length(4).boxed();
        let duplicates = contain_duplicates().boxed();

        let matchers = MatchersBuilder::start_building(contain).push(atmost_length).push(duplicates).combine_as_and();
        let collection = vec!["junit", "assert4j", "junit"];

        matchers.test(&collection).passed.should_be_true();
    }

    #[test]
    fn should_fail_one_matcher() {
        let contain = contain(&"assert4j").boxed();
        let atmost_length = have_atmost_same_length(1).boxed();
        let duplicates = contain_duplicates().boxed();

        let matchers = MatchersBuilder::start_building(contain).push(atmost_length).push(duplicates).combine_as_and();
        let collection = vec!["junit", "assert4j", "junit"];

        matchers.test(&collection).passed.should_be_false();
    }

    #[test]
    fn should_run_any_of_the_matchers_successfully() {
        let contain = contain(&"assert4j").boxed();
        let atleast_length = have_atleast_same_length(4).boxed();
        let duplicates = contain_duplicates().boxed();

        let matchers = MatchersBuilder::start_building(contain).push(atleast_length).push(duplicates).combine_as_or();
        let collection = vec!["junit", "assert4j", "testify"];

        matchers.test(&collection).passed.should_be_true();
    }

    #[test]
    fn should_fail_all_matchers() {
        let contain = contain(&"assert4j").boxed();
        let atleast_length = have_atleast_same_length(4).boxed();
        let duplicates = contain_duplicates().boxed();

        let matchers = MatchersBuilder::start_building(contain).push(atleast_length).push(duplicates).combine_as_or();
        let collection = vec!["junit", "assert", "testify1"];

        matchers.test(&collection).passed.should_be_false();
    }

    #[test]
    fn should_run_all_inverted_matchers_successfully() {
        let contain = contain(&"assert4j").boxed();
        let atmost_length = have_atmost_same_length(4).boxed();
        let duplicates = contain_duplicates().boxed();

        let matchers = MatchersBuilder::start_building(contain).push(atmost_length).push_inverted(duplicates).combine_as_and();
        let collection = vec!["junit", "assert4j", "clearcheck"];

        matchers.test(&collection).passed.should_be_true();
    }
}