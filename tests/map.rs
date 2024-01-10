use std::collections::HashMap;

use clearcheck::assertions::collection::size::SizeAssertion;
use clearcheck::assertions::map::membership::{
    KeyMembershipAssertion, KeyValueMembershipAssertion, NoMembershipAssertion,
    ValueMembershipAssertion,
};

#[derive(Eq, Debug, PartialEq, Hash)]
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
fn should_match_all_books_by_name() {
    let mut book_id_by_name = HashMap::new();
    book_id_by_name.insert("Database internals", 1);
    book_id_by_name.insert("Designing data intensive applications", 2);

    book_id_by_name
        .should_not_be_empty()
        .should_contain_key("Database internals")
        .should_contain_value(&1)
        .should_have_at_least_size(2)
        .should_contain("Database internals", &1);
}

#[test]
fn should_match_all_books() {
    let mut book_rank_by_name = HashMap::new();
    book_rank_by_name.insert(Book::new(20, "Patterns of Distributed Systems"), 1);
    book_rank_by_name.insert(Book::new(21, "Designing data intensive applications"), 2);
    book_rank_by_name.insert(Book::new(21, "Database internals"), 3);

    book_rank_by_name
        .should_not_be_empty()
        .should_contain_key(&Book::new(20, "Patterns of Distributed Systems"))
        .should_not_contain_key(&Book::new(25, "Rust in action"))
        .should_have_size_in_inclusive_range(1..=5)
        .should_contain_any_of_values(&[&1, &3, &4]);
}
