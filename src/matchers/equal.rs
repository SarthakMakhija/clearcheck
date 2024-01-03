pub enum EqualityBased<'a, T: Eq> {
    IgnoringCase(&'a T),
}

pub fn be_equal_ignoring_case<T: Eq>(other: &T) -> EqualityBased<'_, T> {
    EqualityBased::IgnoringCase(other)
}
