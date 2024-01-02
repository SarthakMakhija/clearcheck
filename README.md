[![assert4rs](https://github.com/SarthakMakhija/assert4rs/actions/workflows/build.yml/badge.svg)](https://github.com/SarthakMakhija/assert4rs/actions/workflows/build.yml) 

### assert4rs

**assert4rs** offers powerful and elegant assertions for rust.

**Example**

```rust
let collection = vec!["junit", "assert4j", "catch2"];
        collection
            .should_not_be_empty()
            .should_have_at_least_size(2)
            .should_contain(&"assert4j");
```
