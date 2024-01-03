use std::borrow::Borrow;
use std::fmt::Debug;

use crate::panicking::{assert_failed_binary, AssertKind};

pub trait Contains<T>
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
}

impl<T> Contains<T> for Vec<T>
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
}

impl<T> Contains<T> for [T]
where
    T: Debug,
    T: Eq,
{
    fn should_contain<Q>(&self, element: &Q) -> &Self
    where
        T: Borrow<Q>,
        Q: Eq + Debug + ?Sized,
    {
        let contains = self.iter().any(|source| source.borrow() == element);
        if !contains {
            assert_failed_binary(AssertKind::Contains, self, element);
        }
        self
    }

    fn should_not_contain<Q>(&self, element: &Q) -> &Self
    where
        T: Borrow<Q>,
        Q: Eq + Debug + ?Sized,
    {
        let contains = self.iter().any(|source| source.borrow() == element);
        if contains {
            assert_failed_binary(AssertKind::NotContains, &self, element);
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::matchers::collection::contain::Contains;

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
}
