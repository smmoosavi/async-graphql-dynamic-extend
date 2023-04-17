use async_graphql::{dynamic::*, Value};

// part of another lib
struct Outer {
    string: String,
}

struct Inner<'a> {
    pub string_ref: &'a str,
}

impl Outer {
    fn get_inner(&self) -> Inner {
        Inner {
            string_ref: &self.string,
        }
    }
}

// schema structs
struct Query {
    pub outer: Outer,
}

struct Foo<'a> {
    pub inner: Inner<'a>,
}

impl Query {
    fn foo(&self) -> Foo {
        Foo {
            inner: self.outer.get_inner(),
        }
    }
}

impl<'a> Foo<'a> {
    fn value(&self) -> String {
        self.inner.string_ref.to_string()
    }
}

// type definitions

pub fn create_schema() -> Schema {
    // query
    let query = Object::new("Query");
    let foo_field = Field::new("foo", TypeRef::named_nn("Foo"), |ctx| {
        FieldFuture::new(async move {
            let query = ctx.parent_value.downcast_ref::<Query>().unwrap();
            let foo = query.foo();
            Ok(Some(FieldValue::owned_any(foo)))
        })
    });
    let query = query.field(foo_field);
    // foo
    let foo_object = Object::new("Foo");
    let value_field = Field::new("value", TypeRef::named_nn(TypeRef::STRING), |ctx| {
        FieldFuture::new(async move {
            let foo = ctx.parent_value.downcast_ref::<Foo>().unwrap();
            let string = foo.value();
            Ok(Some(Value::from(string)))
        })
    });
    let foo_object = foo_object.field(value_field);
    // schema

    let schema = Schema::build(query.type_name(), None, None);
    let schema = schema.register(query).register(foo_object);
    schema.finish().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema_utils::normalize_schema;
    use async_graphql::Request;

    #[tokio::test]
    async fn test_schema() {
        let schema = create_schema();
        let sdl = schema.sdl();
        assert_eq!(
            normalize_schema(&sdl),
            normalize_schema(
                "
                type Foo { value: String! }
                type Query { foo: Foo! }
                schema { query: Query }
"
            )
        );
    }

    #[tokio::test]
    async fn test_query() {
        let schema = create_schema();
        let query = r#"{ foo { value } }"#;
        let root = Query {
            outer: Outer {
                string: "world".to_string(),
            },
        };
        let req = Request::new(query).root_value(FieldValue::owned_any(root));
        let res = schema.execute(req).await;
        assert_eq!(
            res.data.into_json().unwrap(),
            serde_json::json!({ "foo": { "value": "world" } })
        );
    }
}
