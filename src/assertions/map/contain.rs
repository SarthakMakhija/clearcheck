use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use crate::panicking::{assert_failed_binary, AssertKind};

pub trait KeyContains<K, V> {
    fn should_contain_key<Q>(&self, key: &Q) -> &Self
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Debug + ?Sized;

    fn should_not_contain_key<Q>(&self, key: &Q) -> &Self
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Debug + ?Sized;
}

pub trait ValueContains<K, V> {
    fn should_contain_value<S>(&self, value: &S) -> &Self
    where
        V: Eq + Borrow<S>,
        S: Debug + ?Sized + Eq;

    fn should_not_contain_value<S>(&self, value: &S) -> &Self
    where
        V: Eq + Borrow<S>,
        S: Debug + ?Sized + Eq;
}

pub trait KeyValueContains<K, V> {
    fn should_contain<Q, S>(&self, key: &Q, value: &S) -> &Self
    where
        K: Borrow<Q>,
        V: Borrow<S>,
        Q: Debug + ?Sized + Hash + Eq,
        S: Debug + ?Sized + Eq;

    fn should_not_contain<Q, S>(&self, key: &Q, value: &S) -> &Self
    where
        K: Borrow<Q>,
        V: Borrow<S>,
        Q: Debug + ?Sized + Hash + Eq,
        S: Debug + ?Sized + Eq;
}

impl<K, V> KeyContains<K, V> for HashMap<K, V>
where
    K: Hash + Eq + Debug,
{
    fn should_contain_key<Q>(&self, key: &Q) -> &Self
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Debug + ?Sized,
    {
        let contains = self.contains_key(key);
        if !contains {
            assert_failed_binary(AssertKind::ContainsKey, &self.keys(), key);
        }
        self
    }

    fn should_not_contain_key<Q>(&self, key: &Q) -> &Self
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Debug + ?Sized,
    {
        let contains = self.contains_key(key);
        if contains {
            assert_failed_binary(AssertKind::NotContainsKey, &self.keys(), key);
        }
        self
    }
}

impl<K, V> ValueContains<K, V> for HashMap<K, V>
where
    K: Hash + Eq + Debug,
    V: Debug,
{
    fn should_contain_value<S>(&self, value: &S) -> &Self
    where
        V: Eq + Borrow<S>,
        S: Debug + ?Sized + Eq,
    {
        let contains = self.values().any(|source| source.borrow() == value);
        if !contains {
            assert_failed_binary(AssertKind::ContainsValue, &self.values(), value);
        }
        self
    }

    fn should_not_contain_value<S>(&self, value: &S) -> &Self
    where
        V: Eq + Borrow<S>,
        S: Debug + ?Sized + Eq,
    {
        let contains = self.values().any(|source| source.borrow() == value);
        if contains {
            assert_failed_binary(AssertKind::NotContainsValue, &self.values(), value);
        }
        self
    }
}

impl<K, V> KeyValueContains<K, V> for HashMap<K, V>
where
    K: Hash + Eq + Debug,
    V: Debug,
{
    fn should_contain<Q, S>(&self, key: &Q, value: &S) -> &Self
    where
        K: Borrow<Q>,
        V: Borrow<S>,
        Q: Debug + ?Sized + Hash + Eq,
        S: Debug + ?Sized + Eq,
    {
        match self.get(key) {
            None => {
                assert_failed_binary(AssertKind::ContainsKey, &self.keys(), key);
            }
            Some(existing) if existing.borrow() != value => {
                assert_failed_binary(AssertKind::ContainsValue, &self.values(), value);
            }
            _ => {}
        }
        self
    }

    fn should_not_contain<Q, S>(&self, key: &Q, value: &S) -> &Self
    where
        K: Borrow<Q>,
        V: Borrow<S>,
        Q: Debug + ?Sized + Hash + Eq,
        S: Debug + ?Sized + Eq,
    {
        match self.get(key) {
            Some(existing) if existing.borrow() == value => {
                assert_failed_binary(AssertKind::NotContainsValue, &self.values(), value);
            }
            _ => {}
        }
        self
    }
}

#[cfg(test)]
mod key_contains_tests {
    use std::collections::HashMap;

    use crate::assertions::map::contain::KeyContains;

    #[test]
    fn should_contain_key() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.should_contain_key("rust");
    }

    #[test]
    #[should_panic]
    fn should_contain_key_but_it_did_not() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.should_contain_key("java");
    }

    #[test]
    fn should_not_contain_key() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.should_not_contain_key("junit");
    }

    #[test]
    #[should_panic]
    fn should_not_contain_key_but_it_contained() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.should_not_contain_key("rust");
    }
}

#[cfg(test)]
mod value_contains_tests {
    use std::collections::HashMap;

    use crate::assertions::map::contain::ValueContains;

    #[test]
    fn should_contain_value() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.should_contain_value("assert");
    }

    #[test]
    #[should_panic]
    fn should_contain_value_but_it_did_not() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.should_contain_value("java");
    }

    #[test]
    fn should_not_contain_value() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.should_not_contain_value("catch");
    }

    #[test]
    #[should_panic]
    fn should_not_contain_value_but_it_contained() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.should_not_contain_value("assert");
    }
}

#[cfg(test)]
mod key_value_contains_tests {
    use std::collections::HashMap;

    use crate::assertions::map::contain::KeyValueContains;

    #[test]
    fn should_contain_key_value() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.should_contain("rust", "assert");
    }

    #[test]
    #[should_panic]
    fn should_contain_key_value_but_it_did_not_1() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.should_contain("rust", "catch");
    }

    #[test]
    #[should_panic]
    fn should_contain_key_value_but_it_did_not_2() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.should_contain("java", "catch");
    }

    #[test]
    fn should_not_contain_key_value() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.should_not_contain("rust", "catch");
    }

    #[test]
    #[should_panic]
    fn should_not_contain_key_value_but_it_did() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.should_not_contain("rust", "assert");
    }
}