pub fn db_table_name_generator(property_name: &str) -> String {
    let mut upper_case = true;
    let mut result = String::with_capacity(property_name.len());

    let as_bytes = property_name.as_bytes();

    for b in as_bytes {
        let c = *b as char;

        if c == '_' {
            upper_case = true;
            continue;
        }

        if upper_case {
            result.push(c.to_ascii_uppercase());
            upper_case = false;
        } else {
            result.push(c);
        }
    }

    result
}

#[cfg(test)]
mod test {
    use crate::generator::db_table_name_generator;

    #[test]
    fn test_name_generator() {
        assert_eq!("MyTestName", db_table_name_generator("my_test_name"));
    }
}
