use crate::reflection::StructProperty;

pub fn generate(result: &mut String, fields: &[StructProperty]) {
    for field in fields {
        result.push_str("bulder.add_select_param(\"");
        result.push_str(&super::db_table_name_generator(&field.name));
        result.push_str("\");");
    }
}
