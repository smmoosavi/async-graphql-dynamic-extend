use crate::schema::output_types::utils::ResolveOwned;
use crate::schema::registry::{InputObject, Object, Register, Registry};
use async_graphql::dynamic;
use async_graphql::dynamic::DynamicRequestExt;
use serde::Deserialize;

struct Query {}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
enum BarInput {
    A(i32),
    B(i32),
}

impl Query {
    // by object
    async fn resolve_by_bar(&self, input: BarInput) -> String {
        match input {
            BarInput::A(a) => {
                format!("A: {}", a)
            }
            BarInput::B(b) => {
                format!("B: {}", b)
            }
        }
    }
}

// generated

impl Object for Query {
    const NAME: &'static str = "Query";
}

impl InputObject for BarInput {
    const NAME: &'static str = "BarInput";
}

impl Register for Query {
    fn register(registry: Registry) -> Registry {
        // define Query object
        let query_object = dynamic::Object::new("Query");

        // define foo field
        let by_bar = dynamic::Field::new(
            "by_bar",
            dynamic::TypeRef::named_nn(dynamic::TypeRef::STRING),
            |ctx| {
                dynamic::FieldFuture::new(async move {
                    let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                    let input = ctx.args.try_get("input")?.deserialize()?;
                    let value = parent.resolve_by_bar(input).await;
                    ResolveOwned::resolve_owned(value, &ctx)
                })
            },
        );
        let by_bar = by_bar.argument(dynamic::InputValue::new(
            "input",
            dynamic::TypeRef::named_nn(<BarInput as InputObject>::NAME),
        ));

        let query_object = query_object.field(by_bar);

        // register Query object
        registry.register_object(query_object)
    }
}

impl Register for BarInput {
    fn register(registry: Registry) -> Registry {
        // define BarInput object
        let bar_input_object = dynamic::InputObject::new("BarInput");
        let bar_input_object = bar_input_object.oneof();

        // define a field
        let a_field = dynamic::InputValue::new("a", dynamic::TypeRef::named(dynamic::TypeRef::INT));
        let bar_input_object = bar_input_object.field(a_field);

        // define b field

        let b_field = dynamic::InputValue::new("b", dynamic::TypeRef::named(dynamic::TypeRef::INT));
        let bar_input_object = bar_input_object.field(b_field);

        // register BarInput object
        registry.register_input_object(bar_input_object)
    }
}

pub fn create_schema() -> dynamic::Schema {
    let registry = Registry::new().register::<Query>().register::<BarInput>();
    let schema = dynamic::Schema::build(Query::NAME, None, None);
    registry.build_schema(schema).finish().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema_utils::normalize_schema;

    #[test]
    fn test_schema() {
        let schema = create_schema();
        let sdl = schema.sdl();
        assert_eq!(
            normalize_schema(&sdl),
            normalize_schema(
                r#"
                directive @oneOf on INPUT_OBJECT
                input BarInput @oneOf {
                  a: Int
                  b: Int
                }
                type Query {
                  by_bar(input: BarInput!): String!
                }
                schema {
                  query: Query
                }

            "#
            ),
        );
    }

    fn create_root() -> Query {
        Query {}
    }

    #[tokio::test]
    async fn test_query_by_bar() {
        let schema = create_schema();
        let query = r#"
            query {
              by_bar(input: { a: 1 })
            }
        "#;

        let req = async_graphql::Request::new(query)
            .root_value(dynamic::FieldValue::owned_any(create_root()));
        let res = schema.execute(req).await;
        let data = res.data.into_json().unwrap();
        assert_eq!(
            data,
            serde_json::json!({
                "by_bar": "A: 1"
            }),
        );
    }
}
