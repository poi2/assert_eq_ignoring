/// Asserts that two values are equal, excluding the specified fields.
///
/// The fields listed in the macro are set to the values copied from the expected
/// value before performing the equality check.
///
/// This allows you to assert that all　other fields are equal while excluding　the
/// specified ones.
///
/// # Syntax
///
/// `assert_eq_excluding!(actual_value, expected_value, field1, field2, ...);`
///
/// - `actual_value`: The actual value to compare.
/// - `expected_value`: The expected value to compare against.
/// - `field1, field2, ...`: The names of the fields to exclude during the comparison.
///
/// # Example
///
/// If you want to compare two `User` instances while excluding the `age` as follows:
///
/// `assert_eq_excluding!(user1, user2, age);`
///
/// ```rust
/// use derive_getters::Getters;
/// use getset::Setters;
/// use selective_assertions::*;
///
/// #[derive(Debug, PartialEq, Clone, Getters, Setters)]
/// #[set = "pub"]
/// pub struct User {
///     id: u32,
///     name: String,
///     age: u8,
/// }
///
/// let user1 = User { id: 1, name: "Alice".to_string(), age: 7 };
/// let user2 = User { id: 1, name: "Alice".to_string(), age: 8 };
///
/// // Compare user1 and user2, excluding the `age` field
/// assert_eq_excluding!(user1, user2, age); // This will pass
/// ```
///
/// # Panics
///
/// This macro will panic if the actual and expected values are not equal when excluding
/// the specified fields.
///
/// # Note
///
/// The macro requires getter and setter methods.
///
/// It is recommended to use `derive_getters::Getters` and `getset::Setters` crates.
#[macro_export]
macro_rules! assert_eq_excluding {
    ($actual:expr, $expect:expr, $($field:ident),+) => {{
        let mut actual_clone = $actual.clone();
        $(
            paste! {
                let value = $expect.[<$field>]().clone();
                actual_clone.[<set_ $field>](value);
            }
        )+

        let fields = vec![
            $(
                stringify!($field),
            )+
        ];
        let description = $crate::impls::assert_eq_excluding::description(None, fields);

        assert_eq!(actual_clone, $expect, "{}", description);
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
