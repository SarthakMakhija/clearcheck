use core::fmt;

pub(crate) enum AssertKind {
    Empty,
    NotEmpty,
    Contains,
    NotContains,
    UpperBound,
    LowerBound,
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
    let op = match kind {
        AssertKind::Empty => "must be empty",
        AssertKind::NotEmpty => "must not be empty",
        AssertKind::Contains => "must contain",
        AssertKind::NotContains => "must not contain",
        AssertKind::UpperBound => "must have an upper-bound",
        AssertKind::LowerBound => "must have a lower-bound",
    };

    match right {
        Some(right) => panic!(
            r#"assertion failed: `(left {} right)`
  left: `{:?}`,
 right: `{:?}`"#,
            op, left, right
        ),
        None => panic!(
            r#"assertion failed: `(left {})`
  left: `{:?}`"#,
            op, left,
        ),
    }
}
