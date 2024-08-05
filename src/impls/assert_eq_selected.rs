/// Asserts that specific fields of two values are equal.
///
/// The fields listed in the macro are compared for equality.
///
/// This allows you to focus on comparing only the specified fields without needing
/// to compare the entire struct or object.
///
/// # Syntax
///
/// `assert_eq_selected!(actual_value, expected_value, field1, field2, ...);`
///
/// - `actual_value`: The actual value to compare.
/// - `expected_value`: The expected value to compare against.
/// - `field1, field2, ...`: The names of the fields to compare.
///
/// # Example
///
/// If you want to compare two `User` instances only the `id` and `name` as follows:
///
/// `assert_eq_selected!(user1, user2, name);`
///
/// ```rust
/// use derive_getters::Getters;
/// use selective_assertions::*;
///
/// #[derive(Debug, PartialEq, Getters)]
/// pub struct User {
///     id: u32,
///     name: String,
///     age: u8,
/// }
///
///
/// let user1 = User { id: 1, name: String::from("Alice"), age: 7 };
/// let user2 = User { id: 1, name: String::from("Alice"), age: 8 };
///
/// // Compare user1 and user2, focusing only on the `id` and `name` fields
/// assert_eq_selected!(user1, user2, name); // This will pass
/// ```
///
/// # Panics
///
/// This macro will panic if the specified fields of the actual and expected values are not equal.
///
/// # Note
///
/// The macro requires getter methods to set the fields to the expected values.
///
/// It is recommended to use `derive_getters::Getters` crate.
#[macro_export]
macro_rules! assert_eq_selected {
    ($actual:expr, $expect:expr, $($field:ident),+) => {{
        $(
            assert_eq!($actual.$field(), $expect.$field(), "Field `{}` does not match", stringify!($field));
        )+
    }};
}
