use async_graphql::{dynamic::*, Value};

pub fn create_schema() -> Schema {
    let schema = Schema::build("Query", None, None);

    // define type Query { hello: String! }
    let query = Object::new("Query");
    let field = Field::new("hello", TypeRef::named_nn(TypeRef::STRING), |_ctx| {
        FieldFuture::new(async move { Ok(Some(Value::from("world"))) })
    });
    let query = query.field(field);
    let schema = schema.register(query);

    // add extend type Query { bye: String! }
    let query = Object::new("Query").extends();
    let field = Field::new("bye", TypeRef::named_nn(TypeRef::STRING), |_ctx| {
        FieldFuture::new(async move { Ok(Some(Value::from("universe"))) })
    });
    let query = query.field(field);
    let schema = schema.register(query);

    // finish schema
    schema.finish().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema_utils::normalize_schema;
    use async_graphql::Request;

    #[ignore]
    #[tokio::test]
    async fn test_schema() {
        let schema = create_schema();
        let sdl = schema.sdl();
        println!("{}", sdl);
        assert_eq!(
            normalize_schema(&sdl),
            normalize_schema(
                "
                type Query {
                  hello: String!
                }
                extend type Query {
                  bye: String!
                }
                schema {
                  query: Query
                }
"
            )
        );
    }

    #[ignore]
    #[tokio::test]
    async fn test_query() {
        let schema = create_schema();
        let query = r#"{ hello bye }"#;
        let res = schema.execute(Request::new(query)).await;
        println!("errors: {:?}", res.errors);
        assert_eq!(
            res.data.into_json().unwrap(),
            serde_json::json!({ "hello": "world", "bye": "universe" })
        );
    }
}
