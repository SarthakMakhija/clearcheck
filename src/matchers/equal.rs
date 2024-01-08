pub enum IgnoreCaseEqualityMatcher<'a, T: Eq> {
    IgnoringCase(&'a T),
}

pub fn be_equal_ignoring_case<T: Eq>(other: &T) -> IgnoreCaseEqualityMatcher<'_, T> {
    IgnoreCaseEqualityMatcher::IgnoringCase(other)
}
