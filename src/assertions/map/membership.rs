use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use crate::matchers::map::membership::{contain_key, contain_key_value, contain_value};
use crate::matchers::{Should, ShouldNot};

pub trait KeyMembership<K, V> {
    fn should_contain_key<Q>(&self, key: &Q) -> &Self
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Debug + ?Sized;

    fn should_not_contain_key<Q>(&self, key: &Q) -> &Self
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Debug + ?Sized;
}

pub trait ValueMembership<K, V> {
    fn should_contain_value<S>(&self, value: &S) -> &Self
    where
        V: Eq + Borrow<S>,
        S: Debug + ?Sized + Eq;

    fn should_not_contain_value<S>(&self, value: &S) -> &Self
    where
        V: Eq + Borrow<S>,
        S: Debug + ?Sized + Eq;
}

pub trait KeyValueMembership<K, V> {
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

fn map_keys<K, V, Q>(collection: &HashMap<K, V>) -> HashMap<&Q, &V>
where
    K: Hash + Eq,
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized,
{
    collection
        .iter()
        .map(|key_value| (key_value.0.borrow(), key_value.1))
        .collect::<HashMap<_, _>>()
}

fn map_values<K, V, S>(collection: &HashMap<K, V>) -> HashMap<&K, &S>
where
    K: Hash + Eq,
    V: Borrow<S>,
    S: Eq + ?Sized,
{
    collection
        .iter()
        .map(|key_value| (key_value.0, key_value.1.borrow()))
        .collect::<HashMap<_, _>>()
}

fn map_key_value<K, V, Q, S>(collection: &HashMap<K, V>) -> HashMap<&Q, &S>
where
    K: Hash + Eq,
    K: Borrow<Q>,
    V: Borrow<S>,
    Q: Hash + Eq + ?Sized,
    S: Eq + ?Sized,
{
    collection
        .iter()
        .map(|key_value| (key_value.0.borrow(), key_value.1.borrow()))
        .collect::<HashMap<_, _>>()
}

impl<K, V> KeyMembership<K, V> for HashMap<K, V>
where
    K: Hash + Eq + Debug,
{
    fn should_contain_key<Q>(&self, key: &Q) -> &Self
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Debug + ?Sized,
    {
        map_keys(&self).should(&contain_key(&key));
        self
    }

    fn should_not_contain_key<Q>(&self, key: &Q) -> &Self
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Debug + ?Sized,
    {
        map_keys(&self).should_not(&contain_key(&key));
        self
    }
}

impl<K, V> ValueMembership<K, V> for HashMap<K, V>
where
    K: Hash + Eq + Debug,
    V: Debug,
{
    fn should_contain_value<S>(&self, value: &S) -> &Self
    where
        V: Eq + Borrow<S>,
        S: Debug + ?Sized + Eq,
    {
        map_values(&self).should(&contain_value(&value));
        self
    }

    fn should_not_contain_value<S>(&self, value: &S) -> &Self
    where
        V: Eq + Borrow<S>,
        S: Debug + ?Sized + Eq,
    {
        map_values(&self).should_not(&contain_value(&value));
        self
    }
}

impl<K, V> KeyValueMembership<K, V> for HashMap<K, V>
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
        map_key_value(&self).should(&contain_key_value(&key, &value));
        self
    }

    fn should_not_contain<Q, S>(&self, key: &Q, value: &S) -> &Self
    where
        K: Borrow<Q>,
        V: Borrow<S>,
        Q: Debug + ?Sized + Hash + Eq,
        S: Debug + ?Sized + Eq,
    {
        map_key_value(&self).should_not(&contain_key_value(&key, &value));
        self
    }
}

#[cfg(test)]
mod key_contains_tests {
    use std::collections::HashMap;

    use crate::assertions::map::membership::KeyMembership;

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

    use crate::assertions::map::membership::ValueMembership;

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

    use crate::assertions::map::membership::KeyValueMembership;

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
