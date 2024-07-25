# assert_eq_ignoring

`assert_eq_ignoring` is a Rust macro designed to assert that two values are equal, while ignoring specified fields.  
This macro is useful for comparing complex structures where certain fields may differ but are not relevant to the equality check.

## Features

- Allows exclusion of specified fields from the equality check.
- Supports flexible comparisons of complex data structures.

## Getting Started

### Installation

Add `assert_eq_ignoring` to your `Cargo.toml`:

```toml
[dependencies]
assert_eq_ignoring = "0.1.0"
```

## Usage

```rust
use assert_eq_ignoring::*;
use getset::Setters;

// Debug and PartialEq required for assert_eq!
// Clone and Setters required for assert_eq_ignoring!
#[derive(Debug, PartialEq, Clone, Setters)]
struct User {
    id: u32,
    name: String,
    age: u32,
}

impl User {
    fn new(id: u32, name: &str, age: u32) -> Self {
        User { id, name: name.to_string(), age }
    }
}

fn main() {
    let user1 = User { id: 1, name: "Alice".to_string(), age: 10 };
    let user2 = User { id: 1, name: "Alice".to_string(), age: 20 };

    // This assertion will pass because the `age` field is ignored in the comparison.
    assert_eq_ignoring!(user1, user2, age);
    // This assertion will also pass. It ignores both `id` and `age` fields,
    // and includes a custom description for the assertion.
    assert_eq_ignoring!(user1, user2, id, age, "Same user except age");
}
```

### Advanced Usage

If assertions are only executed in tests, then it is sufficient for the setters to be defined under a test feature.  
Therefore, it is preferable to define them as follows:

```rust
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(test, derive(Setters))]
#[cfg_attr(test, set = "pub")]
struct User {
    id: u32,
    name: String,
    age: usize,
}
```

## License

This project is licensed under the MIT License.  
For more information on the license, please see the <a href="LICENSE">license</a>.
