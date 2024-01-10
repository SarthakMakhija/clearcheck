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

### Adding clearcheck as a dependency in Cargo.toml 

### Assertions vs Matchers

### List of assertions 

### Composing matchers 

### Writing custom assertions and matchers

### Features