use assert4rs::assertions::collection::duplicate::DuplicateContentAssertions;
use assert4rs::assertions::collection::membership::MembershipAssertions;
use assert4rs::assertions::collection::size::SizeAssertions;

#[derive(Eq, Debug, PartialEq)]
struct Book {
    id: usize,
    title: &'static str,
}

impl Book {
    fn new(id: usize, title: &'static str) -> Self {
        Book { id, title }
    }
}

#[test]
fn should_match_all() {
    let library = vec![
        Book::new(1, "Database internals"),
        Book::new(2, "Designing data intensive applications"),
        Book::new(3, "Learning rust"),
        Book::new(4, "Rust in action"),
    ];

    library
        .should_not_be_empty()
        .should_not_contain_duplicates()
        .should_have_at_least_size(3)
        .should_contain_all(&[
            &Book::new(3, "Learning rust"),
            &Book::new(4, "Rust in action"),
        ]);
}

#[test]
#[should_panic]
fn should_not_match_all() {
    let library = vec![
        Book::new(1, "Database internals"),
        Book::new(2, "Designing data intensive applications"),
        Book::new(3, "Learning rust"),
        Book::new(4, "Rust in action"),
    ];

    library
        .should_not_be_empty()
        .should_have_at_least_size(3)
        .should_not_contain_duplicates()
        .should_contain_all(&[
            &Book::new(3, "Learning rust"),
            &Book::new(4, "Designing a KV storage engine"),
        ]);
}
