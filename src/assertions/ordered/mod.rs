use std::borrow::Borrow;
use std::fmt::Debug;
use std::ops::{Range, RangeInclusive};

use crate::matchers::ordered::{
    be_greater_than, be_greater_than_equal_to, be_less_than, be_less_than_equal_to,
};
use crate::matchers::range::{be_in_exclusive_range, be_in_inclusive_range};
use crate::matchers::{Should, ShouldNot};

pub trait OrderedAssertion<T: PartialOrd> {
    fn should_be_greater_than<Q>(&self, other: &Q) -> &Self
    where
        T: Borrow<Q>,
        Q: PartialOrd + Debug + ?Sized;

    fn should_be_greater_than_equal_to<Q>(&self, other: &Q) -> &Self
    where
        T: Borrow<Q>,
        Q: PartialOrd + Debug + ?Sized;

    fn should_be_less_than<Q>(&self, other: &Q) -> &Self
    where
        T: Borrow<Q>,
        Q: PartialOrd + Debug + ?Sized;

    fn should_be_less_than_equal_to<Q>(&self, other: &Q) -> &Self
    where
        T: Borrow<Q>,
        Q: PartialOrd + Debug + ?Sized;

    fn should_not_be_greater_than<Q>(&self, other: &Q) -> &Self
    where
        T: Borrow<Q>,
        Q: PartialOrd + Debug + ?Sized;

    fn should_not_be_greater_than_equal_to<Q>(&self, other: &Q) -> &Self
    where
        T: Borrow<Q>,
        Q: PartialOrd + Debug + ?Sized;

    fn should_not_be_less_than<Q>(&self, other: &Q) -> &Self
    where
        T: Borrow<Q>,
        Q: PartialOrd + Debug + ?Sized;

    fn should_not_be_less_than_equal_to<Q>(&self, other: &Q) -> &Self
    where
        T: Borrow<Q>,
        Q: PartialOrd + Debug + ?Sized;

    fn should_be_in_inclusive_range(&self, range: RangeInclusive<T>) -> &Self;

    fn should_not_be_in_inclusive_range(&self, range: RangeInclusive<T>) -> &Self;

    fn should_be_in_exclusive_range(&self, range: Range<T>) -> &Self;

    fn should_not_be_in_exclusive_range(&self, range: Range<T>) -> &Self;
}

impl<T: PartialOrd + Debug> OrderedAssertion<T> for T {
    fn should_be_greater_than<Q>(&self, other: &Q) -> &Self
    where
        T: Borrow<Q>,
        Q: PartialOrd + Debug + ?Sized,
    {
        self.borrow().should(&be_greater_than(&other));
        self
    }

    fn should_be_greater_than_equal_to<Q>(&self, other: &Q) -> &Self
    where
        T: Borrow<Q>,
        Q: PartialOrd + Debug + ?Sized,
    {
        self.borrow().should(&be_greater_than_equal_to(&other));
        self
    }

    fn should_be_less_than<Q>(&self, other: &Q) -> &Self
    where
        T: Borrow<Q>,
        Q: PartialOrd + Debug + ?Sized,
    {
        self.borrow().should(&be_less_than(&other));
        self
    }

    fn should_be_less_than_equal_to<Q>(&self, other: &Q) -> &Self
    where
        T: Borrow<Q>,
        Q: PartialOrd + Debug + ?Sized,
    {
        self.borrow().should(&be_less_than_equal_to(&other));
        self
    }

    fn should_not_be_greater_than<Q>(&self, other: &Q) -> &Self
    where
        T: Borrow<Q>,
        Q: PartialOrd + Debug + ?Sized,
    {
        self.borrow().should_not(&be_greater_than(&other));
        self
    }

    fn should_not_be_greater_than_equal_to<Q>(&self, other: &Q) -> &Self
    where
        T: Borrow<Q>,
        Q: PartialOrd + Debug + ?Sized,
    {
        self.borrow().should_not(&be_greater_than_equal_to(&other));
        self
    }

    fn should_not_be_less_than<Q>(&self, other: &Q) -> &Self
    where
        T: Borrow<Q>,
        Q: PartialOrd + Debug + ?Sized,
    {
        self.borrow().should_not(&be_less_than(&other));
        self
    }

    fn should_not_be_less_than_equal_to<Q>(&self, other: &Q) -> &Self
    where
        T: Borrow<Q>,
        Q: PartialOrd + Debug + ?Sized,
    {
        self.borrow().should_not(&be_less_than_equal_to(&other));
        self
    }

    fn should_be_in_inclusive_range(&self, range: RangeInclusive<T>) -> &Self {
        self.should(&be_in_inclusive_range(&range));
        self
    }

    fn should_not_be_in_inclusive_range(&self, range: RangeInclusive<T>) -> &Self {
        self.should_not(&be_in_inclusive_range(&range));
        self
    }

    fn should_be_in_exclusive_range(&self, range: Range<T>) -> &Self {
        self.should(&be_in_exclusive_range(&range));
        self
    }

    fn should_not_be_in_exclusive_range(&self, range: Range<T>) -> &Self {
        self.should_not(&be_in_exclusive_range(&range));
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::assertions::ordered::OrderedAssertion;

    #[test]
    fn should_be_greater_than() {
        let value = 12.5;
        value.should_be_greater_than(&10.98);
    }

    #[test]
    fn should_be_greater_than_equal_to() {
        let value = 12.59;
        value.should_be_greater_than_equal_to(&12.59);
    }

    #[test]
    fn should_be_less_than() {
        let value = 6.98;
        value.should_be_less_than(&9.98);
    }

    #[test]
    fn should_be_less_than_equal_to() {
        let value = 12.59;
        value.should_be_less_than_equal_to(&12.59);
    }

    #[test]
    fn should_not_be_greater_than() {
        let value = 12.5;
        value.should_not_be_greater_than(&14.98);
    }

    #[test]
    fn should_not_be_greater_than_equal_to() {
        let value = 12.59;
        value.should_not_be_greater_than_equal_to(&13.59);
    }

    #[test]
    fn should_not_be_less_than() {
        let value = 9.98;
        value.should_not_be_less_than(&8.98);
    }

    #[test]
    fn should_not_be_less_than_equal_to() {
        let value = 13.59;
        value.should_not_be_less_than_equal_to(&12.59);
    }

    #[test]
    fn should_be_in_inclusive_range() {
        let value = 9.98;
        value.should_be_in_inclusive_range(8.90..=9.99);
    }

    #[test]
    fn should_not_be_in_inclusive_range() {
        let value = 9.98;
        value.should_not_be_in_inclusive_range(8.90..=9.10);
    }

    #[test]
    fn should_be_in_exclusive_range() {
        let value = 9.98;
        value.should_be_in_exclusive_range(8.90..9.99);
    }

    #[test]
    fn should_not_be_in_exclusive_range() {
        let value = 9.98;
        value.should_not_be_in_exclusive_range(8.90..9.10);
    }

    #[test]
    #[should_panic]
    fn should_be_greater_than_but_was_not() {
        let value = 1.1;
        value.should_be_greater_than(&10.98);
    }

    #[test]
    #[should_panic]
    fn should_be_greater_than_equal_to_but_was_not() {
        let value = 1.59;
        value.should_be_greater_than_equal_to(&12.59);
    }

    #[test]
    #[should_panic]
    fn should_be_less_than_but_was_not() {
        let value = 16.98;
        value.should_be_less_than(&9.98);
    }

    #[test]
    #[should_panic]
    fn should_be_less_than_equal_to_but_was_not() {
        let value = 18.59;
        value.should_be_less_than_equal_to(&12.59);
    }

    #[test]
    #[should_panic]
    fn should_not_be_greater_than_but_was() {
        let value = 15.5;
        value.should_not_be_greater_than(&14.98);
    }

    #[test]
    #[should_panic]
    fn should_not_be_greater_than_equal_to_but_was() {
        let value = 14.59;
        value.should_not_be_greater_than_equal_to(&13.59);
    }

    #[test]
    #[should_panic]
    fn should_not_be_less_than_but_was() {
        let value = 1.98;
        value.should_not_be_less_than(&8.98);
    }

    #[test]
    #[should_panic]
    fn should_not_be_less_than_equal_to_but_was() {
        let value = 11.59;
        value.should_not_be_less_than_equal_to(&12.59);
    }

    #[test]
    #[should_panic]
    fn should_be_in_inclusive_range_but_was_not() {
        let value = 9.98;
        value.should_be_in_inclusive_range(8.90..=9.90);
    }

    #[test]
    #[should_panic]
    fn should_not_be_in_inclusive_range_but_was() {
        let value = 9.98;
        value.should_not_be_in_inclusive_range(8.90..=9.99);
    }

    #[test]
    #[should_panic]
    fn should_be_in_exclusive_range_but_was_not() {
        let value = 9.98;
        value.should_be_in_exclusive_range(8.90..9.90);
    }

    #[test]
    #[should_panic]
    fn should_not_be_in_exclusive_range_but_was_not() {
        let value = 9.98;
        value.should_not_be_in_exclusive_range(8.90..9.99);
    }
}

#[cfg(test)]
mod string_tests {
    use crate::assertions::ordered::OrderedAssertion;

    #[test]
    fn should_be_less_than() {
        let name = "junit";
        name.should_be_less_than("scalatest");
    }

    #[test]
    fn should_not_be_less_than() {
        let name = "junit";
        name.should_not_be_less_than("assert4rs");
    }

    #[test]
    fn should_be_in_inclusive_range() {
        let name = "junit";
        name.should_be_in_inclusive_range("assert4rs"..="scalatest");
    }

    #[test]
    fn should_be_in_exclusive_range() {
        let name = "junit";
        name.should_be_in_exclusive_range("assert4rs".."scalatest");
    }
}
