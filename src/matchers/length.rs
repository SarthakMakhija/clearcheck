pub enum LengthBased {
    Same(usize),
    Atleast(usize),
    Atmost(usize),
    Zero,
}

impl LengthBased {
    pub fn test_length(&self, input_length: usize) -> bool {
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
