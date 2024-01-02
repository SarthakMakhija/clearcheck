use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use crate::matchers::collection::size::Size;
use crate::panicking::{assert_failed_binary, AssertKind};

impl<K, V> Size for HashMap<K, V>
    where K: Hash + Eq + PartialEq + Debug,
          V: Debug {
    fn should_have_size(&self, size: usize) -> &Self {
        if self.len() != size {
            assert_failed_binary(AssertKind::EqualSize, self, &size);
        }
        self
    }

    fn should_not_have_size(&self, size: usize) -> &Self {
        if self.len() == size {
            assert_failed_binary(AssertKind::NotEqualSize, self, &size);
        }
        self
    }

    fn should_have_at_least_size(&self, size: usize) -> &Self {
        if !(self.len() >= size) {
            assert_failed_binary(AssertKind::AtleastSize, self, &size);
        }
        self
    }

    fn should_have_at_most_size(&self, size: usize) -> &Self {
        if !(self.len() <= size) {
            assert_failed_binary(AssertKind::AtmostSize, self, &size);
        }
        self
    }

    fn should_be_same_size_as<U>(&self, other: &[U]) -> &Self {
        if self.len() != other.len() {
            assert_failed_binary(AssertKind::EqualSize, self, &other.len());
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::matchers::collection::size::Size;

    #[test]
    fn should_have_size() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.should_have_size(1);
    }

    #[test]
    #[should_panic]
    fn should_have_size_but_was_not() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.should_have_size(3);
    }

    #[test]
    fn should_not_have_size() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.should_not_have_size(3);
    }

    #[test]
    #[should_panic]
    fn should_not_have_size_but_was() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.should_not_have_size(1);
    }

    #[test]
    fn should_have_at_least_size() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.should_have_at_least_size(1);
    }

    #[test]
    #[should_panic]
    fn should_have_at_least_size_but_was_not() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.should_have_at_least_size(2);
    }

    #[test]
    fn should_have_at_most_size() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.should_have_at_most_size(1);
    }

    #[test]
    #[should_panic]
    fn should_have_at_most_size_but_was_not() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.insert("java", "assert4j");
        key_value.should_have_at_most_size(1);
    }

    #[test]
    fn should_be_same_size_as_other() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.should_be_same_size_as(&[1]);
    }

    #[test]
    #[should_panic]
    fn should_be_same_size_as_other_but_was_not() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.should_be_same_size_as(&[1, 2, 3]);
    }
}