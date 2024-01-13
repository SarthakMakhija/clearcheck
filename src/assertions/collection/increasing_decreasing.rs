use crate::matchers::collection::increasing_decreasing::{
    be_monotonically_decreasing, be_monotonically_increasing, be_strictly_decreasing,
    be_strictly_increasing,
};
use crate::matchers::Should;

///IncreasingDecreasingAssertion enables assertions about the order of elements within a collection.
pub trait IncreasingDecreasingAssertion<T>
where
    T: PartialOrd + std::fmt::Debug,
{
    /// - Asserts that the elements in the collection are in non-decreasing order (allowing consecutive equal elements).
    /// - An empty collection is considered monotonically increasing.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::collection::increasing_decreasing::IncreasingDecreasingAssertion;
    ///
    /// let collection = vec![1, 2, 3, 4, 4, 5, 5, 6];
    /// collection.should_be_monotonically_increasing();
    /// ```
    fn should_be_monotonically_increasing(&self) -> &Self;

    /// - Asserts that the elements in the collection are in non-increasing order (allowing consecutive equal elements).
    /// - An empty collection is considered monotonically decreasing.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::collection::increasing_decreasing::IncreasingDecreasingAssertion;
    ///
    /// let collection = vec![6, 6, 5, 5, 4, 4, 2];
    /// collection.should_be_monotonically_decreasing();
    /// ```
    fn should_be_monotonically_decreasing(&self) -> &Self;

    /// - Asserts that the elements in the collection are in strictly increasing order (no consecutive elements can be equal).
    /// - An empty collection is considered monotonically increasing.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::collection::increasing_decreasing::IncreasingDecreasingAssertion;
    ///
    /// let collection = vec![1, 3, 5, 7, 9];
    /// collection.should_be_strictly_increasing();
    /// ```
    fn should_be_strictly_increasing(&self) -> &Self;

    /// - Asserts that the elements in the collection are in strictly decreasing order (no consecutive elements can be equal).
    /// - An empty collection is considered monotonically increasing.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    /// # Example
    /// ```
    /// use clearcheck::assertions::collection::increasing_decreasing::IncreasingDecreasingAssertion;
    ///
    /// let collection = vec![9, 7, 5, 3, 1];
    /// collection.should_be_strictly_decreasing();
    /// ```
    fn should_be_strictly_decreasing(&self) -> &Self;
}

impl<T> IncreasingDecreasingAssertion<T> for Vec<T>
where
    T: std::fmt::Debug,
    T: PartialOrd,
{
    fn should_be_monotonically_increasing(&self) -> &Self {
        (self as &[T]).should_be_monotonically_increasing();
        self
    }

    fn should_be_monotonically_decreasing(&self) -> &Self {
        (self as &[T]).should_be_monotonically_decreasing();
        self
    }

    fn should_be_strictly_increasing(&self) -> &Self {
        (self as &[T]).should_be_strictly_increasing();
        self
    }

    fn should_be_strictly_decreasing(&self) -> &Self {
        (self as &[T]).should_be_strictly_decreasing();
        self
    }
}

impl<T, const N: usize> IncreasingDecreasingAssertion<T> for [T; N]
where
    T: std::fmt::Debug,
    T: PartialOrd,
{
    fn should_be_monotonically_increasing(&self) -> &Self {
        (self as &[T]).should_be_monotonically_increasing();
        self
    }

    fn should_be_monotonically_decreasing(&self) -> &Self {
        (self as &[T]).should_be_monotonically_decreasing();
        self
    }

    fn should_be_strictly_increasing(&self) -> &Self {
        (self as &[T]).should_be_strictly_increasing();
        self
    }

    fn should_be_strictly_decreasing(&self) -> &Self {
        (self as &[T]).should_be_strictly_decreasing();
        self
    }
}

impl<T> IncreasingDecreasingAssertion<T> for [T]
where
    T: std::fmt::Debug,
    T: PartialOrd,
{
    fn should_be_monotonically_increasing(&self) -> &Self {
        self.should(&be_monotonically_increasing());
        self
    }

    fn should_be_monotonically_decreasing(&self) -> &Self {
        self.should(&be_monotonically_decreasing());
        self
    }

    fn should_be_strictly_increasing(&self) -> &Self {
        self.should(&be_strictly_increasing());
        self
    }

    fn should_be_strictly_decreasing(&self) -> &Self {
        self.should(&be_strictly_decreasing());
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::assertions::collection::increasing_decreasing::IncreasingDecreasingAssertion;

    #[test]
    fn empty_should_be_monotonically_increasing() {
        let collection: Vec<i32> = vec![];
        collection.should_be_monotonically_increasing();
    }

    #[test]
    fn empty_should_be_monotonically_decreasing() {
        let collection: Vec<i32> = vec![];
        collection.should_be_monotonically_decreasing();
    }

    #[test]
    fn empty_should_be_strictly_increasing() {
        let collection: Vec<i32> = vec![];
        collection.should_be_strictly_increasing();
    }

    #[test]
    fn empty_should_be_strictly_decreasing() {
        let collection: Vec<i32> = vec![];
        collection.should_be_strictly_decreasing();
    }

    #[test]
    fn should_be_monotonically_increasing() {
        let collection = vec![1, 2, 3, 4, 4, 5, 5, 6];
        collection.should_be_monotonically_increasing();
    }

    #[test]
    #[should_panic]
    fn should_be_monotonically_increasing_but_was_not() {
        let collection = vec![1, 2, 3, 4, 4, 2, 5, 6];
        collection.should_be_monotonically_increasing();
    }

    #[test]
    fn should_be_monotonically_decreasing() {
        let collection = vec![6, 6, 5, 5, 4, 4, 2];
        collection.should_be_monotonically_decreasing();
    }

    #[test]
    #[should_panic]
    fn should_be_monotonically_decreasing_but_was_not() {
        let collection = vec![6, 6, 5, 5, 4, 4, 5];
        collection.should_be_monotonically_decreasing();
    }

    #[test]
    fn should_be_strictly_increasing() {
        let collection = vec![1, 3, 5, 7, 9];
        collection.should_be_strictly_increasing();
    }

    #[test]
    #[should_panic]
    fn should_be_strictly_increasing_but_was_not() {
        let collection = vec![1, 5, 7, 9, 9];
        collection.should_be_strictly_increasing();
    }

    #[test]
    fn should_be_strictly_decreasing() {
        let collection = vec![9, 7, 5, 3, 1];
        collection.should_be_strictly_decreasing();
    }

    #[test]
    #[should_panic]
    fn should_be_strictly_decreasing_but_was_not() {
        let collection = vec![9, 7, 5, 3, 1, 1];
        collection.should_be_strictly_decreasing();
    }
}

#[cfg(test)]
mod array_tests {
    use crate::assertions::collection::increasing_decreasing::IncreasingDecreasingAssertion;

    #[test]
    fn should_be_monotonically_increasing() {
        let collection = [1, 2, 3, 4, 4, 5, 5, 6];
        collection.should_be_monotonically_increasing();
    }

    #[test]
    #[should_panic]
    fn should_be_monotonically_increasing_but_was_not() {
        let collection = [1, 2, 3, 4, 4, 2, 5, 6];
        collection.should_be_monotonically_increasing();
    }

    #[test]
    fn should_be_monotonically_decreasing() {
        let collection = [6, 6, 5, 5, 4, 4, 2];
        collection.should_be_monotonically_decreasing();
    }

    #[test]
    #[should_panic]
    fn should_be_monotonically_decreasing_but_was_not() {
        let collection = [6, 6, 5, 5, 4, 4, 5];
        collection.should_be_monotonically_decreasing();
    }

    #[test]
    fn should_be_strictly_increasing() {
        let collection = [1, 3, 5, 7, 9];
        collection.should_be_strictly_increasing();
    }

    #[test]
    #[should_panic]
    fn should_be_strictly_increasing_but_was_not() {
        let collection = [1, 5, 7, 9, 9];
        collection.should_be_strictly_increasing();
    }

    #[test]
    fn should_be_strictly_decreasing() {
        let collection = [9, 7, 5, 3, 1];
        collection.should_be_strictly_decreasing();
    }

    #[test]
    #[should_panic]
    fn should_be_strictly_decreasing_but_was_not() {
        let collection = [9, 7, 5, 3, 1, 1];
        collection.should_be_strictly_decreasing();
    }
}
