# selective_assertions

`selective_assertions` is a Rust crate that provides macros for flexible assertions in tests. It includes `assert_eq_excluding` and `assert_eq_selected`, which allow you to compare complex structures while ignoring specified fields or only comparing specified fields.

## Features

- **assert_eq_excluding**: Asserts that two values are equal, while ignoring specified fields.
- **assert_eq_selected**: Asserts that specific fields of two values are equal.

## Getting Started

### Installation

Add `selective_assertions` to your `Cargo.toml`:

```toml
[dependencies]
selective_assertions = "0.1.0"
```

## Usage

### assert_eq_excluding

The assert_eq_excluding macro allows you to assert that two values are equal while ignoring specified fields. This is useful for comparing complex structures where certain fields may differ but are not relevant to the equality check.

```rust
use derive_getters::Getters;
use getset::Setters;
use selective_assertions::*;

// Debug and PartialEq are required for assert_eq!
// Clone, Getters, and Setters are required for assert_eq_excluding!
#[derive(Debug, PartialEq, Clone, Getters, Setters)]
#[set = "pub"]
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

    // This assertion will pass because the `age` field is excluded in the comparison.
    assert_eq_excluding!(user1, user2, age);
}
```

### assert_eq_selected

The assert_eq_selected macro allows you to assert that specific fields of two values are equal. This is useful for focusing on comparing only the specified fields without needing to compare the entire struct or object.

```rust
use derive_getters::Getters;
use selective_assertions::*;

// Debug and PartialEq are required for assert_eq!
// Getters is required for assert_eq_excluding!
#[derive(Debug, PartialEq, Getters)]
struct User {
    id: u32,
    name: String,
    age: u8,
}

fn main() {
    let user1 = User { id: 1, name: String::from("Alice"), age: 7 };
    let user2 = User { id: 1, name: String::from("Alice"), age: 8 };

    // Compare user1 and user2, focusing only on the `id` and `name` fields
    assert_eq_selected!(user1, user2, id, name);
}
```

## License

This project is licensed under the MIT License.
For more information on the license, please see the <a href="LICENSE">license</a>.
