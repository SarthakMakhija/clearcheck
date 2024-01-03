use std::ops::{Range, RangeInclusive};

use crate::panicking::{assert_failed_binary, AssertKind};

pub trait CharRange {
    fn should_be_in_inclusive_range(&self, range: RangeInclusive<char>) -> &Self;
    fn should_not_be_in_inclusive_range(&self, range: RangeInclusive<char>) -> &Self;
    fn should_be_in_exclusive_range(&self, range: Range<char>) -> &Self;
    fn should_not_be_in_exclusive_range(&self, range: Range<char>) -> &Self;
}

pub trait Equal {
    fn should_be_equal_ignoring_case(&self, other: &char) -> &Self;
    fn should_not_be_equal_ignoring_case(&self, other: &char) -> &Self;
}

impl CharRange for char {
    fn should_be_in_inclusive_range(&self, range: RangeInclusive<char>) -> &Self {
        if !range.contains(&self) {
            assert_failed_binary(AssertKind::InRange, self, &range);
        }
        self
    }

    fn should_not_be_in_inclusive_range(&self, range: RangeInclusive<char>) -> &Self {
        if range.contains(&self) {
            assert_failed_binary(AssertKind::NotInRange, self, &range);
        }
        self
    }

    fn should_be_in_exclusive_range(&self, range: Range<char>) -> &Self {
        if !range.contains(&self) {
            assert_failed_binary(AssertKind::InRange, self, &range);
        }
        self
    }

    fn should_not_be_in_exclusive_range(&self, range: Range<char>) -> &Self {
        if range.contains(&self) {
            assert_failed_binary(AssertKind::NotInRange, self, &range);
        }
        self
    }
}

impl Equal for char {
    fn should_be_equal_ignoring_case(&self, other: &char) -> &Self {
        if !self.eq_ignore_ascii_case(other) {
            assert_failed_binary(AssertKind::Equal, self, other);
        }
        self
    }

    fn should_not_be_equal_ignoring_case(&self, other: &char) -> &Self {
        if self.eq_ignore_ascii_case(other) {
            assert_failed_binary(AssertKind::NotEqual, self, other);
        }
        self
    }
}

#[cfg(test)]
mod range_tests {
    use crate::assertions::char::CharRange;

    #[test]
    fn should_be_in_the_inclusive_range() {
        let letter = 'd';
        letter.should_be_in_inclusive_range('a'..='d');
    }

    #[test]
    #[should_panic]
    fn should_be_in_the_inclusive_range_but_was_not() {
        let letter = 'd';
        letter.should_be_in_inclusive_range('a'..='c');
    }

    #[test]
    fn should_not_be_in_the_inclusive_range() {
        let letter = 'd';
        letter.should_not_be_in_inclusive_range('a'..='c');
    }

    #[test]
    #[should_panic]
    fn should_not_be_in_the_inclusive_range_but_was() {
        let letter = 'd';
        letter.should_not_be_in_inclusive_range('a'..='d');
    }

    #[test]
    fn should_be_in_the_exclusive_range() {
        let letter = 'd';
        letter.should_be_in_exclusive_range('a'..'e');
    }

    #[test]
    #[should_panic]
    fn should_be_in_the_exclusive_range_but_was_not() {
        let letter = 'd';
        letter.should_be_in_exclusive_range('a'..'d');
    }

    #[test]
    fn should_not_be_in_the_exclusive_range() {
        let letter = 'd';
        letter.should_not_be_in_exclusive_range('a'..'d');
    }

    #[test]
    #[should_panic]
    fn should_not_be_in_the_exclusive_range_but_was() {
        let letter = 'd';
        letter.should_not_be_in_exclusive_range('a'..'e');
    }
}

#[cfg(test)]
mod equal_tests {
    use crate::assertions::char::Equal;

    #[test]
    fn should_be_equal_ignoring_case() {
        let letter = 'd';
        letter.should_be_equal_ignoring_case(&'D');
    }

    #[test]
    #[should_panic]
    fn should_be_equal_ignoring_case_but_was_not() {
        let letter = 'd';
        letter.should_be_equal_ignoring_case(&'E');
    }

    #[test]
    fn should_not_be_equal_ignoring_case() {
        let letter = 'd';
        letter.should_not_be_equal_ignoring_case(&'E');
    }

    #[test]
    #[should_panic]
    fn should_not_be_equal_ignoring_case_but_was() {
        let letter = 'd';
        letter.should_not_be_equal_ignoring_case(&'D');
    }
}
