use std::fmt::Debug;

use crate::matchers::{Matcher, MatcherResult};

enum Kind {
    All,
    Any,
}

pub struct Matchers<T: Debug> {
    matchers: Vec<Box<dyn Matcher<T>>>,
    kind: Kind,
}

impl<T: Debug> Matchers<T> {
    pub fn all(matchers: Vec<Box<dyn Matcher<T>>>) -> Self {
        Matchers {
            matchers,
            kind: Kind::All,
        }
    }

    pub fn any(matchers: Vec<Box<dyn Matcher<T>>>) -> Self {
        Matchers {
            matchers,
            kind: Kind::Any,
        }
    }
}

impl<T: Debug> Matcher<T> for Matchers<T> {
    fn test(&self, value: &T) -> MatcherResult {
        let results = self
            .matchers
            .iter()
            .map(|matcher| matcher.test(value))
            .collect::<Vec<_>>();

        match self.kind {
            Kind::All => MatcherResult::formatted(
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
            Kind::Any => MatcherResult::formatted(
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
    use crate::matchers::compose::Matchers;
    use crate::matchers::empty::be_empty;
    use crate::matchers::length::have_atleast_same_length;
    use crate::matchers::string::boundary::{begin_with, end_with};
    use crate::matchers::{BoxWrap, Invert, Matcher};

    #[test]
    fn should_run_all_matchers_successfully() {
        let begin_with = begin_with("go").wrap();
        let end_with = end_with("select").wrap();
        let atleast_length = have_atleast_same_length(4).wrap();

        let matchers = Matchers::all(vec![begin_with, end_with, atleast_length]);

        let term = "goselect";
        matchers.test(&term).passed.should_be_true();
    }

    #[test]
    fn should_fail_one_of_all_matchers() {
        let begin_with = begin_with("go").wrap();
        let end_with = end_with("select").wrap();
        let atleast_length = have_atleast_same_length(10).wrap();

        let matchers = Matchers::all(vec![begin_with, end_with, atleast_length]);

        let term = "goselect";
        matchers.test(&term).passed.should_be_false();
    }

    #[test]
    fn should_run_any_of_all_matchers_successfully() {
        let begin_with = begin_with("go").wrap();
        let end_with = end_with("ted").wrap();
        let atleast_length = have_atleast_same_length(8).wrap();

        let matchers = Matchers::any(vec![begin_with, end_with, atleast_length]);

        let term = "goselect";
        matchers.test(&term).passed.should_be_true();
    }

    #[test]
    fn should_fail_all_of_any_matchers() {
        let begin_with = begin_with("go").wrap();
        let end_with = end_with("select").wrap();
        let atleast_length = have_atleast_same_length(10).wrap();

        let matchers = Matchers::all(vec![begin_with, end_with, atleast_length]);

        let term = "testify";
        matchers.test(&term).passed.should_be_false();
    }

    #[test]
    fn should_run_negated_matchers_successfully() {
        let begin_with = begin_with("go").wrap();
        let not_end_with = end_with("test").invert().wrap();
        let not_be_empty = be_empty().invert().wrap();

        let matchers = Matchers::all(vec![begin_with, not_end_with, not_be_empty]);

        let term = "goselect";
        matchers.test(&term).passed.should_be_true();
    }
}

#[cfg(test)]
mod slice_matchers {
    use crate::assertions::bool::TrueFalseAssertion;
    use crate::matchers::collection::duplicate::contain_duplicates;
    use crate::matchers::collection::membership::contain;
    use crate::matchers::compose::Matchers;
    use crate::matchers::length::{have_atleast_same_length, have_atmost_same_length};
    use crate::matchers::{BoxWrap, Matcher};

    #[test]
    fn should_run_all_matchers_successfully() {
        let contain = contain(&"assert4j").wrap();
        let atmost_length = have_atmost_same_length(4).wrap();
        let duplicates = contain_duplicates().wrap();

        let matchers = Matchers::all(vec![contain, atmost_length, duplicates]);
        let collection = vec!["junit", "assert4j", "junit"];

        matchers.test(&collection).passed.should_be_true();
    }

    #[test]
    fn should_fail_one_of_all_matchers() {
        let contain = contain(&"assert4j").wrap();
        let atmost_length = have_atmost_same_length(1).wrap();
        let duplicates = contain_duplicates().wrap();

        let matchers = Matchers::all(vec![contain, atmost_length, duplicates]);
        let collection = vec!["junit", "assert4j", "junit"];

        matchers.test(&collection).passed.should_be_false();
    }

    #[test]
    fn should_run_any_of_all_matchers_successfully() {
        let contain = contain(&"assert4j").wrap();
        let atleast_length = have_atleast_same_length(4).wrap();
        let duplicates = contain_duplicates().wrap();

        let matchers = Matchers::any(vec![contain, atleast_length, duplicates]);
        let collection = vec!["junit", "assert4j", "testify"];

        matchers.test(&collection).passed.should_be_true();
    }

    #[test]
    fn should_fail_all_of_any_matchers() {
        let contain = contain(&"assert4j").wrap();
        let atleast_length = have_atleast_same_length(4).wrap();
        let duplicates = contain_duplicates().wrap();

        let matchers = Matchers::any(vec![contain, atleast_length, duplicates]);
        let collection = vec!["junit", "assert", "testify1"];

        matchers.test(&collection).passed.should_be_false();
    }
}
