use crate::panicking::{assert_failed_unary, AssertKind};

pub(crate) trait Empty {
    fn should_be_empty(&self) -> &Self;
    fn should_not_be_empty(&self) -> &Self;
}

impl<T> Empty for Vec<T>
where
    T: std::fmt::Debug,
{
    fn should_be_empty(&self) -> &Self {
        if !self.is_empty() {
            assert_failed_unary(AssertKind::Empty, &self)
        }
        self
    }

    fn should_not_be_empty(&self) -> &Self {
        if self.is_empty() {
            assert_failed_unary(AssertKind::NotEmpty, &self)
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::matchers::collection::empty::Empty;

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
