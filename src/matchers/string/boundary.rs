use crate::matchers::Matcher;

pub enum BoundaryBased<'a> {
    Begin(&'a str),
    End(&'a str),
}

impl<'a> Matcher<&str> for BoundaryBased<'a> {
    fn test(&self, value: &&str) -> bool {
        match self {
            BoundaryBased::Begin(prefix) => value.starts_with(prefix),
            BoundaryBased::End(suffix) => value.ends_with(suffix),
        }
    }
}

pub fn begin_with(prefix: &str) -> BoundaryBased<'_> {
    BoundaryBased::Begin(prefix)
}

pub fn end_with(suffix: &str) -> BoundaryBased<'_> {
    BoundaryBased::End(suffix)
}
