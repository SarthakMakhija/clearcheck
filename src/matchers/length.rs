pub enum LengthBased {
    Same(usize),
    Atleast(usize),
    Atmost(usize),
    Zero,
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
