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

#[cfg(test)]
mod tests {
    use crate::assertions::bool::TrueFalse;
    use crate::matchers::collection::bound::{have_lower_bound, have_upper_bound};

    #[test]
    fn should_have_an_upper_bound() {
        let matcher = have_upper_bound(&4);
        let collection = vec![1, 2, 3, 4];

        matcher.test(&collection).should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_have_an_upper_bound_but_was_not() {
        let matcher = have_upper_bound(&3);
        let collection = vec![1, 2, 3, 4];

        matcher.test(&collection).should_be_true();
    }

    #[test]
    fn should_have_a_lower_bound() {
        let matcher = have_lower_bound(&1);
        let collection = vec![1, 2, 3, 4];

        matcher.test(&collection).should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_have_a_lower_bound_but_was_not() {
        let matcher = have_lower_bound(&3);
        let collection = vec![1, 2, 3, 4];

        matcher.test(&collection).should_be_true();
    }
}
