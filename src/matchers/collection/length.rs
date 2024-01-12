use crate::matchers::{Matcher, MatcherResult};

pub enum CollectionLengthMatcher {
    Same(usize),
    Atleast(usize),
    Atmost(usize),
}

impl<T> Matcher<Vec<T>> for CollectionLengthMatcher {
    fn test(&self, collection: &Vec<T>) -> MatcherResult {
        self.test_length(collection.len())
    }
}

impl<T, const N: usize> Matcher<[T; N]> for CollectionLengthMatcher {
    fn test(&self, collection: &[T; N]) -> MatcherResult {
        self.test_length(collection.len())
    }
}

impl<T> Matcher<&[T]> for CollectionLengthMatcher {
    fn test(&self, collection: &&[T]) -> MatcherResult {
        self.test_length(collection.len())
    }
}

impl CollectionLengthMatcher {
    fn test_length(&self, input_length: usize) -> MatcherResult {
        let message_prefix = "Collection";
        match self {
            CollectionLengthMatcher::Same(length) => MatcherResult::formatted(
                input_length == *length,
                format!(
                    "{:?} length {:?} should be {:?}",
                    message_prefix, input_length, length
                ),
                format!(
                    "{:?} length {:?} should not be {:?}",
                    message_prefix, input_length, length
                ),
            ),
            CollectionLengthMatcher::Atleast(length) => MatcherResult::formatted(
                input_length >= *length,
                format!(
                    "{:?} length {:?} should be atleast {:?}",
                    message_prefix, input_length, length
                ),
                format!(
                    "{:?} length {:?} should not be atleast {:?}",
                    message_prefix, input_length, length
                ),
            ),
            CollectionLengthMatcher::Atmost(length) => MatcherResult::formatted(
                input_length <= *length,
                format!(
                    "{:?} length {:?} should be atmost {:?}",
                    message_prefix, input_length, length
                ),
                format!(
                    "{:?} length {:?} should not be atmost {:?}",
                    message_prefix, input_length, length
                ),
            ),
        }
    }
}

pub fn have_same_length(length: usize) -> CollectionLengthMatcher {
    CollectionLengthMatcher::Same(length)
}

pub fn have_atleast_same_length(length: usize) -> CollectionLengthMatcher {
    CollectionLengthMatcher::Atleast(length)
}

pub fn have_atmost_same_length(length: usize) -> CollectionLengthMatcher {
    CollectionLengthMatcher::Atmost(length)
}

#[cfg(test)]
mod tests {
    use crate::assertions::bool::TrueFalseAssertion;
    use crate::matchers::collection::length::{have_atleast_same_length, have_atmost_same_length, have_same_length};
    use crate::matchers::Matcher;

    #[test]
    fn should_have_same_length() {
        let matcher = have_same_length(4);
        matcher.test(&vec![1, 2, 3, 4]).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_have_same_length_but_was_not() {
        let matcher = have_same_length(5);
        matcher.test(&vec![1, 2, 3, 4]).passed.should_be_true();
    }

    #[test]
    fn should_have_atleast_same_length() {
        let matcher = have_atleast_same_length(3);
        matcher.test(&vec![1, 2, 3, 4]).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_have_atleast_same_length_but_was_not() {
        let matcher = have_atleast_same_length(6);
        matcher.test(&vec![1, 2, 3, 4]).passed.should_be_true();
    }

    #[test]
    fn should_have_atmost_length() {
        let matcher = have_atmost_same_length(4);
        matcher.test(&vec![1, 2, 3, 4]).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_have_atmost_length_but_was_not() {
        let matcher = have_atmost_same_length(3);
        matcher.test(&vec![1, 2, 3, 4]).passed.should_be_true();
    }
}
