use proc_macro::TokenStream;

pub fn generate(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let fields = crate::reflection::StructProperty::read(ast);

    let struct_name = name.to_string();

    let mut result = String::new();

    result.push_str("impl my_azure_storage_sdk::table_storage::TableStorageEntity for ");
    result.push_str(struct_name.as_str());
    result.push_str(" {\n");

    result.push_str("fn create(json: my_json::json_reader::JsonFirstLineReader) -> Self {");
    super::fn_create::generate(&mut result, &fields);
    result.push_str("}\n");

    result.push_str(
        "fn populate_field_names(builder: crate::sdk_azure::table_storage::TableStorageQueryBuilder) -> Self {",
    );
    super::fn_populate::generate(&mut result, &fields);
    result.push_str("}\n");

    result.push_str("}\n");

    result.parse().unwrap()
}
