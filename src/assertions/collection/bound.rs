use crate::matchers::collection::bound::{have_lower_bound, have_upper_bound};
use crate::matchers::Should;

pub trait BoundAssertion<T>
where
    T: PartialOrd + std::fmt::Debug,
{
    fn should_have_upper_bound(&self, element: T) -> &Self;
    fn should_have_lower_bound(&self, element: T) -> &Self;
}

impl<T> BoundAssertion<T> for Vec<T>
where
    T: std::fmt::Debug,
    T: PartialOrd,
{
    fn should_have_upper_bound(&self, element: T) -> &Self {
        (self as &[T]).should_have_upper_bound(element);
        self
    }

    fn should_have_lower_bound(&self, element: T) -> &Self {
        (self as &[T]).should_have_lower_bound(element);
        self
    }
}

impl<T, const N: usize> BoundAssertion<T> for [T; N]
where
    T: std::fmt::Debug,
    T: PartialOrd,
{
    fn should_have_upper_bound(&self, element: T) -> &Self {
        (self as &[T]).should_have_upper_bound(element);
        self
    }

    fn should_have_lower_bound(&self, element: T) -> &Self {
        (self as &[T]).should_have_lower_bound(element);
        self
    }
}

impl<T> BoundAssertion<T> for [T]
where
    T: std::fmt::Debug,
    T: PartialOrd,
{
    fn should_have_upper_bound(&self, element: T) -> &Self {
        self.should(&have_upper_bound(element));
        self
    }

    fn should_have_lower_bound(&self, element: T) -> &Self {
        self.should(&have_lower_bound(element));
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::assertions::collection::bound::BoundAssertion;

    #[test]
    fn should_have_an_upper_bound() {
        let collection = vec![1, 2, 3, 4];
        collection.should_have_upper_bound(4);
    }

    #[test]
    #[should_panic]
    fn should_have_an_upper_bound_but_was_not() {
        let collection = vec![1, 2, 3, 4];
        collection.should_have_upper_bound(1);
    }

    #[test]
    fn should_have_a_lower_bound() {
        let collection = vec![1, 2, 3, 4];
        collection.should_have_lower_bound(1);
    }

    #[test]
    #[should_panic]
    fn should_have_a_lower_bound_but_was_not() {
        let collection = vec![1, 2, 3, 4];
        collection.should_have_lower_bound(3);
    }
}

#[cfg(test)]
mod array_tests {
    use crate::assertions::collection::bound::BoundAssertion;

    #[test]
    fn should_have_an_upper_bound() {
        let collection = [1, 2, 3, 4];
        collection.should_have_upper_bound(4);
    }

    #[test]
    #[should_panic]
    fn should_have_an_upper_bound_but_was_not() {
        let collection = [1, 2, 3, 4];
        collection.should_have_upper_bound(1);
    }

    #[test]
    fn should_have_a_lower_bound() {
        let collection = [1, 2, 3, 4];
        collection.should_have_lower_bound(1);
    }

    #[test]
    #[should_panic]
    fn should_have_a_lower_bound_but_was_not() {
        let collection = [1, 2, 3, 4];
        collection.should_have_lower_bound(3);
    }
}
