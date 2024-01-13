<p align="center">
    <img alt="clearcheck" src="https://github.com/SarthakMakhija/clearcheck/assets/21108320/eb2c2c8a-fc7b-4e82-9ef3-0c43812abf69" />
</p>  

[![clearcheck](https://github.com/SarthakMakhija/clearcheck/actions/workflows/build.yml/badge.svg)](https://github.com/SarthakMakhija/clearcheck/actions/workflows/build.yml) 

### clearcheck

**clearcheck** offers elegant and extensible assertions for rust.

### Example

```rust
let libraries = vec!["clearcheck", "gotest", "junit", "scalatest"];
libraries
    .should_not_be_empty()
    .should_not_contain_duplicates()
    .should_contain("clearcheck")
    .should_be_sorted_ascending();
```

### Why clearcheck?

### Usage

Add this to your `Cargo.toml` without any features:

```toml
[dev-dependencies]
clearcheck = { version = "0.0.1" }
```

Add this to your `Cargo.toml` with all the features:

```toml
[dev-dependencies]
clearcheck = { version = "0.0.1", features = ["num", "date", "regex", "file"] }
chrono = { version = "0.4.31" }
num = { version = "0.4.1" }
regex = { version = "1.10.2" }
walkdir = { version = "2.4.0", features = [] }
```

### Assertions vs Matchers

### List of assertions 

### Composing matchers 

### Writing custom assertions and matchers

### Features

### Example project
[clearcheck examples](https://github.com/SarthakMakhija/clearcheck-examples)
