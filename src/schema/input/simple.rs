use crate::schema::output_types::utils::{ResolveOwned, ResolveRef};
use crate::schema::registry::{InputObject, Object, Register, Registry};
use async_graphql::dynamic::{DynamicRequestExt};
use async_graphql::{dynamic, Context, ID};
use serde::Deserialize;

struct Query {
    foo: Foo,
}

#[derive(Deserialize)]
struct BarInput {
    bar: String,
}

struct Foo;

impl Foo {
    // by object
    async fn resolve_by_object(&self, input: BarInput) -> String {
        format!("bar-input.bar: {}", input.bar)
    }
    async fn resolve_by_string(&self, input: String) -> String {
        format!("string: {}", input)
    }
    async fn resolve_by_int(&self, input: i32) -> String {
        format!("i32: {}", input)
    }
    async fn resolve_by_float(&self, input: f32) -> String {
        format!("f32: {}", input)
    }
    async fn resolve_by_bool(&self, input: bool) -> String {
        format!("bool: {}", input)
    }
    async fn resolve_by_id(&self, input: ID) -> String {
        format!("id: {}", input.0)
    }
}


// generated

impl Object for Query {
    const NAME: &'static str = "Query";
}

impl Object for Foo {
    const NAME: &'static str = "Foo";
}

impl InputObject for BarInput {
    const NAME: &'static str = "BarInput";
}

impl Register for Query {
    fn register(registry: Registry) -> Registry {
        // define Query object
        let query_object = dynamic::Object::new("Query");

        // define foo field
        let foo_field = dynamic::Field::new("foo", dynamic::TypeRef::named_nn(Foo::NAME), |ctx| {
            dynamic::FieldFuture::new(async move {
                let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                let value = parent.resolve_foo().await;
                ResolveRef::resolve_ref(value, &ctx)
            })
        });

        let query_object = query_object.field(foo_field);

        // register Query object
        registry.register_object(query_object)
    }
}

impl Query {
    async fn resolve_foo(&self) -> &Foo {
        &self.foo
    }
}

impl Register for BarInput {
    fn register(registry: Registry) -> Registry {
        // define BarInput object
        let bar_input_object = dynamic::InputObject::new("BarInput");

        // define bar field
        let bar_field = dynamic::InputValue::new("bar", dynamic::TypeRef::named_nn("String"));

        let bar_input_object = bar_input_object.field(bar_field);

        // register BarInput object
        registry.register_input_object(bar_input_object)
    }
}

impl<'a> ResolveOwned<'a> for Foo {
    fn resolve_owned(self, _ctx: &Context) -> async_graphql::Result<Option<dynamic::FieldValue<'a>>> {
        Ok(Some(dynamic::FieldValue::owned_any(self)))
    }
}

impl<'a> ResolveRef<'a> for Foo {
    fn resolve_ref(&'a self, _ctx: &Context) -> async_graphql::Result<Option<dynamic::FieldValue<'a>>> {
        Ok(Some(dynamic::FieldValue::borrowed_any(self)))
    }
}

impl Register for Foo {
    fn register(registry: Registry) -> Registry {
        // define Foo object
        let object_type = dynamic::Object::new(Self::NAME);

        // define by_object field
        let by_object_field =
            dynamic::Field::new("by_object", dynamic::TypeRef::named_nn("String"), |ctx| {
                dynamic::FieldFuture::new(async move {
                    let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                    let arg_0 = ctx.args.try_get("input")?.deserialize()?;
                    let value = parent.resolve_by_object(arg_0).await;
                    ResolveOwned::resolve_owned(value, &ctx)
                })
            });

        let by_object_field = by_object_field.argument(dynamic::InputValue::new(
            "input",
            dynamic::TypeRef::named_nn("BarInput"),
        ));

        let object_type = object_type.field(by_object_field);

        // define by_string field
        let by_string_field = dynamic::Field::new(
            "by_string",
            dynamic::TypeRef::named_nn(dynamic::TypeRef::STRING),
            |ctx| {
                dynamic::FieldFuture::new(async move {
                    let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                    let arg_0 = ctx.args.try_get("input")?.deserialize()?;
                    let value = parent.resolve_by_string(arg_0).await;
                    ResolveOwned::resolve_owned(value, &ctx)
                })
            },
        );
        let by_string_field = by_string_field.argument(dynamic::InputValue::new(
            "input",
            dynamic::TypeRef::named_nn(dynamic::TypeRef::STRING),
        ));
        let object_type = object_type.field(by_string_field);

        // define by_int field

        let by_int_field = dynamic::Field::new(
            "by_int",
            dynamic::TypeRef::named_nn(dynamic::TypeRef::INT),
            |ctx| {
                dynamic::FieldFuture::new(async move {
                    let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                    let arg_0 = ctx.args.try_get("input")?.deserialize()?;
                    let value = parent.resolve_by_int(arg_0).await;
                    ResolveOwned::resolve_owned(value, &ctx)
                })
            },
        );

        let by_int_field = by_int_field.argument(dynamic::InputValue::new(
            "input",
            dynamic::TypeRef::named_nn(dynamic::TypeRef::INT),
        ));

        let object_type = object_type.field(by_int_field);

        // define by_float field

        let by_float_field = dynamic::Field::new(
            "by_float",
            dynamic::TypeRef::named_nn(dynamic::TypeRef::FLOAT),
            |ctx| {
                dynamic::FieldFuture::new(async move {
                    let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                    let arg_0 = ctx.args.try_get("input")?.deserialize()?;
                    let value = parent.resolve_by_float(arg_0).await;
                    ResolveOwned::resolve_owned(value, &ctx)
                })
            },
        );

        let by_float_field = by_float_field.argument(dynamic::InputValue::new(
            "input",
            dynamic::TypeRef::named_nn(dynamic::TypeRef::FLOAT),
        ));

        let object_type = object_type.field(by_float_field);

        // define by_bool field

        let by_bool_field = dynamic::Field::new(
            "by_bool",
            dynamic::TypeRef::named_nn(dynamic::TypeRef::BOOLEAN),
            |ctx| {
                dynamic::FieldFuture::new(async move {
                    let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                    let arg_0 = ctx.args.try_get("input")?.deserialize()?;
                    let value = parent.resolve_by_bool(arg_0).await;
                    ResolveOwned::resolve_owned(value, &ctx)
                })
            },
        );

        let by_bool_field = by_bool_field.argument(dynamic::InputValue::new(
            "input",
            dynamic::TypeRef::named_nn(dynamic::TypeRef::BOOLEAN),
        ));

        let object_type = object_type.field(by_bool_field);

        // define by_id field

        let by_id_field = dynamic::Field::new(
            "by_id",
            dynamic::TypeRef::named_nn(dynamic::TypeRef::ID),
            |ctx| {
                dynamic::FieldFuture::new(async move {
                    let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                    let arg_0 = ctx.args.try_get("input")?.deserialize()?;
                    let value = parent.resolve_by_id(arg_0).await;
                    ResolveOwned::resolve_owned(value, &ctx)
                })
            },
        );

        let by_id_field = by_id_field.argument(dynamic::InputValue::new(
            "input",
            dynamic::TypeRef::named_nn(dynamic::TypeRef::ID),
        ));

        let object_type = object_type.field(by_id_field);

        // register Foo object

        registry.register_object(object_type)
    }
}

pub fn create_schema() -> dynamic::Schema {
    let registry = Registry::new().register::<Query>().register::<Foo>().register::<BarInput>();
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
                    input BarInput {
                      bar: String!
                    }
                    type Foo {
                      by_object(input: BarInput!): String!
                      by_string(input: String!): String!
                      by_int(input: Int!): Int!
                      by_float(input: Float!): Float!
                      by_bool(input: Boolean!): Boolean!
                      by_id(input: ID!): ID!
                    }
                    type Query {
                      foo: Foo!
                    }
                    schema {
                      query: Query
                    }
            "#
            ),
        );
    }

    fn create_root() -> Query {
        Query { foo: Foo }
    }

    #[tokio::test]
    async fn test_query_by_object(){
        let schema = create_schema();
        let query = r#"
            query {
                foo {
                    by_object(input: { bar: "world"})
                }
            }
        "#;

        let req =
            async_graphql::Request::new(query).root_value(dynamic::FieldValue::owned_any(create_root()));
        let res = schema.execute(req).await;
        let data = res.data.into_json().unwrap();
        assert_eq!(
            data,
            serde_json::json!({
                "foo": { "by_object": "bar-input.bar: world" }
            }),
        );
    }

    #[tokio::test]
    async fn test_query_by_string() {
        let schema = create_schema();
        let query = r#"
            query {
                foo {
                    by_string(input: "world")
                    by_id(input: "world")
                }
            }
        "#;

        let req =
            async_graphql::Request::new(query).root_value(dynamic::FieldValue::owned_any(create_root()));
        let res = schema.execute(req).await;
        let data = res.data.into_json().unwrap();
        assert_eq!(
            data,
            serde_json::json!({
                "foo": {
                    "by_string": "string: world",
                    "by_id": "id: world"
                }
            }),
        );
    }

    #[tokio::test]
    async fn test_query_by_number() {
        let schema = create_schema();
        let query = r#"
            query {
                foo {
                    by_bool(input: true)
                    by_int(input: 42)
                    by_float(input: 42.5)
                }
            }
        "#;

        let req =
            async_graphql::Request::new(query).root_value(dynamic::FieldValue::owned_any(create_root()));
        let res = schema.execute(req).await;
        let data = res.data.into_json().unwrap();
        assert_eq!(
            data,
            serde_json::json!({
                "foo": {
                    "by_bool": "bool: true",
                    "by_int": "i32: 42",
                    "by_float": "f32: 42.5"
                }
            }),
        );
    }

}
