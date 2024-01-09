[![assert4rs](https://github.com/SarthakMakhija/assert4rs/actions/workflows/build.yml/badge.svg)](https://github.com/SarthakMakhija/assert4rs/actions/workflows/build.yml) 

### assert4rs

**assert4rs** offers powerful and elegant assertions for rust.

### Example

```rust
let libraries = vec!["assert4rs", "gotest", "junit", "scalatest"];
libraries
    .should_not_be_empty()
    .should_not_contain_duplicates()
    .should_contain("assert4rs")
    .should_be_sorted_ascending();
```

### Why assert4rs?

### Adding assert4rs as a dependency in Cargo.toml 

### Assertions vs Matchers

### List of Assertions 

### Composing Matchers 

### Writing Custom Assertions and Matchers

### Features