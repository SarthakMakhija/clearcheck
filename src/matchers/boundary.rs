use crate::matchers::Matcher;

pub enum StringBoundaryBased<'a> {
    Begin(&'a str),
    End(&'a str),
}

impl<'a> Matcher<&str> for StringBoundaryBased<'a> {
    fn test(&self, value: &&str) -> bool {
        match self {
            StringBoundaryBased::Begin(prefix) => value.starts_with(prefix),
            StringBoundaryBased::End(suffix) => value.ends_with(suffix),
        }
    }
}

pub fn begin_with(prefix: &str) -> StringBoundaryBased<'_> {
    StringBoundaryBased::Begin(prefix)
}

pub fn end_with(suffix: &str) -> StringBoundaryBased<'_> {
    StringBoundaryBased::End(suffix)
}
