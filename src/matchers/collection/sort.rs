use crate::matchers::Matcher;

pub enum SortBased {
    Ascending,
    Descending,
}

impl<T: PartialOrd> Matcher<&[T]> for SortBased {
    fn test(&self, collection: &&[T]) -> bool {
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

pub fn be_sorted_ascending() -> SortBased {
    SortBased::Ascending
}

pub fn be_sorted_descending() -> SortBased {
    SortBased::Descending
}
