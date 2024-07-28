/// Asserts that specific fields of two values are equal.
///
/// The fields listed in the macro are compared for equality. This allows you to focus on comparing only the specified fields
/// without needing to compare the entire struct or object.
///
/// # Syntax
///
/// `assert_eq_only!(actual_value, expected_value, field1, field2, ...);`
/// `assert_eq_only!(actual_value, expected_value, field1, field2, ..., case_name);`
///
/// - `actual_value`: The actual value to compare.
/// - `expected_value`: The expected value to compare against.
/// - `field1, field2, ...`: The names of the fields to compare.
/// - `case_name`: An optional string describing the test case, included in the panic message for easier identification.
///
/// # Example
///
/// Suppose you have a struct `User` with fields `id`, `name`, and `age`, and you want to compare only the `id` and `name` fields:
///
/// ```rust
/// use selective_assertions::*;
///
/// #[derive(Debug)]
/// pub struct User {
///     id: u32,
///     name: String,
///     age: u8,
/// }
///
/// impl User {
///     pub fn id(&self) -> u32 {
///         self.id
///     }
///
///     pub fn name(&self) -> &String {
///         &self.name
///     }
///
///     pub fn age(&self) -> u8 {
///         self.age
///     }
/// }
///
/// let user1 = User { id: 1, name: String::from("Alice"), age: 7 };
/// let user2 = User { id: 1, name: String::from("Alice"), age: 8 };
///
/// // Compare user1 and user2, focusing only on the `id` and `name` fields
/// assert_eq_only!(user1, user2, name); // This will pass
/// ```
///
/// # Panics
///
/// This macro will panic if the specified fields of the actual and expected values are not equal.
#[macro_export]
macro_rules! assert_eq_only {
    ($actual:expr, $expect:expr, $($field:ident),+) => {{
        $(
            assert_eq!($actual.$field(), $expect.$field(), "Field `{}` does not match", stringify!($field));
        )+
    }};
}
