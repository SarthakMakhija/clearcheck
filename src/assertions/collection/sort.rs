use crate::matchers::collection::sort::{be_sorted_ascending, be_sorted_descending};
use crate::matchers::Should;

pub trait Sorted<T>
where
    T: PartialOrd,
{
    fn should_be_sorted_ascending(&self) -> &Self;
    fn should_be_sorted_descending(&self) -> &Self;
}

impl<T> Sorted<T> for Vec<T>
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

impl<T, const N: usize> Sorted<T> for [T; N]
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

impl<T> Sorted<T> for [T]
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
    use crate::assertions::collection::sort::Sorted;

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
