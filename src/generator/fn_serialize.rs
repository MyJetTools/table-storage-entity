use crate::reflection::StructProperty;

pub fn generate(result: &mut String, fields: &[StructProperty]) {
    result.push_str("let mut result = my_json::json_writer::JsonObjectWriter::new();");
    for field in fields {
        match &field.ty {
            crate::reflection::PropertyType::U8 => {}
            crate::reflection::PropertyType::I8 => todo!(),
            crate::reflection::PropertyType::U16 => todo!(),
            crate::reflection::PropertyType::I16 => todo!(),
            crate::reflection::PropertyType::U32 => todo!(),
            crate::reflection::PropertyType::I32 => todo!(),
            crate::reflection::PropertyType::U64 => todo!(),
            crate::reflection::PropertyType::I64 => todo!(),
            crate::reflection::PropertyType::F32 => todo!(),
            crate::reflection::PropertyType::F64 => todo!(),
            crate::reflection::PropertyType::USize => todo!(),
            crate::reflection::PropertyType::ISize => todo!(),
            crate::reflection::PropertyType::String => {
                result.push_str("result.write_string_value(\"");
                result.push_str(&super::db_table_name_generator(&field.name));
                result.push_str("\", self.");
                result.push_str(&field.name);
                result.push_str(");\n");
            }
            crate::reflection::PropertyType::Str => todo!(),
            crate::reflection::PropertyType::Bool => todo!(),
            crate::reflection::PropertyType::DateTime => todo!(),
            crate::reflection::PropertyType::OptionOf(_) => {
                result.push_str("if let Some(value) = &self.");
                result.push_str(&field.name);
                result.push('{');

                result.push_str("result.write_string_value(\"");
                result.push_str(&super::db_table_name_generator(&field.name));
                result.push_str("\", self.");
                result.push_str(&field.name);
                result.push_str(".as_str());\n");

                result.push('}');
            }
            crate::reflection::PropertyType::VecOf(_) => todo!(),
            crate::reflection::PropertyType::Struct(_) => todo!(),
        }
    }

    result.push_str("result.build()");
}
