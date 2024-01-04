use crate::matchers::Matcher;

pub enum MembershipBased<'a, T: Eq> {
    Contain(&'a T),
}

impl<T> Matcher<Vec<T>> for MembershipBased<'_, T>
where
    T: Eq,
{
    fn test(&self, collection: &Vec<T>) -> bool {
        match self {
            MembershipBased::Contain(element) => collection.contains(element),
        }
    }
}

impl<T> Matcher<&[T]> for MembershipBased<'_, T>
where
    T: Eq,
{
    fn test(&self, collection: &&[T]) -> bool {
        match self {
            MembershipBased::Contain(element) => collection.contains(element),
        }
    }
}

pub fn contain<T>(element: &T) -> MembershipBased<'_, T>
where
    T: Eq,
{
    MembershipBased::Contain(element)
}
