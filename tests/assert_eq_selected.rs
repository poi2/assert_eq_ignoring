#[cfg(test)]
mod test {
    use derive_getters::Getters;
    use selective_assertions::*;

    #[derive(Debug, PartialEq, Clone)]
    #[cfg_attr(test, derive(Getters))]
    struct User {
        id: u32,
        name: String,
        age: usize,
    }

    #[test]
    fn test_assert_eq_selected_should_pass_when_selects_same_one_field() {
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

        // This will pass because the `id` is the same
        assert_eq_selected!(alice_in_wonder_land, alice_in_looking_glass_land, id);
    }

    #[test]
    fn test_assert_eq_selected_should_pass_when_selects_same_some_fields() {
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

        // This will pass because the `id` and `name` are the same
        assert_eq_selected!(alice_in_wonder_land, alice_in_looking_glass_land, id, name);
    }

    #[should_panic]
    #[test]
    fn test_assert_eq_selected_should_fail_when_selects_fields_has_with_some_different_values() {
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

        // This will pass because the `age` is different
        assert_eq_selected!(
            alice_in_wonder_land,
            alice_in_looking_glass_land,
            id,
            name,
            age
        );
    }
}
