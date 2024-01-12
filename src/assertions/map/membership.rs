use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use crate::matchers::map::empty::be_empty;
use crate::matchers::map::membership::{
    contain_all_key_values, contain_all_keys, contain_all_values, contain_any_of_key_values,
    contain_any_of_keys, contain_any_of_values, contain_key, contain_key_value, contain_value,
};
use crate::matchers::{Should, ShouldNot};

pub trait NoMembershipAssertion<K, V> {
    fn should_be_empty(&self) -> &Self;

    fn should_not_be_empty(&self) -> &Self;
}

pub trait KeyMembershipAssertion<K> {
    fn should_contain_key<Q>(&self, key: &Q) -> &Self
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Debug + ?Sized;

    fn should_not_contain_key<Q>(&self, key: &Q) -> &Self
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Debug + ?Sized;

    fn should_contain_all_keys<Q>(&self, keys: Vec<&Q>) -> &Self
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Debug + ?Sized;

    fn should_not_contain_all_keys<Q>(&self, keys: Vec<&Q>) -> &Self
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Debug + ?Sized;

    fn should_contain_any_of_keys<Q>(&self, keys: Vec<&Q>) -> &Self
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Debug + ?Sized;

    fn should_not_contain_any_of_keys<Q>(&self, keys: Vec<&Q>) -> &Self
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Debug + ?Sized;
}

pub trait ValueMembershipAssertion<V> {
    fn should_contain_value<S>(&self, value: &S) -> &Self
    where
        V: Eq + Borrow<S>,
        S: Debug + ?Sized + Eq;

    fn should_not_contain_value<S>(&self, value: &S) -> &Self
    where
        V: Eq + Borrow<S>,
        S: Debug + ?Sized + Eq;

    fn should_contain_all_values<S>(&self, values: Vec<&S>) -> &Self
    where
        V: Eq + Borrow<S>,
        S: Debug + ?Sized + Eq;

    fn should_not_contain_all_values<S>(&self, values: Vec<&S>) -> &Self
    where
        V: Eq + Borrow<S>,
        S: Debug + ?Sized + Eq;

    fn should_contain_any_of_values<S>(&self, values: Vec<&S>) -> &Self
    where
        V: Eq + Borrow<S>,
        S: Debug + ?Sized + Eq;

    fn should_not_contain_any_of_values<S>(&self, values: Vec<&S>) -> &Self
    where
        V: Eq + Borrow<S>,
        S: Debug + ?Sized + Eq;
}

pub trait KeyValueMembershipAssertion<K, V> {
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

    fn should_contain_all<Q, S>(&self, entries: HashMap<&Q, &S>) -> &Self
    where
        K: Borrow<Q>,
        V: Borrow<S>,
        Q: Debug + ?Sized + Hash + Eq,
        S: Debug + ?Sized + Eq;

    fn should_not_contain_all<Q, S>(&self, entries: HashMap<&Q, &S>) -> &Self
    where
        K: Borrow<Q>,
        V: Borrow<S>,
        Q: Debug + ?Sized + Hash + Eq,
        S: Debug + ?Sized + Eq;

    fn should_contain_any<Q, S>(&self, entries: HashMap<&Q, &S>) -> &Self
    where
        K: Borrow<Q>,
        V: Borrow<S>,
        Q: Debug + ?Sized + Hash + Eq,
        S: Debug + ?Sized + Eq;

    fn should_not_contain_any<Q, S>(&self, entries: HashMap<&Q, &S>) -> &Self
    where
        K: Borrow<Q>,
        V: Borrow<S>,
        Q: Debug + ?Sized + Hash + Eq,
        S: Debug + ?Sized + Eq;
}

impl<K, V> NoMembershipAssertion<K, V> for HashMap<K, V>
where
    K: Hash + Eq,
{
    fn should_be_empty(&self) -> &Self {
        self.should(&be_empty());
        self
    }

    fn should_not_be_empty(&self) -> &Self {
        self.should_not(&be_empty());
        self
    }
}

impl<K, V> KeyMembershipAssertion<K> for HashMap<K, V>
where
    K: Hash + Eq + Debug,
{
    fn should_contain_key<Q>(&self, key: &Q) -> &Self
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Debug + ?Sized,
    {
        map_keys(self).should(&contain_key(key));
        self
    }

    fn should_not_contain_key<Q>(&self, key: &Q) -> &Self
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Debug + ?Sized,
    {
        map_keys(self).should_not(&contain_key(key));
        self
    }

    fn should_contain_all_keys<Q>(&self, keys: Vec<&Q>) -> &Self
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Debug + ?Sized,
    {
        map_keys(self).should(&contain_all_keys(keys));
        self
    }

    fn should_not_contain_all_keys<Q>(&self, keys: Vec<&Q>) -> &Self
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Debug + ?Sized,
    {
        map_keys(self).should_not(&contain_all_keys(keys));
        self
    }

    fn should_contain_any_of_keys<Q>(&self, keys: Vec<&Q>) -> &Self
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Debug + ?Sized,
    {
        map_keys(self).should(&contain_any_of_keys(keys));
        self
    }

    fn should_not_contain_any_of_keys<Q>(&self, keys: Vec<&Q>) -> &Self
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Debug + ?Sized,
    {
        map_keys(self).should_not(&contain_any_of_keys(keys));
        self
    }
}

impl<K, V> ValueMembershipAssertion<V> for HashMap<K, V>
where
    K: Hash + Eq + Debug,
    V: Debug,
{
    fn should_contain_value<S>(&self, value: &S) -> &Self
    where
        V: Eq + Borrow<S>,
        S: Debug + ?Sized + Eq,
    {
        map_values(self).should(&contain_value(value));
        self
    }

    fn should_not_contain_value<S>(&self, value: &S) -> &Self
    where
        V: Eq + Borrow<S>,
        S: Debug + ?Sized + Eq,
    {
        map_values(self).should_not(&contain_value(value));
        self
    }

    fn should_contain_all_values<S>(&self, values: Vec<&S>) -> &Self
    where
        V: Eq + Borrow<S>,
        S: Debug + ?Sized + Eq,
    {
        map_values(self).should(&contain_all_values(values));
        self
    }

    fn should_not_contain_all_values<S>(&self, values: Vec<&S>) -> &Self
    where
        V: Eq + Borrow<S>,
        S: Debug + ?Sized + Eq,
    {
        map_values(self).should_not(&contain_all_values(values));
        self
    }

    fn should_contain_any_of_values<S>(&self, values: Vec<&S>) -> &Self
    where
        V: Eq + Borrow<S>,
        S: Debug + ?Sized + Eq,
    {
        map_values(self).should(&contain_any_of_values(values));
        self
    }

    fn should_not_contain_any_of_values<S>(&self, values: Vec<&S>) -> &Self
    where
        V: Eq + Borrow<S>,
        S: Debug + ?Sized + Eq,
    {
        map_values(self).should_not(&contain_any_of_values(values));
        self
    }
}

impl<K, V> KeyValueMembershipAssertion<K, V> for HashMap<K, V>
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
        map_key_value(self).should(&contain_key_value(key, value));
        self
    }

    fn should_not_contain<Q, S>(&self, key: &Q, value: &S) -> &Self
    where
        K: Borrow<Q>,
        V: Borrow<S>,
        Q: Debug + ?Sized + Hash + Eq,
        S: Debug + ?Sized + Eq,
    {
        map_key_value(self).should_not(&contain_key_value(key, value));
        self
    }

    fn should_contain_all<Q, S>(&self, entries: HashMap<&Q, &S>) -> &Self
    where
        K: Borrow<Q>,
        V: Borrow<S>,
        Q: Debug + ?Sized + Hash + Eq,
        S: Debug + ?Sized + Eq,
    {
        map_key_value(self).should(&contain_all_key_values(entries));
        self
    }

    fn should_not_contain_all<Q, S>(&self, entries: HashMap<&Q, &S>) -> &Self
    where
        K: Borrow<Q>,
        V: Borrow<S>,
        Q: Debug + ?Sized + Hash + Eq,
        S: Debug + ?Sized + Eq,
    {
        map_key_value(self).should_not(&contain_all_key_values(entries));
        self
    }

    fn should_contain_any<Q, S>(&self, entries: HashMap<&Q, &S>) -> &Self
    where
        K: Borrow<Q>,
        V: Borrow<S>,
        Q: Debug + ?Sized + Hash + Eq,
        S: Debug + ?Sized + Eq,
    {
        map_key_value(self).should(&contain_any_of_key_values(entries));
        self
    }

    fn should_not_contain_any<Q, S>(&self, entries: HashMap<&Q, &S>) -> &Self
    where
        K: Borrow<Q>,
        V: Borrow<S>,
        Q: Debug + ?Sized + Hash + Eq,
        S: Debug + ?Sized + Eq,
    {
        map_key_value(self).should_not(&contain_any_of_key_values(entries));
        self
    }
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

#[cfg(test)]
mod empty_tests {
    use std::collections::HashMap;

    use crate::assertions::map::membership::NoMembershipAssertion;

    #[test]
    fn should_be_empty() {
        let key_value: HashMap<i32, i32> = HashMap::new();
        key_value.should_be_empty();
    }

    #[test]
    #[should_panic]
    fn should_be_empty_but_was_not() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.should_be_empty();
    }

    #[test]
    fn should_not_be_empty() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.should_not_be_empty();
    }

    #[test]
    #[should_panic]
    fn should_not_be_empty_but_was() {
        let key_value: HashMap<i32, i32> = HashMap::new();
        key_value.should_not_be_empty();
    }
}

#[cfg(test)]
mod key_contains_tests {
    use std::collections::HashMap;

    use crate::assertions::map::membership::KeyMembershipAssertion;

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

    #[test]
    fn should_contain_all_keys() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.insert("java", "junit");
        key_value.should_contain_all_keys(vec!["rust", "java"]);
    }

    #[test]
    #[should_panic]
    fn should_contain_all_keys_but_it_did_not() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.should_contain_all_keys(vec!["rust", "java"]);
    }

    #[test]
    fn should_not_contain_all_keys() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.should_not_contain_all_keys(vec!["rust", "java"]);
    }

    #[test]
    #[should_panic]
    fn should_not_contain_all_keys_but_it_did() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.insert("java", "junit");
        key_value.should_not_contain_all_keys(vec!["rust", "java"]);
    }

    #[test]
    fn should_contain_any_of_keys() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.insert("java", "junit");
        key_value.should_contain_any_of_keys(vec!["rust", "scala"]);
    }

    #[test]
    #[should_panic]
    fn should_contain_any_of_keys_but_it_did_not() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.should_contain_any_of_keys(vec!["golang", "scala"]);
    }

    #[test]
    fn should_not_contain_any_of_keys() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.should_not_contain_any_of_keys(vec!["scala", "golang"]);
    }

    #[test]
    #[should_panic]
    fn should_not_contain_any_of_keys_but_it_did() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.insert("java", "junit");
        key_value.should_not_contain_any_of_keys(vec!["rust", "scala"]);
    }
}

#[cfg(test)]
mod value_contains_tests {
    use std::collections::HashMap;

    use crate::assertions::map::membership::ValueMembershipAssertion;

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

    #[test]
    fn should_contain_all_values() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.insert("java", "junit");
        key_value.should_contain_all_values(vec!["assert", "junit"]);
    }

    #[test]
    #[should_panic]
    fn should_contain_all_values_but_it_did_not() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.should_contain_all_values(vec!["java", "xunit"]);
    }

    #[test]
    fn should_not_contain_all_values() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.insert("java", "junit");
        key_value.should_not_contain_all_values(vec!["catch", "junit"]);
    }

    #[test]
    #[should_panic]
    fn should_not_contain_all_values_but_it_contained() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.should_not_contain_all_values(vec!["assert", "assert"]);
    }

    #[test]
    fn should_contain_any_of_values() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.insert("java", "junit");
        key_value.should_contain_any_of_values(vec!["assert", "xunit"]);
    }

    #[test]
    #[should_panic]
    fn should_contain_any_of_values_but_it_did_not() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.should_contain_any_of_values(vec!["catch", "xunit"]);
    }

    #[test]
    fn should_not_contain_any_of_values() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.insert("java", "junit");
        key_value.should_not_contain_any_of_values(vec!["catch", "xunit"]);
    }

    #[test]
    #[should_panic]
    fn should_not_contain_any_of_values_but_it_contained() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.should_not_contain_any_of_values(vec!["assert", "junit"]);
    }
}

#[cfg(test)]
mod key_value_contains_tests {
    use std::collections::HashMap;

    use crate::assertions::map::membership::KeyValueMembershipAssertion;

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

    #[test]
    fn should_contain_all_key_values() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.insert("java", "junit");

        let mut to_contain = HashMap::new();
        to_contain.insert("rust", "assert");
        to_contain.insert("java", "junit");

        key_value.should_contain_all(to_contain);
    }

    #[test]
    #[should_panic]
    fn should_contain_all_key_values_but_it_did_not() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.insert("java", "junit");

        let mut to_contain = HashMap::new();
        to_contain.insert("rust", "assert");
        to_contain.insert("java", "xunit");

        key_value.should_contain_all(to_contain);
    }

    #[test]
    fn should_not_contain_all_key_values() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.insert("java", "junit");

        let mut to_contain = HashMap::new();
        to_contain.insert("rust", "assert");
        to_contain.insert("java", "xunit");

        key_value.should_not_contain_all(to_contain);
    }

    #[test]
    #[should_panic]
    fn should_not_contain_all_key_values_but_it_did() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.insert("java", "junit");

        let mut to_contain = HashMap::new();
        to_contain.insert("rust", "assert");
        to_contain.insert("java", "junit");

        key_value.should_not_contain_all(to_contain);
    }

    #[test]
    fn should_contain_any_key_values() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.insert("java", "junit");

        let mut to_contain = HashMap::new();
        to_contain.insert("rust", "assert");
        to_contain.insert("golang", "gotest");

        key_value.should_contain_any(to_contain);
    }

    #[test]
    #[should_panic]
    fn should_contain_any_key_values_but_it_did_not() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.insert("java", "junit");

        let mut to_contain = HashMap::new();
        to_contain.insert("rust", "clearcheck");
        to_contain.insert("java", "xunit");

        key_value.should_contain_any(to_contain);
    }

    #[test]
    fn should_not_contain_any_key_values() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.insert("java", "junit");

        let mut to_contain = HashMap::new();
        to_contain.insert("rust", "clearcheck");
        to_contain.insert("java", "xunit");

        key_value.should_not_contain_any(to_contain);
    }

    #[test]
    #[should_panic]
    fn should_not_contain_any_key_values_but_it_did() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.insert("java", "junit");

        let mut to_contain = HashMap::new();
        to_contain.insert("rust", "assert");
        to_contain.insert("java", "junit");

        key_value.should_not_contain_any(to_contain);
    }
}
