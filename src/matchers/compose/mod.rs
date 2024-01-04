use std::fmt::Debug;

use crate::matchers::{Matcher, MatcherResult};

enum Kind {
    All,
    Any,
}

pub struct Matchers<'a, T: Debug> {
    matchers: Vec<&'a dyn Matcher<T>>,
    kind: Kind,
}

impl<'a, T: Debug> Matchers<'a, T> {
    pub fn all(matchers: Vec<&'a dyn Matcher<T>>) -> Self {
        Matchers {
            matchers,
            kind: Kind::All,
        }
    }

    pub fn any(matchers: Vec<&'a dyn Matcher<T>>) -> Self {
        Matchers {
            matchers,
            kind: Kind::Any,
        }
    }
}

impl<'a, T: Debug> Matcher<T> for Matchers<'a, T> {
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

fn messages<P, M>(results: &Vec<MatcherResult>, predicate: P, mapper: M) -> String
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
    use crate::assertions::bool::TrueFalse;
    use crate::matchers::compose::Matchers;
    use crate::matchers::length::have_atleast_same_length;
    use crate::matchers::string::boundary::{begin_with, end_with};
    use crate::matchers::Matcher;

    #[test]
    fn should_run_all_matchers_successfully() {
        let begin_with = &begin_with("go");
        let end_with = &end_with("select");
        let atleast_length = &have_atleast_same_length(4);

        let matchers: Matchers<&str> = Matchers::all(vec![begin_with, end_with, atleast_length]);

        let term = "goselect";
        matchers.test(&term).passed.should_be_true();
    }

    #[test]
    fn should_fail_one_of_all_matchers() {
        let begin_with = &begin_with("go");
        let end_with = &end_with("select");
        let atleast_length = &have_atleast_same_length(10);

        let matchers: Matchers<&str> = Matchers::all(vec![begin_with, end_with, atleast_length]);

        let term = "goselect";
        matchers.test(&term).passed.should_be_false();
    }

    #[test]
    fn should_run_any_of_all_matchers_successfully() {
        let begin_with = &begin_with("go");
        let end_with = &end_with("ted");
        let atleast_length = &have_atleast_same_length(8);

        let matchers: Matchers<&str> = Matchers::any(vec![begin_with, end_with, atleast_length]);

        let term = "goselect";
        matchers.test(&term).passed.should_be_true();
    }

    #[test]
    fn should_fail_all_of_any_matchers() {
        let begin_with = &begin_with("go");
        let end_with = &end_with("select");
        let atleast_length = &have_atleast_same_length(10);

        let matchers: Matchers<&str> = Matchers::all(vec![begin_with, end_with, atleast_length]);

        let term = "testify";
        matchers.test(&term).passed.should_be_false();
    }
}

#[cfg(test)]
mod slice_matchers {
    use crate::assertions::bool::TrueFalse;
    use crate::matchers::collection::duplicate::contain_duplicates;
    use crate::matchers::collection::membership::contain;
    use crate::matchers::compose::Matchers;
    use crate::matchers::length::{have_atleast_same_length, have_atmost_same_length};
    use crate::matchers::Matcher;

    #[test]
    fn should_run_all_matchers_successfully() {
        let contain = &contain(&"assert4j");
        let atmost_length = &have_atmost_same_length(4);
        let duplicates = &contain_duplicates();

        let matchers: Matchers<Vec<&str>> = Matchers::all(vec![contain, atmost_length, duplicates]);
        let collection = vec!["junit", "assert4j", "junit"];

        matchers.test(&collection).passed.should_be_true();
    }

    #[test]
    fn should_fail_one_of_all_matchers() {
        let contain = &contain(&"assert4j");
        let atmost_length = &have_atmost_same_length(1);
        let duplicates = &contain_duplicates();

        let matchers: Matchers<Vec<&str>> = Matchers::all(vec![contain, atmost_length, duplicates]);
        let collection = vec!["junit", "assert4j", "junit"];

        matchers.test(&collection).passed.should_be_false();
    }

    #[test]
    fn should_run_any_of_all_matchers_successfully() {
        let contain = &contain(&"assert4j");
        let atleast_length = &have_atleast_same_length(4);
        let duplicates = &contain_duplicates();

        let matchers: Matchers<Vec<&str>> =
            Matchers::any(vec![contain, atleast_length, duplicates]);
        let collection = vec!["junit", "assert4j", "testify"];

        matchers.test(&collection).passed.should_be_true();
    }

    #[test]
    fn should_fail_all_of_any_matchers() {
        let contain = &contain(&"assert4j");
        let atleast_length = &have_atleast_same_length(4);
        let duplicates = &contain_duplicates();

        let matchers: Matchers<Vec<&str>> =
            Matchers::any(vec![contain, atleast_length, duplicates]);
        let collection = vec!["junit", "assert", "testify1"];

        matchers.test(&collection).passed.should_be_false();
    }
}
