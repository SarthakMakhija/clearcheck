use crate::matchers::Matcher;

pub enum SortBased {
    Ascending,
    Descending,
}

impl SortBased {
    fn test<T: PartialOrd>(&self, collection: &[T]) -> bool {
        match self {
            SortBased::Ascending => {
                (0..collection.len() - 1).all(|index| collection[index] <= collection[index + 1])
            }
            SortBased::Descending => {
                (0..collection.len() - 1).all(|index| collection[index] >= collection[index + 1])
            }
        }
    }
}

impl<T: PartialOrd> Matcher<Vec<T>> for SortBased {
    fn test(&self, collection: &Vec<T>) -> bool {
        self.test(&collection)
    }
}

impl<T: PartialOrd, const N: usize> Matcher<[T; N]> for SortBased {
    fn test(&self, collection: &[T; N]) -> bool {
        self.test(collection as &[T])
    }
}

impl<T: PartialOrd> Matcher<&[T]> for SortBased {
    fn test(&self, collection: &&[T]) -> bool {
        self.test(&collection)
    }
}

pub fn be_sorted_ascending() -> SortBased {
    SortBased::Ascending
}

pub fn be_sorted_descending() -> SortBased {
    SortBased::Descending
}

#[cfg(test)]
mod tests {
    use crate::assertions::bool::TrueFalse;
    use crate::matchers::collection::sort::{be_sorted_ascending, be_sorted_descending};

    #[test]
    fn should_be_sorted_ascending() {
        let matcher = be_sorted_ascending();
        let collection = vec!["assert4j", "junit"];
        matcher.test(&collection).should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_sorted_ascending_but_was_not() {
        let matcher = be_sorted_ascending();
        let collection = vec!["junit", "assert4j"];
        matcher.test(&collection).should_be_true();
    }

    #[test]
    fn should_be_sorted_descending() {
        let matcher = be_sorted_descending();
        let collection = vec!["junit", "assert4j"];
        matcher.test(&collection).should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_sorted_descending_but_was_not() {
        let matcher = be_sorted_descending();
        let collection = vec!["assert4j", "junit"];
        matcher.test(&collection).should_be_true();
    }
}
