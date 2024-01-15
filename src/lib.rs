//! # Clearcheck: Elegant and extensible assertions in Rust
//!
//! clearcheck is designed to make assertion statements as clear and concise as possible.
//! It allows chaining multiple assertions together for a fluent and intuitive syntax, leading to more self-documenting test cases.
//!
//!```rust
//! use clearcheck::assertions::string::length::LengthAssertion;
//! use clearcheck::assertions::string::membership::MembershipAssertion;
//! use clearcheck::assertions::string::numeric::NumericAssertion;
//! use clearcheck::assertions::string::regex::RegularExpressionAssertion;
//!
//! let pass_phrase = "P@@sw0rd1 zebra alpha";
//! pass_phrase.should_not_be_empty()
//!     .should_have_at_least_length(10)
//!     .should_contain_all_characters(vec!['@', ' '])
//!     .should_contain_a_digit()
//!     .should_not_contain_ignoring_case("pass")
//!     .should_not_contain_ignoring_case("word");
//!```
//! # Key features
//!
//! - **Fluent API**: Chain assertions for a natural and readable experience.
//! - **Extensive assertions**: Variety of assertions covering common validation needs.
//! - **Customizable**: Extend with your own assertions for specific domain requirements.
//! - **Type-safe**: Built with Rust's type system for reliable and expressive assertions.
//! - **Custom assertions**: Craft assertions tailored to your exact needs, ensuring comprehensive validations for various data structures.
//!
//! # Rust features
//!
//! clearcheck crate supports the following features:
//! - date enables [assertions on date](assertions::date::DateAssertion)
//! - file enables [assertions on filepath](assertions::file::FileAssertion)
//! - num enables [assertions on float](assertions::float::FloatAssertion) and [assertions on integer](assertions::int::IntAssertion)
//! - regex enables [regular expression assertions on string](assertions::string::regex)
//!
//! # Assertions vs Matchers
//!
//! **Assertions** serve as the cornerstone of the test cases, defining the exact expectations the code must fulfill.
//! They act as a contract, ensuring that each data type (/data structure) adheres to its intended behavior.
//!
//! **Matchers**, on the other hand, provide the granular tools for carrying out these assertions.
//! They examine data and verify that the data conforms to specific criteria.
//!
//! In essence, assertions orchestrate the high-level validation logic, while matchers act as the
//! code-level inspectors, ensuring every detail aligns with the expectations.
//!
//! # Unleashing the power of custom matchers and assertions
//!
//! While this crate comes loaded with a plethora of ready-made assertions, sometimes your testing needs demand a bespoke touch.
//! clearcheck allows crafting your own custom matchers and assertions!
//!
//! The possibilities are endless:
//!
//! - **Domain-specific validation**: Craft assertions that understand the nuances of your business logic.
//! - **Enhanced readability**: Write clear and concise matchers that mirror your domain vocabulary, making your tests self-documenting and understandable.
//! - **Reduced redundancy**: Eliminate repetitive code by encapsulating complex validation logic within reusable matchers.
//!
//! Let's craft a custom password matcher that enforces the following:
//!
//! - Password must not be empty.
//! - Password must have a minimum length of 10 characters.
//! - Password must contain at least one digit.
//! - Password must contain any of the following characters: '@', '#'.
//! - Password must not begin with the string "pass" (case-insensitive).
//! - Password must not contain the strings "pass" or "word" (case-insensitive).
//!
//!```rust
//! //1. Write a matcher.
//! use std::fmt::Debug;
//!
//! use clearcheck::matchers::{BoxWrap, Should};
//! use clearcheck::matchers::compose::{Matchers, MatchersBuilder};
//! use clearcheck::matchers::string::boundary::begin_with;
//! use clearcheck::matchers::string::empty::be_empty;
//! use clearcheck::matchers::string::length::have_atleast_same_length;
//! use clearcheck::matchers::string::membership::{
//!    contain_a_digit,
//!    contain_any_of_characters,
//!    contain_ignoring_case
//! };
//!
//! fn be_a_valid_password<T: AsRef<str> + Debug>() -> Matchers<T> {
//!     MatchersBuilder::start_building_with_inverted(be_empty().boxed())
//!       .push(have_atleast_same_length(10).boxed())
//!       .push(contain_a_digit().boxed())
//!       .push(contain_any_of_characters(vec!['@', '#']).boxed())
//!       .push_inverted(begin_with("pass").boxed())
//!       .push_inverted(contain_ignoring_case("pass").boxed())
//!       .push_inverted(contain_ignoring_case("word").boxed())
//!       .combine_as_and()
//! }
//!
//! //2. Combine the matcher into a powerful assertion for valid passwords.
//! trait PasswordAssertion {
//!     fn should_be_a_valid_password(&self) -> &Self;
//! }
//!
//! impl PasswordAssertion for &str {
//!     fn should_be_a_valid_password(&self) -> &Self {
//!         self.should(&be_a_valid_password());
//!         self
//!     }
//! }
//!
//! //3. That's it. Use the password assertion.
//! #[test]
//! fn should_be_a_valid_password() {
//!     let password = "P@@sw0rd9082";
//!     password.should_be_a_valid_password();
//! }
//! ```

pub mod assertions;
pub mod matchers;