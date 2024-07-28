#[cfg(test)]
mod test {
    use getset::Setters;
    use selective_assertions::*;

    #[derive(Debug, PartialEq, Clone)]
    #[cfg_attr(test, derive(Setters))]
    #[cfg_attr(test, set = "pub")]
    struct User {
        id: u32,
        name: String,
        age: usize,
    }

    #[test]
    fn test_assert_eq_ignoring_should_pass_when_one_different_field_is_excluded() {
        let alice_in_wonder_land = User {
            id: 1,
            name: "Alice".to_string(),
            age: 7,
        };

        let alice_in_looking_glass_land = User {
            id: 1,
            name: "Alice".to_string(),
            age: 8,
        };

        // This will pass because the `age` field is ignored
        assert_eq_ignoring!(alice_in_wonder_land, alice_in_looking_glass_land, age);
    }

    #[test]
    fn test_assert_eq_ignoring_should_pass_when_multiple_different_fields_are_excluded() {
        let classmate1 = User {
            id: 1,
            name: "Alice".to_string(),
            age: 16,
        };

        let classmate2 = User {
            id: 2,
            name: "Bob".to_string(),
            age: 16,
        };

        // This will pass because the `id` and `name` fields are ignored
        assert_eq_ignoring!(classmate1, classmate2, id, name);
    }

    #[should_panic]
    #[test]
    fn test_assert_eq_ignoring_should_fail_when_different_fields_are_left() {
        let alice_in_wonder_land = User {
            id: 1,
            name: "Alice".to_string(),
            age: 7,
        };

        let alice_in_looking_glass_land = User {
            id: 1,
            name: "Alice".to_string(),
            age: 8,
        };

        // This will panic because the `age` field is different
        assert_eq_ignoring!(alice_in_wonder_land, alice_in_looking_glass_land, id, name);
    }
}
