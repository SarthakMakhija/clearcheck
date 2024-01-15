//! Assertions serve as the cornerstone of the test cases, defining the exact expectations the code must fulfill.
//! They act as a contract, ensuring that each data type (/data structure) adheres to its intended behavior.
//! clearcheck provides plethora of ready-made assertions.
//!
//! Let's take an example.
//!
//! ```rust
//! use clearcheck::assertions::collection::duplicate::DuplicateContentAssertion;
//! use clearcheck::assertions::collection::membership::MembershipAssertion;
//! use clearcheck::assertions::collection::size::SizeAssertion;
//!
//! let keywords = ["testing", "automation", "clearcheck", "junit"];
//! keywords.should_not_be_empty()
//!     .should_have_size_in_inclusive_range(4..=10)
//!     .should_not_contain_duplicates()
//!     .should_contain_any(vec!["junit", "clearcheck", "testing"])
//!     .should_not_contain_any(vec!["scalatest", "gotest"]);
//! ```

pub mod bool;
pub mod char;
pub mod collection;
#[cfg(feature = "date")]
pub mod date;
pub mod equal;
#[cfg(feature = "file")]
pub mod file;
#[cfg(feature = "num")]
pub mod float;
#[cfg(feature = "num")]
pub mod int;
pub mod map;
pub mod option;
pub mod ordered;
pub mod result;
pub mod string;
