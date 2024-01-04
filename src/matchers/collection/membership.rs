use crate::matchers::Matcher;

pub enum MembershipBased<'a, T: Eq> {
    Contain(&'a T),
}

impl<'a, T: Eq> MembershipBased<'a, T> {
    fn test(&self, collection: &[T]) -> bool {
        match self {
            MembershipBased::Contain(element) => collection.contains(element),
        }
    }
}

impl<T> Matcher<Vec<T>> for MembershipBased<'_, T>
where
    T: Eq,
{
    fn test(&self, collection: &Vec<T>) -> bool {
        self.test(&collection)
    }
}

impl<T, const N: usize> Matcher<[T; N]> for MembershipBased<'_, T>
where
    T: Eq,
{
    fn test(&self, collection: &[T; N]) -> bool {
        self.test(collection as &[T])
    }
}

impl<T> Matcher<&[T]> for MembershipBased<'_, T>
where
    T: Eq,
{
    fn test(&self, collection: &&[T]) -> bool {
        self.test(&collection)
    }
}

pub fn contain<T>(element: &T) -> MembershipBased<'_, T>
where
    T: Eq,
{
    MembershipBased::Contain(element)
}

#[cfg(test)]
mod tests {
    use crate::assertions::bool::TrueFalse;
    use crate::matchers::collection::membership::contain;

    #[test]
    fn should_contain() {
        let collection = vec!["junit", "testify"];
        let matcher = contain(&"junit");
        matcher.test(&collection).should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_contain_but_id_did_not() {
        let collection = vec!["unit4j", "testify"];
        let matcher = contain(&"junit");
        matcher.test(&collection).should_be_true();
    }
}
