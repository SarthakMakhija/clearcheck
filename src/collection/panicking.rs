use core::fmt;

pub(crate) enum AssertKind {
    Empty,
    NotEmpty,
}

pub(crate) fn assert_failed_unary<T>(
    kind: AssertKind,
    left: &T,
) -> !
    where
        T: fmt::Debug + ?Sized, {
    assert_failed_inner(kind, &left, None)
}

fn assert_failed_inner(
    kind: AssertKind,
    left: &dyn fmt::Debug,
    right: Option<&dyn fmt::Debug>,
) -> ! {
    let op = match kind {
        AssertKind::Empty => "left must be empty",
        AssertKind::NotEmpty => "left must not be empty",
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
