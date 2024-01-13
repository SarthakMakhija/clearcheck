use crate::matchers::collection::sort::{be_sorted_ascending, be_sorted_descending};
use crate::matchers::Should;

/// SortAssertion enables assertions about whether a collection's elements are sorted in a specific order.
pub trait SortAssertion<T>
where
    T: PartialOrd,
{
    /// - Asserts that the elements of the collection are in ascending order (non-decreasing, allowing duplicates).
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::collection::sort::SortAssertion;
    ///
    /// let collection = vec![1, 2, 3, 3, 5];
    /// collection.should_be_sorted_ascending();
    /// ```
    fn should_be_sorted_ascending(&self) -> &Self;

    /// - Asserts that the elements of the collection are in descending order (non-increasing, allowing duplicates).
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::collection::sort::SortAssertion;
    ///
    /// let collection = vec![5, 4, 3, 3, 2, 1];
    /// collection.should_be_sorted_descending();
    /// ```
    fn should_be_sorted_descending(&self) -> &Self;
}

impl<T> SortAssertion<T> for Vec<T>
where
    T: std::fmt::Debug + PartialOrd,
{
    fn should_be_sorted_ascending(&self) -> &Self {
        (self as &[T]).should_be_sorted_ascending();
        self
    }

    fn should_be_sorted_descending(&self) -> &Self {
        (self as &[T]).should_be_sorted_descending();
        self
    }
}

impl<T, const N: usize> SortAssertion<T> for [T; N]
where
    T: std::fmt::Debug + PartialOrd,
{
    fn should_be_sorted_ascending(&self) -> &Self {
        (self as &[T]).should_be_sorted_ascending();
        self
    }

    fn should_be_sorted_descending(&self) -> &Self {
        (self as &[T]).should_be_sorted_descending();
        self
    }
}

impl<T> SortAssertion<T> for [T]
where
    T: std::fmt::Debug + PartialOrd,
{
    fn should_be_sorted_ascending(&self) -> &Self {
        self.should(&be_sorted_ascending());
        self
    }

    fn should_be_sorted_descending(&self) -> &Self {
        self.should(&be_sorted_descending());
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::assertions::collection::sort::SortAssertion;

    #[test]
    fn should_be_sorted_in_ascending_order() {
        let collection = vec!["actual", "assert", "catch", "testify"];
        collection.should_be_sorted_ascending();
    }

    #[test]
    #[should_panic]
    fn should_be_sorted_in_ascending_order_but_was_not() {
        let collection = vec!["actual", "testify", "catch"];
        collection.should_be_sorted_ascending();
    }

    #[test]
    fn should_be_sorted_in_descending_order() {
        let collection = vec!["testify", "catch", "assert", "actual"];
        collection.should_be_sorted_descending();
    }

    #[test]
    #[should_panic]
    fn should_be_sorted_in_descending_order_but_was_not() {
        let collection = vec!["actual", "testify", "catch"];
        collection.should_be_sorted_descending();
    }
}

#[cfg(test)]
mod array_tests {
    use crate::assertions::collection::sort::SortAssertion;

    #[test]
    fn should_be_sorted_in_ascending_order() {
        let collection = ["actual", "assert", "catch", "testify"];
        collection.should_be_sorted_ascending();
    }

    #[test]
    #[should_panic]
    fn should_be_sorted_in_ascending_order_but_was_not() {
        let collection = ["actual", "testify", "catch"];
        collection.should_be_sorted_ascending();
    }

    #[test]
    fn should_be_sorted_in_descending_order() {
        let collection = ["testify", "catch", "assert", "actual"];
        collection.should_be_sorted_descending();
    }

    #[test]
    #[should_panic]
    fn should_be_sorted_in_descending_order_but_was_not() {
        let collection = ["actual", "testify", "catch"];
        collection.should_be_sorted_descending();
    }
}
