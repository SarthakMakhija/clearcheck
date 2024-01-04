pub enum LengthBased {
    Same(usize),
    Atleast(usize),
    Atmost(usize),
    Zero,
}

impl LengthBased {
    pub fn test(&self, input_length: usize) -> bool {
        match self {
            LengthBased::Same(length) => input_length == *length,
            LengthBased::Atleast(length) => input_length >= *length,
            LengthBased::Atmost(length) => input_length <= *length,
            LengthBased::Zero => input_length == 0,
        }
    }
}

pub fn have_same_length(length: usize) -> LengthBased {
    LengthBased::Same(length)
}

pub fn have_atleast_same_length(length: usize) -> LengthBased {
    LengthBased::Atleast(length)
}

pub fn have_atmost_same_length(length: usize) -> LengthBased {
    LengthBased::Atmost(length)
}

pub fn have_zero_length() -> LengthBased {
    LengthBased::Zero
}

#[cfg(test)]
mod tests {
    use crate::assertions::bool::TrueFalse;
    use crate::matchers::length::{
        have_atleast_same_length, have_atmost_same_length, have_same_length, have_zero_length,
    };

    #[test]
    fn should_have_same_length() {
        let matcher = have_same_length(4);
        matcher.test(4).should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_have_same_length_but_was_not() {
        let matcher = have_same_length(4);
        matcher.test(2).should_be_true();
    }

    #[test]
    fn should_have_atleast_same_length() {
        let matcher = have_atleast_same_length(4);
        matcher.test(5).should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_have_atleast_same_length_but_was_not() {
        let matcher = have_atleast_same_length(4);
        matcher.test(2).should_be_true();
    }

    #[test]
    fn should_have_atmost_length() {
        let matcher = have_atmost_same_length(4);
        matcher.test(3).should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_have_atmost_length_but_was_not() {
        let matcher = have_atmost_same_length(4);
        matcher.test(5).should_be_true();
    }

    #[test]
    fn should_have_zero_length() {
        let matcher = have_zero_length();
        matcher.test(0).should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_have_zero_length_but_was_not() {
        let matcher = have_zero_length();
        matcher.test(2).should_be_true();
    }
}
