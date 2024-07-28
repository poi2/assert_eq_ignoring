# selective_assert

`selective_assert` is a Rust crate that provides macros for flexible assertions in tests. It includes `assert_eq_ignoring` and `assert_eq_only`, which allow you to compare complex structures while ignoring specified fields or only comparing specified fields.

## Features

- **assert_eq_ignoring**: Asserts that two values are equal, while ignoring specified fields.
- **assert_eq_only**: Asserts that specific fields of two values are equal.

## Getting Started

### Installation

Add `selective_assert` to your `Cargo.toml`:

````toml
[dependencies]
selective_assert = "0.1.0"

## Usage

### assert_eq_ignoring

The assert_eq_ignoring macro allows you to assert that two values are equal while ignoring specified fields. This is useful for comparing complex structures where certain fields may differ but are not relevant to the equality check.

```rust
use selective_assert::*;
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
}
```

### assert_eq_only

The assert_eq_only macro allows you to assert that specific fields of two values are equal. This is useful for focusing on comparing only the specified fields without needing to compare the entire struct or object.

```rust
use selective_assert::*;

struct User {
    id: u32,
    name: String,
    age: u8,
}

impl User {
    fn id(&self) -> u32 { self.id }
    fn name(&self) -> &String { &self.name }
    fn age(&self) -> u8 { self.age }
}

fn main() {
    let user1 = User { id: 1, name: String::from("Alice"), age: 7 };
    let user2 = User { id: 1, name: String::from("Alice"), age: 8 };

    // Compare user1 and user2, focusing only on the `id` and `name` fields
    assert_eq_only!(user1, user2, id, name);
}
```

## License

This project is licensed under the MIT License.
For more information on the license, please see the <a href="LICENSE">license</a>.
````
