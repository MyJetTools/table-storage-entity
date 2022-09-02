use proc_macro::TokenStream;

use crate::reflection::StructProperty;

pub fn generate(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let fields = crate::reflection::StructProperty::read(ast);
    check_mandatory_fields(&fields);

    let struct_name = name.to_string();

    let mut result = String::new();

    result.push_str("impl my_azure_storage_sdk::table_storage::TableStorageEntity for ");
    result.push_str(struct_name.as_str());
    result.push_str(" {\n");

    result.push_str("fn create(json: my_json::json_reader::JsonFirstLineReader) -> Self {");
    super::fn_create::generate(&mut result, &fields);
    result.push_str("}\n");

    result.push_str(
        "fn populate_field_names(builder: &mut my_azure_storage_sdk::sdk_azure::table_storage::TableStorageQueryBuilder,) -> Self {",
    );
    super::fn_populate::generate(&mut result, &fields);
    result.push_str("}\n");

    result.push_str("}\n");

    result.parse().unwrap()
}

const PARTITION_KEY_FIELD: &str = "partition_key";
const ROW_KEY_FIELD: &str = "row_key";
const TIMESTAMP_FIELD: &str = "timestamp";

fn check_mandatory_fields(fields: &[StructProperty]) {
    let mut partition_key = None;
    let mut row_key = None;
    let mut timestamp = None;

    for field in fields {
        if field.name == PARTITION_KEY_FIELD {
            partition_key = Some(field);
        } else if field.name == ROW_KEY_FIELD {
            row_key = Some(field);
        } else if field.name == TIMESTAMP_FIELD {
            timestamp = Some(field);
        }
    }

    if partition_key.is_none() {
        panic!("{} is not found", PARTITION_KEY_FIELD);
    }

    if row_key.is_none() {
        panic!("{} is not found", ROW_KEY_FIELD);
    }

    if timestamp.is_none() {
        panic!("{} is not found", TIMESTAMP_FIELD);
    }

    if !partition_key.unwrap().ty.is_string() {
        panic!("{} must be String", PARTITION_KEY_FIELD);
    }

    if !row_key.unwrap().ty.is_string() {
        panic!("{} must be String", ROW_KEY_FIELD);
    }

    if !timestamp.unwrap().ty.is_string() {
        panic!("{} must be String", TIMESTAMP_FIELD);
    }
}
