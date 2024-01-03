use crate::matchers::Matcher;

pub enum BoundBased<'a, T: PartialOrd> {
    Upper(&'a T),
    Lower(&'a T),
}

impl<T: PartialOrd> Matcher<&[T]> for BoundBased<'_, T> {
    fn test(&self, value: &&[T]) -> bool {
        match self {
            BoundBased::Upper(bound) => value.iter().all(|source| bound >= &source),
            BoundBased::Lower(bound) => value.iter().all(|source| bound <= &source),
        }
    }
}

pub fn have_upper_bound<T: PartialOrd>(bound: &T) -> BoundBased<'_, T> {
    BoundBased::Upper(bound)
}

pub fn have_lower_bound<T: PartialOrd>(bound: &T) -> BoundBased<'_, T> {
    BoundBased::Lower(bound)
}
