use crate::reflection::StructProperty;

pub fn generate(result: &mut String, fields: &[StructProperty]) {
    for field in fields {
        result.push_str("let mut ");
        result.push_str(&field.name);
        result.push_str(" = None;");
    }

    result.push_str("for line in json {");
    result.push_str("let line = line.unwrap();");

    result.push_str("match line.get_name().unwrap() {");

    for field in fields {
        result.push('"');
        result.push_str(&super::db_table_name_generator(&field.name));
        result.push_str("\" => {");
        result.push_str(&field.name);
        result.push_str("let value = line.get_value().unwrap().as_str().unwrap().to_string()\n");
        result.push_str(" = Some(value");

        if !field.ty.is_string() {
            result.push_str(".parse().unwrap()");
        }

        result.push_str(");}");
    }

    result.push_str(" _ => {} } }");

    for field in fields {
        if !field.ty.is_option() {
            result.push_str("if ");
            result.push_str(&field.name);
            result.push_str(".is_none() {panic!(\"");
            result.push_str(&super::db_table_name_generator(&field.name));
            result.push_str(" is not found\");}\n");
        }
    }

    result.push_str("Self {");

    for field in fields {
        if field.ty.is_option() {
            result.push_str(&field.name);
            result.push_str(",\n");
        } else {
            result.push_str(&field.name);
            result.push_str(": ");
            result.push_str(&field.name);

            result.push_str(".unwrap(),\n");
        }
    }

    result.push('}');
}
