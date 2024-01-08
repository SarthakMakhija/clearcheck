pub enum EqualityMatcher<'a, T: Eq> {
    IgnoringCase(&'a T),
}

pub fn be_equal_ignoring_case<T: Eq>(other: &T) -> EqualityMatcher<'_, T> {
    EqualityMatcher::IgnoringCase(other)
}
