# table-storage-entity


### General requirements

Azure Table Storage Entity must have compulsory fields:
* partition_key;
* row_key
* timestamp

```Rust
use table_storage_entity::TableStorageEntity;

#[derive(TableStorageEntity)]
pub struct TestTableEntity {
    pub partition_key: String,
    pub row_key: String,
    pub timestamp: Option<String>,
}

```

### Naming convention

Struct fileds have to be create in snake case pattern:

Example:
* partition_key
* row_key
* my_data

Table storage is going to keep data in fields

* PartitionKey
* RowKey
* MyData


### Ignoring Field

To ignore filed - it has to be marked with ignore_table_storage_field attribute

```Rust
use table_storage_entity::TableStorageEntity;

#[derive(TableStorageEntity)]
pub struct TestTableEntity {
    pub partition_key: String,
    pub row_key: String,
    pub timestamp: Option<String>,
    #[ignore_table_storage_field]
    pub field_to_ignore: String,
}

```
