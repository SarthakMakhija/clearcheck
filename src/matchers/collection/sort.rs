use crate::panicking::{assert_failed_unary, AssertKind};

pub(crate) trait Sorted<T>
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
        let sorted = (0..self.len() - 1).all(|index| self[index] <= self[index + 1]);
        if !sorted {
            assert_failed_unary(AssertKind::SortsAscending, &self);
        }
        self
    }

    fn should_be_sorted_descending(&self) -> &Self {
        let sorted = (0..self.len() - 1).all(|index| self[index] >= self[index + 1]);
        if !sorted {
            assert_failed_unary(AssertKind::SortsDescending, &self);
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::matchers::collection::sort::Sorted;

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
