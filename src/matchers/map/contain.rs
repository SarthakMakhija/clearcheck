use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;

use crate::panicking::{assert_failed_binary, AssertKind};

trait Contain<K, V> {
    fn should_contain_key<Q>(&self, key: &Q) -> &Self
        where K: Borrow<Q>,
              Q: Hash + Eq + std::fmt::Debug;

    fn should_not_contain_key<Q>(&self, key: &Q) -> &Self
        where K: Borrow<Q>,
              Q: Hash + Eq + std::fmt::Debug;
}


impl<K, V> Contain<K, V> for HashMap<K, V>
    where K: Hash + Eq + PartialEq + std::fmt::Debug {
    fn should_contain_key<Q>(&self, key: &Q) -> &Self
        where K: Borrow<Q>,
              Q: Hash + Eq + std::fmt::Debug {
        let contains = self.contains_key(key);
        if !contains {
            assert_failed_binary(AssertKind::Contains, &self.keys(), key);
        }
        self
    }

    fn should_not_contain_key<Q>(&self, key: &Q) -> &Self
        where K: Borrow<Q>,
              Q: Hash + Eq + std::fmt::Debug {
        let contains = self.contains_key(key);
        if contains {
            assert_failed_binary(AssertKind::NotContains, &self.keys(), key);
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::matchers::map::contain::Contain;

    #[test]
    fn should_contain_key() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.should_contain_key(&"rust");
    }

    #[test]
    #[should_panic]
    fn should_contain_key_but_it_did_not() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.should_contain_key(&"java");
    }

    #[test]
    fn should_not_contain_key() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.should_not_contain_key(&"junit");
    }

    #[test]
    #[should_panic]
    fn should_not_contain_key_but_it_contained() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.should_not_contain_key(&"rust");
    }
}