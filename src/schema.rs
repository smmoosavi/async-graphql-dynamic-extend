use async_graphql::{dynamic::*, Value};

pub fn create_schema() -> Schema {
    let query = Object::new("Query");
    let field = Field::new("hello", TypeRef::named_nn(TypeRef::STRING), |_ctx| {
        FieldFuture::new(async move {
            Ok(Some(Value::from("world")))
        })
    });
    let query = query.field(field);

    let schema = Schema::build(query.type_name(), None, None);
    let schema = schema.register(query);

    schema.finish().unwrap()
}
