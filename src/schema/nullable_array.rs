use async_graphql::dynamic::*;

pub fn create_schema() -> Schema {
    let query = Object::new("Query");
    let field = Field::new("hello", TypeRef::named_nn_list(TypeRef::STRING), |_ctx| {
        FieldFuture::new(async move { Ok(None as Option<FieldValue>) })
    });
    let query = query.field(field);

    let schema = Schema::build(query.type_name(), None, None);
    let schema = schema.register(query);

    schema.finish().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema_utils::normalize_schema;

    #[tokio::test]
    async fn test_schema() {
        let schema = create_schema();
        let sdl = schema.sdl();
        assert_eq!(
            normalize_schema(&sdl),
            normalize_schema(
                "
                type Query { hello: [String!] }
                schema { query: Query }
"
            )
        );
    }

    #[tokio::test]
    async fn test_query() {
        let schema = create_schema();
        let query = r#"{ hello }"#;
        let res = schema.execute(query).await;
        assert_eq!(
            res.data.into_json().unwrap(),
            serde_json::json!({ "hello": null })
        );
    }
}
