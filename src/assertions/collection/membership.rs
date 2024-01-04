use std::borrow::Borrow;
use std::fmt::Debug;

use crate::matchers::collection::membership::contain;
use crate::matchers::length::have_zero_length;
use crate::matchers::{Should, ShouldNot};
use crate::panicking::{assert_failed_binary, assert_failed_unary, AssertKind};

pub trait Membership<T>
where
    T: Eq + Debug,
{
    fn should_contain<Q>(&self, element: &Q) -> &Self
    where
        T: Borrow<Q>,
        Q: Eq + Debug + ?Sized;
    fn should_not_contain<Q>(&self, element: &Q) -> &Self
    where
        T: Borrow<Q>,
        Q: Eq + Debug + ?Sized;
    fn should_be_empty(&self) -> &Self;
    fn should_not_be_empty(&self) -> &Self;
}

impl<T> Membership<T> for Vec<T>
where
    T: Debug,
    T: Eq,
{
    fn should_contain<Q>(&self, element: &Q) -> &Self
    where
        T: Borrow<Q>,
        Q: Eq + Debug + ?Sized,
    {
        (self as &[T]).should_contain(element);
        self
    }

    fn should_not_contain<Q>(&self, element: &Q) -> &Self
    where
        T: Borrow<Q>,
        Q: Eq + Debug + ?Sized,
    {
        (self as &[T]).should_not_contain(element);
        self
    }

    fn should_be_empty(&self) -> &Self {
        (self as &[T]).should_be_empty();
        self
    }

    fn should_not_be_empty(&self) -> &Self {
        (self as &[T]).should_not_be_empty();
        self
    }
}

impl<T, const N: usize> Membership<T> for [T; N]
where
    T: Debug,
    T: Eq,
{
    fn should_contain<Q>(&self, element: &Q) -> &Self
    where
        T: Borrow<Q>,
        Q: Eq + Debug + ?Sized,
    {
        (self as &[T]).should_contain(element);
        self
    }

    fn should_not_contain<Q>(&self, element: &Q) -> &Self
    where
        T: Borrow<Q>,
        Q: Eq + Debug + ?Sized,
    {
        (self as &[T]).should_not_contain(element);
        self
    }

    fn should_be_empty(&self) -> &Self {
        (self as &[T]).should_be_empty();
        self
    }

    fn should_not_be_empty(&self) -> &Self {
        (self as &[T]).should_not_be_empty();
        self
    }
}

impl<T> Membership<T> for [T]
where
    T: Debug,
    T: Eq,
{
    fn should_contain<Q>(&self, element: &Q) -> &Self
    where
        T: Borrow<Q>,
        Q: Eq + Debug + ?Sized,
    {
        let mapped: Vec<_> = self.iter().map(|source| source.borrow()).collect();
        if !mapped.should(&contain(&element)) {
            assert_failed_binary(AssertKind::Contains, self, element);
        }
        self
    }

    fn should_not_contain<Q>(&self, element: &Q) -> &Self
    where
        T: Borrow<Q>,
        Q: Eq + Debug + ?Sized,
    {
        let mapped: Vec<_> = self.iter().map(|source| source.borrow()).collect();
        if !mapped.should_not(&contain(&element)) {
            assert_failed_binary(AssertKind::NotContains, &self, element);
        }
        self
    }

    fn should_be_empty(&self) -> &Self {
        if !self.should(&have_zero_length()) {
            assert_failed_unary(AssertKind::Empty, &self)
        }
        self
    }

    fn should_not_be_empty(&self) -> &Self {
        if !self.should_not(&have_zero_length()) {
            assert_failed_unary(AssertKind::NotEmpty, &self)
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::assertions::collection::membership::Membership;

    #[test]
    fn should_contain() {
        let collection = vec!["junit", "assert4j", "catch2"];
        collection.should_contain("assert4j");
    }

    #[test]
    #[should_panic]
    fn should_contain_but_was_not_contained() {
        let collection = vec!["junit", "assert4j", "catch2"];
        collection.should_contain("catch");
    }

    #[test]
    fn should_not_contain() {
        let collection = vec!["junit", "assert4j", "catch2"];
        collection.should_not_contain("catch");
    }

    #[test]
    #[should_panic]
    fn should_not_contain_but_was_contained() {
        let collection = vec!["junit", "assert4j", "catch2"];
        collection.should_not_contain("catch2");
    }

    #[test]
    #[should_panic]
    fn should_be_empty_but_was_not() {
        let collection = vec!["junit", "testify"];
        collection.should_be_empty();
    }

    #[test]
    fn should_be_empty() {
        let collection: Vec<i32> = vec![];
        collection.should_be_empty();
    }

    #[test]
    #[should_panic]
    fn should_not_be_empty_but_was() {
        let collection: Vec<i32> = vec![];
        collection.should_not_be_empty();
    }

    #[test]
    fn should_not_be_empty() {
        let collection = vec!["junit", "testify"];
        collection.should_not_be_empty();
    }
}
