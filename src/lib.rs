pub use paste::paste;

pub mod description;

extern crate paste;

/// Asserts that two values are equal, ignoring the specified fields.
///
/// The fields listed in the macro are set to their default values before performing
/// the equality check. This allows you to assert that all other fields are equal
/// while ignoring the specified ones.
///
/// # Syntax
///
/// `assert_eq_ignoring!(actual_value, expected_value, field1, field2, ...);`
///
/// - `actual_value`: The actual value to compare.
/// - `expected_value`: The expected value to compare against.
/// - `field1, field2, ...`: The names of the fields to ignore during the comparison.
///
/// # Example
///
/// Suppose you have a struct `User` with fields `id`, `name`, and `age`, and you want
/// to compare two `User` instances while ignoring the `age` field:
///
/// ```rust
/// use assert_eq_ignoring::*;
///
/// #[derive(Debug, PartialEq, Clone)]
/// pub struct User {
///     id: u32,
///     name: String,
///     age: u8,
/// }
///
/// impl User {
///     pub fn new(id: u32, name: String, age: u8) -> Self {
///         User { id, name, age }
///     }
///
///     pub fn set_id(&mut self, id: u32) {
///         self.id = id;
///     }
///
///     pub fn set_name(&mut self, name: String) {
///         self.name = name;
///     }
///
///     pub fn set_age(&mut self, age: u8) {
///         self.age = age;
///     }
/// }
///
/// let user1 = User::new(1, "Alice".to_string(), 7);
/// let user2 = User::new(1, "Alice".to_string(), 8);
///
/// // Compare user1 and user2, ignoring the `age` field
/// assert_eq_ignoring!(user1, user2, age); // This will pass
/// ```
///
/// # Panics
///
/// This macro will panic if the actual and expected values are not equal when ignoring the specified fields.
///
/// # Note
///
/// The macro assumes that the struct implements a `set_<field_name>` method to set fields to their default values.
/// Ensure that such methods are available and properly implemented for the fields you want to ignore.
///
/// For convenience and to avoid manually writing setter methods, it is recommended to use the `getset::Setters`
/// crate, which can automatically generate setter methods for your fields. This helps ensure consistency and
/// reduces boilerplate code.
#[macro_export]
macro_rules! assert_eq_ignoring {
    ($actual:expr, $expect:expr, $($field:ident),+) => {{
        let mut actual_clone = $actual.clone();
        let mut expect_clone = $expect.clone();
        $(
            paste! {
                actual_clone.[<set_ $field>](Default::default());
                expect_clone.[<set_ $field>](Default::default());
            }
        )+

        let fields = vec![
            $(
                stringify!($field),
            )+
        ];
        let description = $crate::description::description(None, fields);

        assert_eq!(actual_clone, expect_clone, "{}", description);
    }};
    ($actual:expr, $expect:expr, $($field:ident),+, $case_name:expr) => {{
        let mut actual_clone = $actual.clone();
        let mut expect_clone = $expect.clone();
        $(
            paste! {
                actual_clone.[<set_ $field>](Default::default());
                expect_clone.[<set_ $field>](Default::default());
            }
        )+

        let fields = vec![
            $(
                stringify!($field),
            )+
        ];
        let description = $crate::description::description(Some($case_name), fields);

        assert_eq!(actual_clone, expect_clone, "{}", description);
    }};
}
