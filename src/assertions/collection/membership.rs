use std::borrow::Borrow;
use std::fmt::Debug;

use crate::matchers::{Should, ShouldNot};
use crate::matchers::collection::empty::be_empty;
use crate::matchers::collection::membership::{contain, contain_all, contain_any};

/// MembershipAssertion enables assertions about the presence or the absence of elements in a collection.
pub trait MembershipAssertion<T>
where
    T: Eq
{
    /// - Asserts that the collection contains the given element.
    /// - Supports flexible comparison through the `Borrow<Q>` trait bound.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::collection::membership::MembershipAssertion;
    ///
    /// let collection = vec!["junit", "assert4j", "clearcheck"];
    /// collection.should_contain("clearcheck");
    /// ```
    fn should_contain<Q>(&self, element: &Q) -> &Self
    where
        T: Borrow<Q>,
        Q: Eq + Debug + ?Sized;

    /// - Asserts that the collection does not contain the given element.
    /// - Supports flexible comparison through the `Borrow<Q>` trait bound.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::collection::membership::MembershipAssertion;
    ///
    /// let collection = vec!["junit", "assert4j", "clearcheck"];
    /// collection.should_not_contain("assert");
    /// ```
    fn should_not_contain<Q>(&self, element: &Q) -> &Self
    where
        T: Borrow<Q>,
        Q: Eq + Debug + ?Sized;

    /// - Asserts that the collection contains all the given elements.
    /// - Supports flexible comparison through the `Borrow<Q>` trait bound.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::collection::membership::MembershipAssertion;
    ///
    /// let collection = vec!["junit", "assert4j", "clearcheck"];
    /// collection.should_contain_all(vec!["clearcheck", "assert4j"]);
    /// ```
    fn should_contain_all<Q>(&self, elements: Vec<&Q>) -> &Self
    where
        T: Borrow<Q>,
        Q: Eq + Debug + ?Sized;

    /// - Asserts that the collection does not contain all the given elements.
    /// - Supports flexible comparison through the `Borrow<Q>` trait bound.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::collection::membership::MembershipAssertion;
    ///
    /// let collection = vec!["junit", "assert4j", "clearcheck"];
    /// collection.should_not_contain_all(vec!["clearcheck", "assert"]);
    /// ```
    fn should_not_contain_all<Q>(&self, elements: Vec<&Q>) -> &Self
    where
        T: Borrow<Q>,
        Q: Eq + Debug + ?Sized;

    /// - Asserts that the collection contains any of the given elements.
    /// - Supports flexible comparison through the `Borrow<Q>` trait bound.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::collection::membership::MembershipAssertion;
    ///
    /// let collection = vec!["junit", "assert4j", "clearcheck"];
    /// collection.should_contain_any(vec!["clearcheck", "assert", "gotest"]);
    /// ```
    fn should_contain_any<Q>(&self, elements: Vec<&Q>) -> &Self
    where
        T: Borrow<Q>,
        Q: Eq + Debug + ?Sized;

    /// - Asserts that the collection does not contain any of the given elements.
    /// - Supports flexible comparison through the `Borrow<Q>` trait bound.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::collection::membership::MembershipAssertion;
    ///
    /// let collection = vec!["junit", "assert4j", "clearcheck"];
    /// collection.should_not_contain_any(vec!["catch2", "assert"]);
    /// ```
    fn should_not_contain_any<Q>(&self, elements: Vec<&Q>) -> &Self
    where
        T: Borrow<Q>,
        Q: Eq + Debug + ?Sized;

    /// - Asserts that the collection is empty.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::collection::membership::MembershipAssertion;
    ///
    /// let collection: Vec<&str> = vec![];
    /// collection.should_be_empty();
    /// ```
    fn should_be_empty(&self) -> &Self;

    /// - Asserts that the collection is not empty.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::collection::membership::MembershipAssertion;
    ///
    /// let collection = vec!["junit", "assert4j", "clearcheck"];
    /// collection.should_not_be_empty();
    /// ```
    fn should_not_be_empty(&self) -> &Self;
}

impl<T> MembershipAssertion<T> for Vec<T>
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

    fn should_contain_all<Q>(&self, elements: Vec<&Q>) -> &Self
    where
        T: Borrow<Q>,
        Q: Eq + Debug + ?Sized,
    {
        (self as &[T]).should_contain_all(elements);
        self
    }

    fn should_not_contain_all<Q>(&self, elements: Vec<&Q>) -> &Self
    where
        T: Borrow<Q>,
        Q: Eq + Debug + ?Sized,
    {
        (self as &[T]).should_not_contain_all(elements);
        self
    }

    fn should_contain_any<Q>(&self, elements: Vec<&Q>) -> &Self
    where
        T: Borrow<Q>,
        Q: Eq + Debug + ?Sized,
    {
        (self as &[T]).should_contain_any(elements);
        self
    }

    fn should_not_contain_any<Q>(&self, elements: Vec<&Q>) -> &Self
    where
        T: Borrow<Q>,
        Q: Eq + Debug + ?Sized,
    {
        (self as &[T]).should_not_contain_any(elements);
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

impl<T, const N: usize> MembershipAssertion<T> for [T; N]
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

    fn should_contain_all<Q>(&self, elements: Vec<&Q>) -> &Self
    where
        T: Borrow<Q>,
        Q: Eq + Debug + ?Sized,
    {
        (self as &[T]).should_contain_all(elements);
        self
    }

    fn should_not_contain_all<Q>(&self, elements: Vec<&Q>) -> &Self
    where
        T: Borrow<Q>,
        Q: Eq + Debug + ?Sized,
    {
        (self as &[T]).should_not_contain_all(elements);
        self
    }

    fn should_contain_any<Q>(&self, elements: Vec<&Q>) -> &Self
    where
        T: Borrow<Q>,
        Q: Eq + Debug + ?Sized,
    {
        (self as &[T]).should_contain_any(elements);
        self
    }

    fn should_not_contain_any<Q>(&self, elements: Vec<&Q>) -> &Self
    where
        T: Borrow<Q>,
        Q: Eq + Debug + ?Sized,
    {
        (self as &[T]).should_not_contain_any(elements);
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

impl<T> MembershipAssertion<T> for [T]
where
    T: Debug,
    T: Eq,
{
    fn should_contain<Q>(&self, element: &Q) -> &Self
    where
        T: Borrow<Q>,
        Q: Eq + Debug + ?Sized,
    {
        map(self).should(&contain(element));
        self
    }

    fn should_not_contain<Q>(&self, element: &Q) -> &Self
    where
        T: Borrow<Q>,
        Q: Eq + Debug + ?Sized,
    {
        map(self).should_not(&contain(element));
        self
    }

    fn should_contain_all<Q>(&self, elements: Vec<&Q>) -> &Self
    where
        T: Borrow<Q>,
        Q: Eq + Debug + ?Sized,
    {
        map(self).should(&contain_all(elements));
        self
    }

    fn should_not_contain_all<Q>(&self, elements: Vec<&Q>) -> &Self
    where
        T: Borrow<Q>,
        Q: Eq + Debug + ?Sized,
    {
        map(self).should_not(&contain_all(elements));
        self
    }

    fn should_contain_any<Q>(&self, elements: Vec<&Q>) -> &Self
    where
        T: Borrow<Q>,
        Q: Eq + Debug + ?Sized,
    {
        map(self).should(&contain_any(elements));
        self
    }

    fn should_not_contain_any<Q>(&self, elements: Vec<&Q>) -> &Self
    where
        T: Borrow<Q>,
        Q: Eq + Debug + ?Sized,
    {
        map(self).should_not(&contain_any(elements));
        self
    }

    fn should_be_empty(&self) -> &Self {
        self.should(&be_empty());
        self
    }

    fn should_not_be_empty(&self) -> &Self {
        self.should_not(&be_empty());
        self
    }
}

fn map<T, Q: ?Sized>(collection: &[T]) -> Vec<&Q>
where
    T: Borrow<Q>,
{
    collection.iter().map(|source| source.borrow()).collect()
}

#[cfg(test)]
mod tests {
    use crate::assertions::collection::membership::MembershipAssertion;

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

    #[test]
    fn should_contain_all() {
        let collection = vec!["junit", "assert4j", "catch2"];
        let to_be_contained = vec!["assert4j", "junit"];
        collection.should_contain_all(to_be_contained);
    }

    #[test]
    #[should_panic]
    fn should_contain_all_but_was_not_contained() {
        let collection = vec!["junit", "assert4j", "catch2"];
        let to_be_contained = vec!["assert4j", "xunit"];
        collection.should_contain_all(to_be_contained);
    }

    #[test]
    fn should_not_contain_all() {
        let collection = vec!["junit", "assert4j", "catch2"];
        let to_be_contained = vec!["assert4j", "junit", "catch"];
        collection.should_not_contain_all(to_be_contained);
    }

    #[test]
    #[should_panic]
    fn should_not_contain_all_but_it_did() {
        let collection = vec!["junit", "assert4j", "catch2"];
        let to_be_contained = vec!["assert4j", "junit"];
        collection.should_not_contain_all(to_be_contained);
    }

    #[test]
    fn should_contain_any() {
        let collection = vec!["junit", "assert4j", "catch2"];
        let to_be_contained = vec!["assert4j", "xunit"];
        collection.should_contain_any(to_be_contained);
    }

    #[test]
    #[should_panic]
    fn should_contain_any_but_was_not_contained() {
        let collection = vec!["junit", "assert4j", "catch2"];
        let to_be_contained = vec!["catch", "xunit"];
        collection.should_contain_any(to_be_contained);
    }

    #[test]
    fn should_not_contain_any() {
        let collection = vec!["junit", "assert4j", "catch2"];
        let to_be_contained = vec!["assert", "xunit", "catch"];
        collection.should_not_contain_any(to_be_contained);
    }

    #[test]
    #[should_panic]
    fn should_not_contain_any_but_it_did() {
        let collection = vec!["junit", "assert4j", "catch2"];
        let to_be_contained = vec!["assert4j", "junit"];
        collection.should_not_contain_any(to_be_contained);
    }
}

#[cfg(test)]
mod array_tests {
    use crate::assertions::collection::membership::MembershipAssertion;

    #[test]
    fn should_contain() {
        let collection = ["junit", "assert4j", "catch2"];
        collection.should_contain("assert4j");
    }

    #[test]
    #[should_panic]
    fn should_contain_but_was_not_contained() {
        let collection = ["junit", "assert4j", "catch2"];
        collection.should_contain("catch");
    }

    #[test]
    fn should_not_contain() {
        let collection = ["junit", "assert4j", "catch2"];
        collection.should_not_contain("catch");
    }

    #[test]
    #[should_panic]
    fn should_not_contain_but_was_contained() {
        let collection = ["junit", "assert4j", "catch2"];
        collection.should_not_contain("catch2");
    }

    #[test]
    #[should_panic]
    fn should_be_empty_but_was_not() {
        let collection = ["junit", "testify"];
        collection.should_be_empty();
    }

    #[test]
    fn should_be_empty() {
        let collection: [i32; 0] = [];
        collection.should_be_empty();
    }

    #[test]
    #[should_panic]
    fn should_not_be_empty_but_was() {
        let collection: [i32; 0] = [];
        collection.should_not_be_empty();
    }

    #[test]
    fn should_not_be_empty() {
        let collection = ["junit", "testify"];
        collection.should_not_be_empty();
    }

    #[test]
    fn should_contain_all() {
        let collection = ["junit", "assert4j", "catch2"];
        let to_be_contained = vec!["assert4j", "junit"];
        collection.should_contain_all(to_be_contained);
    }

    #[test]
    #[should_panic]
    fn should_contain_all_but_was_not_contained() {
        let collection = ["junit", "assert4j", "catch2"];
        let to_be_contained = vec!["assert4j", "xunit"];
        collection.should_contain_all(to_be_contained);
    }

    #[test]
    fn should_not_contain_all() {
        let collection = ["junit", "assert4j", "catch2"];
        let to_be_contained = vec!["assert4j", "junit", "catch"];
        collection.should_not_contain_all(to_be_contained);
    }

    #[test]
    #[should_panic]
    fn should_not_contain_all_but_it_did() {
        let collection = ["junit", "assert4j", "catch2"];
        let to_be_contained = vec!["assert4j", "junit"];
        collection.should_not_contain_all(to_be_contained);
    }

    #[test]
    fn should_contain_any() {
        let collection = ["junit", "assert4j", "catch2"];
        let to_be_contained = vec!["assert4j", "xunit"];
        collection.should_contain_any(to_be_contained);
    }

    #[test]
    #[should_panic]
    fn should_contain_any_but_was_not_contained() {
        let collection = ["junit", "assert4j", "catch2"];
        let to_be_contained = vec!["catch", "xunit"];
        collection.should_contain_any(to_be_contained);
    }

    #[test]
    fn should_not_contain_any() {
        let collection = ["junit", "assert4j", "catch2"];
        let to_be_contained = vec!["assert", "xunit", "catch"];
        collection.should_not_contain_any(to_be_contained);
    }

    #[test]
    #[should_panic]
    fn should_not_contain_any_but_it_did() {
        let collection = ["junit", "assert4j", "catch2"];
        let to_be_contained = vec!["assert4j", "junit"];
        collection.should_not_contain_any(to_be_contained);
    }
}
