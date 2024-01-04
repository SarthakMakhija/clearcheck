use crate::matchers::length::LengthBased;
use crate::matchers::Matcher;

impl<T> Matcher<&[T]> for LengthBased {
    fn test(&self, collection: &&[T]) -> bool {
        self.test_length(collection.len())
    }
}

impl<T> Matcher<Vec<T>> for LengthBased {
    fn test(&self, collection: &Vec<T>) -> bool {
        self.test_length(collection.len())
    }
}
