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
