use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Range, RangeInclusive};

use crate::assertions::collection::size::Size;
use crate::matchers::range::{be_in_exclusive_range, be_in_inclusive_range};
use crate::matchers::{Should, ShouldNot};
use crate::panicking::{assert_failed_binary, AssertKind};

impl<K, V> Size for HashMap<K, V>
where
    K: Hash + Eq + PartialEq + Debug,
    V: Debug,
{
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

    fn should_have_size_in_inclusive_range(&self, range: RangeInclusive<usize>) -> &Self {
        let in_range = self.len().should(&be_in_inclusive_range(&range));
        if !in_range {
            assert_failed_binary(AssertKind::InRangeSize, self, &range);
        }
        self
    }

    fn should_not_have_size_in_inclusive_range(&self, range: RangeInclusive<usize>) -> &Self {
        let not_in_range = self.len().should_not(&be_in_inclusive_range(&range));
        if !not_in_range {
            assert_failed_binary(AssertKind::NotInRangeSize, self, &range);
        }
        self
    }

    fn should_have_size_in_exclusive_range(&self, range: Range<usize>) -> &Self {
        let in_range = self.len().should(&be_in_exclusive_range(&range));
        if !in_range {
            assert_failed_binary(AssertKind::InRangeSize, self, &range);
        }
        self
    }

    fn should_not_have_size_in_exclusive_range(&self, range: Range<usize>) -> &Self {
        let not_in_range = self.len().should_not(&be_in_exclusive_range(&range));
        if !not_in_range {
            assert_failed_binary(AssertKind::NotInRangeSize, self, &range);
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::assertions::collection::size::Size;

    #[test]
    fn should_have_size_as_1() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.should_have_size(1);
    }

    #[test]
    #[should_panic]
    fn should_have_size_3_but_was_not() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.should_have_size(3);
    }

    #[test]
    fn should_not_have_size_3() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.should_not_have_size(3);
    }

    #[test]
    #[should_panic]
    fn should_not_have_size_1_but_was() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.should_not_have_size(1);
    }

    #[test]
    fn should_have_at_least_size_1() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.should_have_at_least_size(1);
    }

    #[test]
    #[should_panic]
    fn should_have_at_least_size_2_but_was_not() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.should_have_at_least_size(2);
    }

    #[test]
    fn should_have_at_most_size_1() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.should_have_at_most_size(1);
    }

    #[test]
    #[should_panic]
    fn should_have_at_most_size_1_but_was_not() {
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

    #[test]
    fn should_have_size_in_the_inclusive_range() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.should_have_size_in_inclusive_range(1..=8);
    }

    #[test]
    #[should_panic]
    fn should_have_size_in_the_inclusive_range_but_was_not() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.should_have_size_in_inclusive_range(3..=4);
    }

    #[test]
    fn should_not_have_size_in_the_inclusive_range() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.should_not_have_size_in_inclusive_range(3..=4);
    }

    #[test]
    #[should_panic]
    fn should_not_have_size_in_the_inclusive_range_but_was() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.should_not_have_size_in_inclusive_range(1..=2);
    }

    #[test]
    fn should_have_size_in_the_exclusive_range() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.should_have_size_in_exclusive_range(1..3);
    }

    #[test]
    #[should_panic]
    fn should_have_size_in_the_range_but_was_not() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.should_have_size_in_exclusive_range(3..8);
    }

    #[test]
    fn should_not_have_size_in_the_exclusive_range() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.should_not_have_size_in_exclusive_range(3..4);
    }

    #[test]
    #[should_panic]
    fn should_not_have_size_in_the_exclusive_range_but_was() {
        let mut key_value = HashMap::new();
        key_value.insert("rust", "assert");
        key_value.should_not_have_size_in_exclusive_range(1..9);
    }
}
