use tantivy::schema::*;

pub fn create_schema() -> Schema {
    let mut schema_builder = Schema::builder();
    
    schema_builder.add_text_field("name", TEXT | STORED);
    schema_builder.add_text_field("os", STRING | STORED);
    schema_builder.add_text_field("arch", STRING | STORED);
    schema_builder.add_text_field("category", STRING | STORED);
    schema_builder.add_text_field("version", STRING | STORED);
    schema_builder.add_text_field("family", STRING | STORED);
    schema_builder.add_u64_field("number", STORED);
    
    schema_builder.build()
}

pub struct SchemaFields {
    pub name: Field,
    pub os: Field,
    pub arch: Field,
    pub category: Field,
    pub version: Field,
    pub family: Field,
    pub number: Field,
}

impl SchemaFields {
    pub fn from_schema(schema: &Schema) -> Self {
        Self {
            name: schema.get_field("name").unwrap(),
            os: schema.get_field("os").unwrap(),
            arch: schema.get_field("arch").unwrap(),
            category: schema.get_field("category").unwrap(),
            version: schema.get_field("version").unwrap(),
            family: schema.get_field("family").unwrap(),
            number: schema.get_field("number").unwrap(),
        }
    }
}