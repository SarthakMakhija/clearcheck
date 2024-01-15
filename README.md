<p align="center">
    <img alt="clearcheck" src="https://github.com/SarthakMakhija/clearcheck/assets/21108320/eb2c2c8a-fc7b-4e82-9ef3-0c43812abf69" />
</p>  

[![clearcheck](https://github.com/SarthakMakhija/clearcheck/actions/workflows/build.yml/badge.svg)](https://github.com/SarthakMakhija/clearcheck/actions/workflows/build.yml) 

### Table of content

- [Introducing clearcheck](#introducing-clearcheck)
- [Key features](#key-features)
- [Usage](#usage)
- [Assertions vs Matchers](#assertions-vs-matchers)
- [Supported assertions ](#supported-assertions)
    * [Bool](#bool)
        + [Assertions](#assertions)
        + [Usage](#usage-1)
    * [Char](#char)
        + [Assertions](#assertions-1)
        + [Usage](#usage-2)
    * [Collections (Vector, Arrays, Slices)](#collections-vector-arrays-slices)
        + [Assertions](#assertions-2)
        + [Size based assertions](#size-based-assertions)
        + [Usage](#usage-3)
    * [Date](#date-enabled-by-date-feature-depends-on-chrono)
        + [Assertions](#assertions-3)
        + [Usage](#usage-4)
    * [Filepath](#filepath-enabled-by-file-feature-depends-on-walkdir)
        + [Assertions](#assertions-4)
        + [Usage](#usage-5)
    * [Float](#float-enabled-by-num-feature-depends-on-num)
        + [Assertions](#assertions-5)
        + [Usage](#usage-6)
    * [Integer](#integer-enabled-by-num-feature-depends-on-num)
        + [Assertions](#assertions-6)
        + [Usage](#usage-7)
    * [HashMap](#hashmap)
        + [Assertions](#assertions-7)
        + [Usage](#usage-8)
    * [Option](#option)
        + [Assertions](#assertions-8)
        + [Usage](#usage-9)
    * [Result](#result)
        + [Assertions](#assertions-9)
        + [Usage](#usage-10)
    * [T: PartialOrd](#t-partialord)
        + [Assertions](#assertions-10)
        + [Usage](#usage-11)
    * [T: Eq](#t-eq)
        + [Assertions](#assertions-11)
        + [Usage](#usage-12)
    * [String](#string)
        + [Assertions](#assertions-12)
        + [Length based assertions](#length-based-assertions)
        + [Usage](#usage-13)
- [Unleashing the power of custom matchers and assertions](#unleashing-the-power-of-custom-matchers-and-assertions)
- [Rust features](#rust-features)
- [Example project](#example-project)
- [Reference](#reference)
- [Mention](#mention)

### Introducing clearcheck

Write expressive and elagant assertions with ease!  

**clearcheck** is designed to make assertion statements as clear and concise as possible. It allows chaining multiple assertions together for a fluent and intuitive syntax, leading to more self-documenting test cases.

```rust
let pass_phrase = "P@@sw0rd1 zebra alpha";
pass_phrase.should_not_be_empty()
    .should_have_at_least_length(10)
    .should_contain_all_characters(vec!['@', ' '])
    .should_contain_a_digit()
    .should_not_contain_ignoring_case("pass")
    .should_not_contain_ignoring_case("word");
```

### Key features:

- **Fluent API**: Chain assertions for a natural and readable experience.
- **Extensive assertions**: Variety of assertions covering common validation needs.
- **Customizable**: Extend with your own assertions for specific domain requirements.
- **Type-safe**: Built with Rust's type system for reliable and expressive assertions.
- **Custom assertions**: Craft assertions tailored to your exact needs, ensuring comprehensive validations for various data structures.

### Usage

Add this to your `Cargo.toml` (no features):

```toml
[dev-dependencies]
clearcheck = { version = "0.0.1" }
```

Add this to your `Cargo.toml` (all features):

```toml
[dev-dependencies]
clearcheck = { version = "0.0.1", features = ["num", "date", "regex", "file"] }
chrono = { version = "0.4.31" }
num = { version = "0.4.1" }
regex = { version = "1.10.2" }
walkdir = { version = "2.4.0", features = [] }
```

### Assertions vs Matchers

Assertions serve as the cornerstone of the test cases, **defining the exact expectations the code must fulfill**. They act as a contract, ensuring that each data type (/data structure) adheres to its intended behavior.

Matchers, on the other hand, **provide the granular tools for carrying out these assertions**. They examine data and verify that the data conforms to specific criteria.

In essence, assertions orchestrate the high-level validation logic, while matchers act as the code-level inspectors, ensuring every detail aligns with the expectations.

### Supported assertions

#### Bool

##### Assertions

| **Assertion**   | **Description**                              |
|-----------------|----------------------------------------------|
| should_be_true  | Asserts that the boolean evaluates to true.  |
| should_be_false | Asserts that the boolean evaluates to false. |

##### Usage

```rust
let value = true;
value.should_be_true();
```

#### Char

##### Assertions

| **Assertion**                     | **Description**                                                               |
|-----------------------------------|-------------------------------------------------------------------------------|
| should_be_in_inclusive_range      | Asserts that the character falls within the given inclusive range.            |
| should_not_be_in_inclusive_range  | Asserts that the character does not fall within the given inclusive range.    |
| should_be_in_exclusive_range      | Asserts that the character falls within the given exclusive range.            |
| should_not_be_in_exclusive_range  | Asserts that the character does not fall within the given exclusive range.    |
| should_be_equal_ignoring_case     | Asserts that the character equals other character, with case ignored.         |
| should_not_be_equal_ignoring_case | Asserts that the character does not equal other character, with case ignored. |

##### Usage

```rust
let letter = 'd';
letter.should_be_in_inclusive_range('a'..='d');

let letter = 'D';
letter.should_be_equal_ignoring_case('d');
```

#### Collections (Vector, Arrays, Slices)

##### Assertions

| **Assertion**                           | **Description**                                                                                                                                                           |
|-----------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| should_have_upper_bound                 | Asserts that all elements in the collection are less than or equal to the given element.                                                                                  |
| should_have_lower_bound                 | Asserts that all elements in the collection are greater than or equal to the given element.                                                                               |
| should_contain_duplicates               | Asserts that the collection contains atleast one duplicate element.                                                                                                       |
| should_not_contain_duplicates           | Asserts that the collection does not contain any duplicate element.                                                                                                       |
| should_be_equal_ignoring_case           | Asserts that the elements in the collection are equal to those in other, ignoring case differences. (_Only applicable where elements can be represented as strings_).     |
| should_not_be_equal_ignoring_case       | Asserts that the elements in the collection are not equal to those in other, ignoring case differences. (_Only applicable where elements can be represented as strings_). |
| should_be_monotonically_increasing      | Asserts that the elements in the collection are in non-decreasing order (allowing consecutive equal elements).                                                            | 
| should_be_monotonically_decreasing      | Asserts that the elements in the collection are in non-increasing order (allowing consecutive equal elements).                                                            |
| should_be_strictly_increasing           | Asserts that the elements in the collection are in strictly increasing order (no consecutive elements can be equal).                                                      | 
| should_be_strictly_decreasing           | Asserts that the elements in the collection are in strictly decreasing order (no consecutive elements can be equal).                                                      | 
| should_contain                          | Asserts that the collection contains the given element.                                                                                                                   |
| should_not_contain                      | Asserts that the collection does not contain the given element.                                                                                                           |
| should_contain_all                      | Asserts that the collection contains all the given elements.                                                                                                              |
| should_not_contain_all                  | Asserts that the collection does not contain all the given elements.                                                                                                      |
| should_contain_any                      | Asserts that the collection contains any of the given elements.                                                                                                           |
| should_not_contain_any                  | Asserts that the collection does not contain any of the given elements.                                                                                                   |
| should_be_empty                         | Asserts that the collection is empty.                                                                                                                                     |
| should_not_be_empty                     | Asserts that the collection is not empty.                                                                                                                                 |

##### Size based assertions

| **Assertion**                           | **Description**                                                                                                                                                           |
|-----------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| should_have_size                        | Asserts that the size of the underlying collection is exactly the given size.                                                                                             |
| should_not_have_size                    | Asserts that the size of the underlying collection is not the given size.                                                                                                 |
| should_have_at_least_size               | Asserts that the size of the underlying collection is greater than or equal to the given size.                                                                            |
| should_have_at_most_size                | Asserts that the size of the underlying collection is less than or equal to the given size.                                                                               |
| should_be_same_size_as                  | Asserts that the size of the underlying collection is same as that of the given collection.                                                                               |
| should_have_size_in_inclusive_range     | Asserts that the size of the underlying collection falls within the given inclusive range.                                                                                |
| should_not_have_size_in_inclusive_range | Asserts that the size of the underlying collection does not fall within the given inclusive range.                                                                        |
| should_have_size_in_exclusive_range     | Asserts that the size of the underlying collection falls within the given exclusive range.                                                                                |
| should_not_have_size_in_exclusive_range | Asserts that the size of the underlying collection does not fall within the given exclusive range.                                                                        |

##### Usage

```rust
let keywords = ["testing", "automation", "clearcheck", "junit"];
keywords.should_not_be_empty()
    .should_have_size_in_inclusive_range(4..=10)
    .should_not_contain_duplicates()
    .should_contain_any(vec!["junit", "clearcheck", "testing"])
    .should_not_contain_any(vec!["scalatest", "gotest"]);
```

#### Date (enabled by 'date' feature, depends on [chrono](https://docs.rs/chrono/latest/chrono/))

##### Assertions

| **Assertion**                 | **Description**                                                        |
|-------------------------------|------------------------------------------------------------------------|
| should_have_same_year_as      | Asserts that the date has the same year as the other date.             |
| should_not_have_same_year_as  | Asserts that the date does not have the same year as the other date.   |
| should_have_year              | Asserts that the date has the same year as the given year.             |
| should_not_have_year          | Asserts that the date does not have the same year as the given year.   |
| should_have_same_month_as     | Asserts that the date has the same month as the other date.            |
| should_not_have_same_month_as | Asserts that the date does not have the same month as the other date.  |
| should_have_month             | Asserts that the date has the same month as the given month.           |
| should_not_have_month         | Asserts that the date does not have the same month as the given month. |
| should_have_same_day_as       | Asserts that the date has the same day as the other date.              |
| should_not_have_same_day_as   | Asserts that the date does not have the same day as the other date.    |
| should_have_day               | Asserts that the date has the same day as the given day.               |
| should_not_have_day           | Asserts that the date does not have the same day as the given day.     |
| should_be_a_leap_year         | Asserts that the date falls in a leap year.                            | 
| should_not_be_a_leap_year     | Asserts that the date does not fall in a leap year.                    |

##### Usage

```rust
use chrono::NaiveDate;

let date = NaiveDate::from_ymd_opt(2024, 1, 10).unwrap();
date
     .should_be_a_leap_year()
     .should_have_month(1)
     .should_be_greater_than(&NaiveDate::from_ymd_opt(2023, 1, 10).unwrap());
```

#### Filepath (enabled by 'file' feature, depends on [walkdir](https://docs.rs/walkdir/latest/walkdir/))

##### Assertions

| **Assertion**                        | **Description**                                                                                     |
|--------------------------------------|-----------------------------------------------------------------------------------------------------|
| should_be_a_directory                | Asserts that the path is a directory.                                                               |
| should_be_a_file                     | Asserts that the path is a file.                                                                    |
| should_be_a_symbolic_link            | Asserts that the path is a symbolic link.                                                           |
| should_be_zero_sized                 | Asserts that the path corresponds to a zero sized file.                                             |
| should_not_be_zero_sized             | Asserts that the path corresponds to a non-zero sized file.                                         |
| should_be_readonly                   | Asserts that the path corresponds to a readonly file.                                               |
| should_be_writable                   | Asserts that the path corresponds to a writable file.                                               |
| should_be_absolute                   | Asserts that the path is absolute.                                                                  |
| should_be_relative                   | Asserts that the path is relative.                                                                  |
| should_have_extension                | Asserts that the path corresponds to a file with the given extension.                               |
| should_not_have_extension            | Asserts that the path corresponds to a file that does not have the given extension.                 |
| should_contain_file_name             | Asserts that the path corresponds to a directory that contains the given file name.                 |
| should_not_contain_file_name         | Asserts that the path corresponds to a directory that does not contain the given file name.         |
| should_contain_all_file_names        | Asserts that the path corresponds to a directory that contains all the given file names.            |
| should_not_contain_all_file_names    | Asserts that the path corresponds to a directory that does not contain all the given file names.    |
| should_contain_any_of_file_names     | Asserts that the path corresponds to a directory that contains any of the given file names.         |
| should_not_contain_any_of_file_names | Asserts that the path corresponds to a directory that does not contain any of the given file names. |

##### Usage

```rust
use tempdir::TempDir;

let temporary_directory = TempDir::new(".").unwrap();
let file_path_junit = temporary_directory.path().join("junit.txt");
let file_path_clearcheck = temporary_directory.path().join("clearcheck.txt");

let _ = File::create(file_path_junit).unwrap();
let _ = File::create(file_path_clearcheck).unwrap();

let directory_path = temporary_directory.path();
directory_path
    .should_be_a_directory()
    .should_contain_any_of_file_names(vec!["junit.txt", "clearcheck.txt"]);
```

#### Float (enabled by 'num' feature, depends on [num](https://docs.rs/num/latest/num/))

##### Assertions

| **Assertion**                                   | **Description**                                                                                      |
|-------------------------------------------------|------------------------------------------------------------------------------------------------------|
| should_be_nan                                   | Asserts that the floating-point value is NaN (Not a Number).                                         |
| should_not_be_nan                               | Asserts that the floating-point value is not NaN (Not a Number).                                     |
| should_be_zero                                  | Asserts that the floating-point value is zero.                                                       |
| should_not_be_zero                              | Asserts that the floating-point value is not zero.                                                   |
| should_be_positive                              | Asserts that the floating-point value is positive.                                                   |
| should_be_negative                              | Asserts that the floating-point value is negative.                                                   |
| should_be_in_inclusive_range_with_tolerance     | Asserts that the floating-point value falls within the given inclusive range with tolerance.         |
| should_not_be_in_inclusive_range_with_tolerance | Asserts that the floating-point value does not fall within the given inclusive range with tolerance. |
| should_be_in_exclusive_range_with_tolerance     | Asserts that the floating-point value falls within the given exclusive range with tolerance.         |
| should_not_be_in_exclusive_range_with_tolerance | Asserts that the floating-point value does not fall within the given exclusive range with tolerance. |

##### Usage

```rust
let value: f64 = 1.34589;
value
   .should_not_be_nan()
   .should_be_positive()
   .should_be_in_inclusive_range_with_tolerance(1.11..=1.3458, 0.23);
```

#### Integer (enabled by 'num' feature, depends on [num](https://docs.rs/num/latest/num/))

##### Assertions

| **Assertion**      | **Description**                             |
|--------------------|---------------------------------------------|
| should_be_positive | Asserts that the integer value is positive. |
| should_be_negative | Asserts that the integer value is negative. |
| should_be_even     | Asserts that the integer value is even.     |
| should_be_odd      | Asserts that the integer value is odd.      |
| should_be_zero     | Asserts that the integer value is zero.     |
| should_not_be_zero | Asserts that the integer value is not zero. |

##### Usage

```rust
let value = 24;
value
    .should_be_positive()
    .should_be_even()
    .should_be_in_inclusive_range(10..=40);
```

#### HashMap

##### Assertions

| **Assertion**                    | **Description**                                                                      |
|----------------------------------|--------------------------------------------------------------------------------------|
| should_contain_key               | Asserts that the HashMap contains the given key.                                     |
| should_not_contain_key           | Asserts that the HashMap does not contain the given key.                             |
| should_contain_all_keys          | Asserts that the HashMap contains all the given keys.                                |
| should_not_contain_all_keys      | Asserts that the HashMap does not contain all the given keys.                        |
| should_contain_any_of_keys       | Asserts that the HashMap contains any of the given keys.                             |
| should_not_contain_any_of_keys   | Asserts that the HashMap does not contain any of the given keys.                     |
| should_contain_value             | Asserts that the HashMap contains the given value.                                   |
| should_not_contain_value         | Asserts that the HashMap does not contain the given value.                           |
| should_contain_all_values        | Asserts that the HashMap contains all the given values.                              |
| should_not_contain_all_values    | Asserts that the HashMap does not contain all the given values.                      |
| should_contain_any_of_values     | Asserts that the HashMap contains any of the given values.                           |
| should_not_contain_any_of_values | Asserts that the HashMap does not contain any of the given values.                   |
| should_contain                   | Asserts that the HashMap contains the given key and the value.                       |
| should_not_contain               | Asserts that the HashMap does not contain the given key and the value.               |
| should_contain_all               | Asserts that the HashMap contains all the entries from the given HashMap.            |
| should_not_contain_all           | Asserts that the HashMap does not contain all the entries from the given HashMap.    |
| should_contain_any               | Asserts that the HashMap contains any of the entries from the given HashMap.         |
| should_not_contain_any           | Asserts that the HashMap does not contain any of the entries from the given HashMap. |
| should_be_empty                  | Asserts that the HashMap is empty.                                                   |
| should_not_be_empty              | Asserts that the HashMap is not empty.                                               |
| +                                | [Size based assertions](#size-based-assertions).                                     |

##### Usage

```rust
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

let mut book_id_by_name = HashMap::new();
book_id_by_name.insert("Database internals", 1);
book_id_by_name.insert("Designing data intensive applications", 2);

book_id_by_name
    .should_not_be_empty()
    .should_contain_key("Database internals")
    .should_contain_value(&1)
    .should_have_at_least_size(2)
    .should_contain("Database internals", &1);
```

#### Option

##### Assertions

| **Assertion**  | **Description**                            |
|----------------|--------------------------------------------|
| should_be_some | Asserts that the Option evaluates to Some. |
| should_be_none | Asserts that the Option evaluates to None. |

##### Usage

```rust
let option = Some("clearcheck");
option.should_be_some();
```

#### Result

##### Assertions

| **Assertion** | **Description**                          |
|---------------|------------------------------------------|
| should_be_ok  | Asserts that the Result evaluates to Ok. |
| should_be_err | Result evaluates to Err.                 |

##### Usage

```rust
let value: Result<i32, &str> = Ok(32);
value.should_be_ok();
```

#### T: PartialOrd

##### Assertions

| **Assertion**                       | **Description**                                                                                                                 |
|-------------------------------------|---------------------------------------------------------------------------------------------------------------------------------|
| should_be_greater_than              | Asserts that the self value is greater than the given value (other) according to the PartialOrd implementation.                 |
| should_be_greater_than_equal_to     | Asserts that the self value is greater than or equal to the given value (other) according to the PartialOrd implementation.     |
| should_be_less_than                 | Asserts that the self value is less than the given value (other) according to the PartialOrd implementation.                    |
| should_be_less_than_equal_to        | Asserts that the self value is less than or equal to the given value (other) according to the PartialOrd implementation.        |
| should_not_be_greater_than          | Asserts that the self value is not greater than the given value (other) according to the PartialOrd implementation.             |
| should_not_be_greater_than_equal_to | Asserts that the self value is not greater than or equal to the given value (other) according to the PartialOrd implementation. |
| should_not_be_less_than             | Asserts that the self value is not less than the given value (other) according to the PartialOrd implementation.                |
| should_not_be_less_than_equal_to    | Asserts that the self value is not less than or equal to the given value (other) according to the PartialOrd implementation.    |
| should_be_in_inclusive_range        | Asserts that the self value falls within the given inclusive range.                                                             |
| should_not_be_in_inclusive_range    | Asserts that the self value does not fall within the given inclusive range.                                                     |
| should_be_in_exclusive_range        | Asserts that the self value falls within the given exclusive range.                                                             |
| should_not_be_in_exclusive_range    | Asserts that the self value does not fall within the given exclusive range.                                                     |

##### Usage

```rust
let value = 12.56;
value
    .should_be_greater_than(&10.90)
    .should_be_less_than(&15.98)
    .should_be_in_inclusive_range(10.90..=13.10);
```

#### T: Eq

##### Assertions

| **Assertion**    | **Description**                                            |
|------------------|------------------------------------------------------------|
| should_equal     | Asserts that the value held by self is equal to other.     |
| should_not_equal | Asserts that the value held by self is not equal to other. |

##### Usage

```rust
#[derive(Debug, Eq, PartialEq)]
struct Book { name: &'static str }

let books = vec![
  Book {name: "Database internals"},
  Book {name: "Rust in action"}
];

let other = vec![
  Book {name: "Database internals"},
  Book {name: "Rust in action"}
];

books.should_equal(&other);
```

#### String

##### Assertions

| **Assertion**                     | **Description**                                                                                                                                               |
|-----------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------|
| should_begin_with                 | Asserts that the string begins with the given prefix.                                                                                                         |
| should_not_begin_with             | Asserts that the string does not begin with the given prefix                                                                                                  |
| should_end_with                   | Asserts that the string ends with the given suffix.                                                                                                           |
| should_not_end_with               | Asserts that the string does not end with the given suffix.                                                                                                   |
| should_be_lower_case              | Asserts that the string is lowercase.                                                                                                                         |
| should_be_upper_case              | Asserts that the string is uppercase.                                                                                                                         |
| should_be_equal_ignoring_case     | Asserts that the string equals other string, with case ignored.                                                                                               | 
| should_not_be_equal_ignoring_case | Asserts that the string does not equal other string, with case ignored.                                                                                       |
| should_only_contain_digits        | Asserts that the string contains only digits.                                                                                                                 | 
| should_contain_a_digit            | Asserts that the string contains a digit.                                                                                                                     | 
| should_not_contain_digits         | Asserts that the string does not contain any digits.                                                                                                          |
| should_contain_character          | Asserts that the string contains the given character.                                                                                                         |
| should_not_contain_character      | Asserts that the string does not contain the given character.                                                                                                 |
| should_contain_all_characters     | Asserts that the string contains all the given characters.                                                                                                    |
| should_not_contain_all_characters | Asserts that the string does not contain all the given characters.                                                                                            |
| should_contain_any_characters     | Asserts that the string contains any of the given characters.                                                                                                 |
| should_not_contain_any_characters | Asserts that the string does not contain any of the given characters.                                                                                         |
| should_contain                    | Asserts that the string contains the given substring.                                                                                                         |
| should_not_contain                | Asserts that the string does not contain the given substring.                                                                                                 | 
| should_contain_ignoring_case      | Asserts that the string contains the substring, ignoring case differences.                                                                                    |
| should_not_contain_ignoring_case  | Asserts that the string does not contain the substring, ignoring case differences.                                                                            |
| should_be_empty                   | Asserts that the string is empty (has zero characters).                                                                                                       |
| should_not_be_empty               | Asserts that the string is not empty.                                                                                                                         |
| should_be_numeric                 | Asserts that the string is numeric.                                                                                                                           |
| should_not_be_numeric             | Asserts that the string is not numeric.                                                                                                                       |
| should_match                      | Asserts that the string matches the given regular expression.           (enabled by 'regex' feature, depends on [regex](https://docs.rs/regex/latest/regex/)) |
| should_not_match                  | Asserts that the string does not match the given regular expression.    (enabled by 'regex' feature, depends on [regex](https://docs.rs/regex/latest/regex/)) |

##### Length based assertions

| **Assertion**                             | **Description**                                                                       |
|-------------------------------------------|---------------------------------------------------------------------------------------|
| should_have_length                        | Asserts that the length of the string is exactly the given length.                    |
| should_not_have_length                    | Asserts that the length of the string is not the given length.                        |
| should_have_at_least_length               | Asserts that the length of the string is greater than or equal to the given length.   |
| should_have_at_most_length                | Asserts that the length of the string is less than or equal to the given length.      |
| should_have_length_in_inclusive_range     | Asserts that the length of the string falls within the given inclusive range.         |
| should_not_have_length_in_inclusive_range | Asserts that the length of the string does not fall within the given inclusive range. |
| should_have_length_in_exclusive_range     | Asserts that the length of the string falls within the given exclusive range.         |
| should_not_have_length_in_exclusive_range | Asserts that the length of the string does not fall within the given exclusive range. |

##### Usage

```rust
let pass_phrase = "P@@sw0rd1 zebra alpha";
pass_phrase.should_not_be_empty()
    .should_have_at_least_length(10)
    .should_contain_all_characters(vec!['@', ' '])
    .should_contain_a_digit()
    .should_not_contain_ignoring_case("pass")
    .should_not_contain_ignoring_case("word");
```

### Unleashing the power of custom matchers and assertions

While this crate comes loaded with a plethora of ready-made assertions, sometimes your testing needs demand a bespoke touch.
**clearcheck** allows crafting your own custom matchers and assertions!

The possibilities are endless:
- **Domain-specific validation**: Craft assertions that understand the nuances of your business logic.
- **Enhanced readability**: Write clear and concise matchers that mirror your domain vocabulary, making your tests self-documenting and understandable.
- **Reduced redundancy**: Eliminate repetitive code by encapsulating complex validation logic within reusable matchers.

Let's craft a custom password matcher with specific criteria like length, digits, and banned patterns.

```rust
fn be_a_valid_password<T: AsRef<str> + Debug>() -> Matchers<T> {
    MatchersBuilder::start_building_with_inverted(be_empty().boxed())
      .push(have_atleast_same_length(10).boxed())
      .push(contain_a_digit().boxed())
      .push(contain_any_of_characters(vec!['@', '#']).boxed())
      .push_inverted(begin_with("pass").boxed())
      .push_inverted(contain_ignoring_case("pass").boxed())
      .push_inverted(contain_ignoring_case("word").boxed())
      .combine_as_and()
}
```

The matcher enforces the following:
- Must not be empty.
- Must have a minimum length of 10 characters.
- Must contain at least one digit.
- Must contain any of the following characters: '@', '#'.
- Must not begin with the string "pass" (case-insensitive).
- Must not contain the strings "pass" or "word" (case-insensitive).

Let's combine it into a powerful assertion for valid passwords.

```rust
trait PasswordAssertion {
    fn should_be_a_valid_password(&self) -> &Self;
}

impl PasswordAssertion for &str {
    fn should_be_a_valid_password(&self) -> &Self {
        self.should(&be_a_valid_password());
        self
    }
}
```

Time to put our password assertion to use.

```rust
#[test]
fn should_be_a_valid_password() {
    let password = "P@@sw0rd9082";
    password.should_be_a_valid_password();
}
```

Let's consider that a password is valid, if:

- it satisfies the previous matcher conditions
- it is greater than the string "pass"

It might appear that we can combine an `OrderedMatcher` in the existing `be_a_valid_password` method.

A key idea behind **matcher compatibility** is that the matchers can only be combined if they work with the same data types. 

In this case, `OrderedMatcher` handles any partially ordered types `(T: PartialOrd)`, while other matchers in `be_a_valid_password` work with string slices (&str).

To get around this, a separate matcher, `greater_than_pass`, can be created to handle the "greater than 'pass'" condition. This matcher works with String values, ensuring compatibility.

```rust
fn greater_than_pass() -> Matchers<String> {
    MatchersBuilder::start_building(be_greater_than(String::from("pass")).boxed())
        .push_inverted(be_empty().boxed())
        .combine_as_and()
}
```

It returns an instance of `Matchers<String>`, so `PasswordAssertion` is now implemented for `String`. The method `should_be_a_valid_password`
now uses both the composed matchers to assert that the password is valid.

```rust
impl PasswordAssertion for String {
    fn should_be_a_valid_password(&self) -> &Self {
        self.should(&be_a_valid_password());
        self.should(&greater_than_pass());
        self
    }
}

#[test]
fn should_be_a_valid_password() {
    let password = "P@@sw0rd9082".to_string();
    password.should_be_a_valid_password();
}
```

### Rust features

**clearcheck** crate supports the following features:
- **date**  enables [assertions on date](#date-enabled-by-date-feature-depends-on-chrono)
- **file**  enables [assertions on filepath](#filepath-enabled-by-file-feature-depends-on-walkdir)
- **num**   enables [assertions on float](#float-enabled-by-num-feature-depends-on-num) and [assertions on integer](#integer-enabled-by-num-feature-depends-on-num)
- **regex** enables [regular expression assertions on string](#string)

### Example project

Head over to the [examples](https://github.com/SarthakMakhija/clearcheck-examples) project to understand the usage of this crate.

### Reference

- [kotest](https://github.com/kotest/kotest)

### Mention

- [Google bard](https://bard.google.com/chat) helped with the documentation and README :).
