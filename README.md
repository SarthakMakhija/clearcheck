<p align="center">
    <img alt="clearcheck" src="https://github.com/SarthakMakhija/clearcheck/assets/21108320/eb2c2c8a-fc7b-4e82-9ef3-0c43812abf69" />
</p>  

[![clearcheck](https://github.com/SarthakMakhija/clearcheck/actions/workflows/build.yml/badge.svg)](https://github.com/SarthakMakhija/clearcheck/actions/workflows/build.yml) 

### Introducting clearcheck

Write expressive and readable assertions with ease!  

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

#### Key Features:

- **Fluent API**: Chain assertions for a natural and readable experience.
- **Extensive assertions**: Variety of assertions covering common validation needs.
- **Customizable**: Extend with your own assertions for specific domain requirements.
- **Type-Safe**: Built with Rust's type system for reliable and expressive assertions.
- **Custom Matchers**: Craft assertions tailored to your exact needs, ensuring comprehensive validation for unique data structures and logic.

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

### Supported assertions 

#### Boolean assertions

| **Assertion**   | **Description**                              |
|-----------------|----------------------------------------------|
| should_be_true  | Asserts that the boolean evaluates to true.  |
| should_be_false | Asserts that the boolean evaluates to false. |

#### Usage

```rust
let value = true;
value.should_be_true();
```

#### Char assertions

| **Assertion**     | **Description**                              |
|-------------------|----------------------------------------------|
| should_be_in_inclusive_range  | Asserts that the character falls within the given inclusive range.  |
| should_not_be_in_inclusive_range | Asserts that the character does not fall within the given inclusive range. |
| should_be_in_exclusive_range | Asserts that the character falls within the given exclusive range. |
| should_not_be_in_exclusive_range | Asserts that the character does not fall within the given exclusive range. |
| should_be_equal_ignoring_case | Asserts that the character equals other character, with case ignored. |
| should_not_be_equal_ignoring_case | Asserts that the character does not equal other character, with case ignored. |

#### Usage

```rust
let letter = 'd';
letter.should_be_in_inclusive_range('a'..='d');

let letter = 'D';
letter.should_be_equal_ignoring_case('d');
```

#### Collection assertions (Vector, Arrays, Slices)

| **Assertion**     | **Description**                              |
|-------------------|----------------------------------------------|
| should_have_upper_bound  | Asserts that all elements in the collection are less than or equal to the given element. |
| should_have_lower_bound | Asserts that all elements in the collection are greater than or equal to the given element. |
| should_contain_duplicates | Asserts that the collection contains atleast one duplicate element. |
| should_not_contain_duplicates | Asserts that the collection does not contain any duplicate element. |
| should_be_equal_ignoring_case | Asserts that the elements in the collection are equal to those in other, ignoring case differences. (_Only applicable where elements can be represented as strings_). |
| should_not_be_equal_ignoring_case | Asserts that the elements in the collection are not equal to those in other, ignoring case differences. (_Only applicable where elements can be represented as strings_). |
| should_be_monotonically_increasing | Asserts that the elements in the collection are in non-decreasing order (allowing consecutive equal elements). | 
| should_be_monotonically_decreasing | Asserts that the elements in the collection are in non-increasing order (allowing consecutive equal elements). |
| should_be_strictly_increasing | Asserts that the elements in the collection are in strictly increasing order (no consecutive elements can be equal). | 
| should_be_strictly_decreasing | Asserts that the elements in the collection are in strictly decreasing order (no consecutive elements can be equal). | 
| should_contain | Asserts that the collection contains the given element. |
| should_not_contain | Asserts that the collection does not contain the given element. |
| should_contain_all | Asserts that the collection contains all the given elements. |
| should_not_contain_all | Asserts that the collection does not contain all the given elements. |
| should_contain_any | Asserts that the collection contains any of the given elements. |
| should_not_contain_any | Asserts that the collection does not contain any of the given elements. |
| should_be_empty | Asserts that the collection is empty. |
| should_not_be_empty | Asserts that the collection is not empty. |
| should_have_size | Asserts that the size of the underlying collection is exactly the given size. | 
| should_not_have_size | Asserts that the size of the underlying collection is not the given size. |
| should_have_at_least_size | Asserts that the size of the underlying collection is greater than or equal to the given size. |
| should_have_at_most_size | Asserts that the size of the underlying collection is less than or equal to the given size. |
| should_be_same_size_as | Asserts that the size of the underlying collection is same as that of the given collection. |
| should_have_size_in_inclusive_range | Asserts that the size of the underlying collection falls within the given inclusive range. |
| should_not_have_size_in_inclusive_range | Asserts that the size of the underlying collection does not fall within the given inclusive range. |
| should_have_size_in_exclusive_range | Asserts that the size of the underlying collection falls within the given exclusive range. |
| should_not_have_size_in_exclusive_range | Asserts that the size of the underlying collection does not fall within the given exclusive range. |
| should_be_sorted_ascending | Asserts that the elements of the collection are in ascending order (non-decreasing, allowing duplicates). | 
| should_be_sorted_descending | Asserts that the elements of the collection are in descending order (non-increasing, allowing duplicates). |

#### Usage

```rust
let keywords = ["testing", "automation", "clearcheck", "junit"];
keywords.should_not_be_empty()
    .should_have_size_in_inclusive_range(4..=10)
    .should_not_contain_duplicates()
    .should_contain_any(vec!["junit", "clearcheck", "testing"])
    .should_not_contain_any(vec!["scalatest", "gotest"]);
```

#### Date assertions (Enabled by 'date' feature, depends on [chrono](https://docs.rs/chrono/latest/chrono/))

| **Assertion**     | **Description**                              |
|-------------------|----------------------------------------------|
| should_have_same_year_as  | Asserts that the date has the same year as the other date.  |
| should_not_have_same_year_as | Asserts that the date does not have the same year as the other date. |
| should_have_year | Asserts that the date has the same year as the given year. |
| should_not_have_year | Asserts that the date does not have the same year as the given year. |
| should_have_same_month_as | Asserts that the date has the same month as the other date. |
| should_not_have_same_month_as | Asserts that the date does not have the same month as the other date. |
| should_have_month | Asserts that the date has the same month as the given month. |
| should_not_have_month | Asserts that the date does not have the same month as the given month. |
| should_have_same_day_as | Asserts that the date has the same day as the other date. |
| should_not_have_same_day_as | Asserts that the date does not have the same day as the other date. |
| should_have_day | Asserts that the date has the same day as the given day. |
| should_not_have_day | Asserts that the date does not have the same day as the given day. |
| should_be_a_leap_year | Asserts that the date falls in a leap year. | 
| should_not_be_a_leap_year | Asserts that the date does not fall in a leap year. |

#### Usage

```rust
use chrono::NaiveDate;

let date = NaiveDate::from_ymd_opt(2024, 1, 10).unwrap();
date
     .should_be_a_leap_year()
     .should_have_month(1)
     .should_be_greater_than(&NaiveDate::from_ymd_opt(2023, 1, 10).unwrap());
```

#### Filepath assertions (Enabled by 'file' feature, depends on [walkdir](https://docs.rs/walkdir/latest/walkdir/))

| **Assertion**     | **Description**                              |
|-------------------|----------------------------------------------|
| should_be_a_directory  | Asserts that the path is a directory. |
| should_be_a_file | Asserts that the path is a file. |
| should_be_a_symbolic_link | Asserts that the path is a symbolic link. |
| should_be_zero_sized | Asserts that the path corresponds to a zero sized file. |
| should_not_be_zero_sized | Asserts that the path corresponds to a non-zero sized file. |
| should_be_readonly | Asserts that the path corresponds to a readonly file. |
| should_be_writable | Asserts that the path corresponds to a writable file. |
| should_be_absolute | Asserts that the path is absolute. |
| should_be_relative | Asserts that the path is relative. |
| should_have_extension | Asserts that the path corresponds to a file with the given extension. |
| should_not_have_extension | Asserts that the path corresponds to a file that does not have the given extension. |
| should_contain_file_name | Asserts that the path corresponds to a directory that contains the given file name. |
| should_not_contain_file_name | Asserts that the path corresponds to a directory that does not contain the given file name. |
| should_contain_all_file_names | Asserts that the path corresponds to a directory that contains all the given file names. |
| should_not_contain_all_file_names | Asserts that the path corresponds to a directory that does not contain all the given file names. |
| should_contain_any_of_file_names | Asserts that the path corresponds to a directory that contains any of the given file names. |
| should_not_contain_any_of_file_names | Asserts that the path corresponds to a directory that does not contain any of the given file names. |

#### Usage

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

#### Float assertions (Enabled by 'num' feature, depends on [num](https://docs.rs/num/latest/num/))

| **Assertion**     | **Description**                              |
|-------------------|----------------------------------------------|
| should_be_nan  | Asserts that the floating-point value is NaN (Not a Number). |
| should_not_be_nan | Asserts that the floating-point value is not NaN (Not a Number). |
| should_be_zero | Asserts that the floating-point value is zero. |
| should_not_be_zero | Asserts that the floating-point value is not zero. |
| should_be_positive | Asserts that the floating-point value is positive. |
| should_be_negative | Asserts that the floating-point value is negative. |
| should_be_in_inclusive_range_with_tolerance | Asserts that the floating-point value falls within the given inclusive range with tolerance. |
| should_not_be_in_inclusive_range_with_tolerance | Asserts that the floating-point value does not fall within the given inclusive range with tolerance. |
| should_be_in_exclusive_range_with_tolerance | Asserts that the floating-point value falls within the given exclusive range with tolerance. |
| should_not_be_in_exclusive_range_with_tolerance | Asserts that the floating-point value does not fall within the given exclusive range with tolerance. |

#### Usage

```rust
let value: f64 = 1.34589;
value
 .should_not_be_nan()
 .should_be_positive()
 .should_be_in_inclusive_range_with_tolerance(1.11..=1.3458, 0.23);
```


### Composing matchers 

### Writing custom assertions and matchers

### Features

### Example project

Head over to the [examples](https://github.com/SarthakMakhija/clearcheck-examples) project to understand the usage of this crate.
