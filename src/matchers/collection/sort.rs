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
