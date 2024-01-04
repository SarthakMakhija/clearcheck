use crate::matchers::length::LengthBased;
use crate::matchers::Matcher;

impl<T> Matcher<Vec<T>> for LengthBased {
    fn test(&self, collection: &Vec<T>) -> bool {
        self.test(collection.len())
    }
}

impl<T, const N: usize> Matcher<[T; N]> for LengthBased {
    fn test(&self, collection: &[T; N]) -> bool {
        self.test(collection.len())
    }
}

impl<T> Matcher<&[T]> for LengthBased {
    fn test(&self, collection: &&[T]) -> bool {
        self.test(collection.len())
    }
}
