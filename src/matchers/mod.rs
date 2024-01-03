pub mod range;

pub trait Should<T> {
    fn should(&self, matcher: &dyn Matcher<T>) -> bool;
}

pub trait ShouldNot<T> {
    fn should_not(&self, matcher: &dyn Matcher<T>) -> bool;
}

impl<T> Should<T> for T {
    fn should(&self, matcher: &dyn Matcher<T>) -> bool {
        matcher.test(&self)
    }
}

impl<T> ShouldNot<T> for T {
    fn should_not(&self, matcher: &dyn Matcher<T>) -> bool {
        !matcher.test(&self)
    }
}

pub trait Matcher<T> {
    fn test(&self, value: &T) -> bool;
}
