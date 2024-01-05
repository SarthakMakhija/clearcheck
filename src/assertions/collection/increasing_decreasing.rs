use crate::matchers::collection::increasing_decreasing::{
    be_monotonically_decreasing, be_monotonically_increasing, be_strictly_decreasing,
    be_strictly_increasing,
};
use crate::matchers::Should;

pub trait IncreasingDecreasing<T>
where
    T: PartialOrd + std::fmt::Debug,
{
    fn should_be_monotonically_increasing(&self) -> &Self;
    fn should_be_monotonically_decreasing(&self) -> &Self;
    fn should_be_strictly_increasing(&self) -> &Self;
    fn should_be_strictly_decreasing(&self) -> &Self;
}

impl<T> IncreasingDecreasing<T> for Vec<T>
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

impl<T, const N: usize> IncreasingDecreasing<T> for [T; N]
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

impl<T> IncreasingDecreasing<T> for [T]
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
    use crate::assertions::collection::increasing_decreasing::IncreasingDecreasing;

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
