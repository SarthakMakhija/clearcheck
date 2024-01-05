use crate::matchers::MatcherResult;

pub enum LengthBased {
    Same(usize),
    Atleast(usize),
    Atmost(usize),
}

impl LengthBased {
    pub fn test(&self, input_length: usize) -> MatcherResult {
        match self {
            LengthBased::Same(length) => MatcherResult::formatted(
                input_length == *length,
                format!("Length {:?} should be {:?}", input_length, length),
                format!("Length {:?} should not be {:?}", input_length, length),
            ),
            LengthBased::Atleast(length) => MatcherResult::formatted(
                input_length >= *length,
                format!("Length {:?} should be atleast {:?}", input_length, length),
                format!(
                    "Length {:?} should not be atleast {:?}",
                    input_length, length
                ),
            ),
            LengthBased::Atmost(length) => MatcherResult::formatted(
                input_length <= *length,
                format!("Length {:?} should be atmost {:?}", input_length, length),
                format!(
                    "Length {:?} should not be atmost {:?}",
                    input_length, length
                ),
            ),
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

#[cfg(test)]
mod tests {
    use crate::assertions::bool::TrueFalse;
    use crate::matchers::length::{
        have_atleast_same_length, have_atmost_same_length, have_same_length,
    };

    #[test]
    fn should_have_same_length() {
        let matcher = have_same_length(4);
        matcher.test(4).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_have_same_length_but_was_not() {
        let matcher = have_same_length(4);
        matcher.test(2).passed.should_be_true();
    }

    #[test]
    fn should_have_atleast_same_length() {
        let matcher = have_atleast_same_length(4);
        matcher.test(5).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_have_atleast_same_length_but_was_not() {
        let matcher = have_atleast_same_length(4);
        matcher.test(2).passed.should_be_true();
    }

    #[test]
    fn should_have_atmost_length() {
        let matcher = have_atmost_same_length(4);
        matcher.test(3).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_have_atmost_length_but_was_not() {
        let matcher = have_atmost_same_length(4);
        matcher.test(5).passed.should_be_true();
    }
}
