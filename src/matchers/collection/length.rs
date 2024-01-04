use crate::matchers::length::LengthBased;
use crate::matchers::Matcher;

impl<T> Matcher<&[T]> for LengthBased {
    fn test(&self, collection: &&[T]) -> bool {
        self.test_length(collection.len())
    }
}
