/// Asserts that two values are equal, ignoring the specified fields.
///
/// The fields listed in the macro are set to their default values before performing
/// the equality check. This allows you to assert that all other fields are equal
/// while ignoring the specified ones.
///
/// # Syntax
///
/// `assert_eq_excluding!(actual_value, expected_value, field1, field2, ...);`
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
/// use selective_assertions::*;
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
/// assert_eq_excluding!(user1, user2, age); // This will pass
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
macro_rules! assert_eq_excluding {
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
        let description = $crate::impls::assert_eq_excluding::description(None, fields);

        assert_eq!(actual_clone, expect_clone, "{}", description);
    }};
}

pub fn description(case_name: Option<&str>, fields: Vec<&str>) -> String {
    let verb = verb(fields.len());
    let fields = fields_string(fields);

    if let Some(case_name) = case_name {
        format!(
            "{} (Fields {} {} updated by default)",
            case_name, fields, verb
        )
    } else {
        format!("(Fields {} {} updated by default)", fields, verb)
    }
}

fn verb(field_size: usize) -> String {
    if field_size == 1 {
        "is".to_string()
    } else {
        "are".to_string()
    }
}

fn fields_string(fields: Vec<&str>) -> String {
    let field_size = fields.len();

    let fields = fields
        .into_iter()
        .map(|s| format!("`{}`", s))
        .collect::<Vec<String>>();

    let fields = match field_size {
        0 => "".to_string(),
        1 => fields[0].clone(),
        2 => format!("{} and {}", fields[0], fields[1]),
        _ => {
            let (head, tail) = fields.split_at(fields.len() - 1);
            let formatted_tail = tail.join(", ");
            format!("{} and {}", formatted_tail, head.last().unwrap())
        }
    };

    fields
}
