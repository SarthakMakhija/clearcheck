use core::fmt;

pub(crate) enum AssertKind {
    Empty,
    NotEmpty,
    Contains,
    NotContains,
    UpperBound,
    LowerBound,
    ContainsDuplicates,
    NotContainsDuplicates,
    SortsAscending,
    SortsDescending,
    EqualSize,
    NotEqualSize,
    AtleastSize,
    AtmostSize,
    ContainsValue,
    NotContainsValue,
    None,
    Some,
    Ok,
    Err,
    True,
    False,
    LowerCase,
    UpperCase,
    EqualLength,
    NotEqualLength,
    AtleastLength,
    AtmostLength,
    InRangeLength,
    NotInRangeLength,
    ContainsOnlyDigits,
    ContainsADigit,
    NotContainsDigits,
}

pub(crate) fn assert_failed_unary<T>(
    kind: AssertKind,
    left: &T,
) -> !
    where
        T: fmt::Debug + ?Sized, {
    assert_failed_inner(kind, &left, None)
}

pub(crate) fn assert_failed_binary<T, U>(
    kind: AssertKind,
    left: &T,
    right: &U,
) -> !
    where
        T: fmt::Debug + ?Sized,
        U: fmt::Debug + ?Sized, {
    assert_failed_inner(kind, &left, Some(&right))
}

fn assert_failed_inner(
    kind: AssertKind,
    left: &dyn fmt::Debug,
    right: Option<&dyn fmt::Debug>,
) -> ! {
    let operation = match kind {
        AssertKind::Empty => "must be empty",
        AssertKind::NotEmpty => "must not be empty",
        AssertKind::Contains => "must contain",
        AssertKind::NotContains => "must not contain",
        AssertKind::UpperBound => "must have an upper-bound",
        AssertKind::LowerBound => "must have a lower-bound",
        AssertKind::ContainsDuplicates => "must contain duplicates",
        AssertKind::NotContainsDuplicates => "must not contain duplicates",
        AssertKind::SortsAscending => "must be sorted in ascending order",
        AssertKind::SortsDescending => "must be sorted in descending order",
        AssertKind::EqualSize => "must have the same size as",
        AssertKind::NotEqualSize => "must not have the same size as",
        AssertKind::AtleastSize => "must have at least the same size as",
        AssertKind::AtmostSize => "must have at most the same size as",
        AssertKind::ContainsValue => "must contain the value of",
        AssertKind::NotContainsValue => "must not contain the value of",
        AssertKind::None => "must be none",
        AssertKind::Some => "must be some",
        AssertKind::Ok => "must be ok",
        AssertKind::Err => "must be error",
        AssertKind::True => "must be true",
        AssertKind::False => "must be false",
        AssertKind::LowerCase => "must be lowercase",
        AssertKind::UpperCase => "must be uppercase",
        AssertKind::EqualLength => "must have the same length as",
        AssertKind::NotEqualLength => "must not have the same length as",
        AssertKind::AtleastLength => "must have at least the same length as",
        AssertKind::AtmostLength => "must have at most the same length as",
        AssertKind::InRangeLength => "must have the length in the range provided by",
        AssertKind::NotInRangeLength => "must not have the length in the range provided by",
        AssertKind::ContainsOnlyDigits => "must only contain digits",
        AssertKind::ContainsADigit => "must contain a digit",
        AssertKind::NotContainsDigits => "must not contain a digit",
    };

    match right {
        Some(right) => panic!(
            r#"assertion failed: `(left {} right)`
  left: `{:?}`,
 right: `{:?}`"#,
            operation, left, right
        ),
        None => panic!(
            r#"assertion failed: `(left {})`
  left: `{:?}`"#,
            operation, left,
        ),
    }
}
