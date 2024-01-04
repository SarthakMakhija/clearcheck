use crate::matchers::Matcher;

pub enum BoundBased<'a, T: PartialOrd> {
    Upper(&'a T),
    Lower(&'a T),
}

impl<'a, T> BoundBased<'a, T>
where
    T: PartialOrd,
{
    fn test(&self, collection: &[T]) -> bool {
        match self {
            BoundBased::Upper(bound) => collection.iter().all(|source| bound >= &source),
            BoundBased::Lower(bound) => collection.iter().all(|source| bound <= &source),
        }
    }
}

impl<T: PartialOrd> Matcher<Vec<T>> for BoundBased<'_, T> {
    fn test(&self, collection: &Vec<T>) -> bool {
        self.test(&collection)
    }
}

impl<T: PartialOrd, const N: usize> Matcher<[T; N]> for BoundBased<'_, T> {
    fn test(&self, collection: &[T; N]) -> bool {
        self.test(collection as &[T])
    }
}

impl<T: PartialOrd> Matcher<&[T]> for BoundBased<'_, T> {
    fn test(&self, collection: &&[T]) -> bool {
        self.test(&collection)
    }
}

pub fn have_upper_bound<T: PartialOrd>(bound: &T) -> BoundBased<'_, T> {
    BoundBased::Upper(bound)
}

pub fn have_lower_bound<T: PartialOrd>(bound: &T) -> BoundBased<'_, T> {
    BoundBased::Lower(bound)
}
