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

### Composing matchers 

### Writing custom assertions and matchers

### Features

### Example project

Head over to the [examples](https://github.com/SarthakMakhija/clearcheck-examples) project to understand the usage of this crate.
