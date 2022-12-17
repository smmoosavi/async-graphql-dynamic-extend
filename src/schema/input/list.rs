use crate::schema::output_types::utils::{ResolveOwned, ResolveRef};
use crate::schema::registry::{InputObject, Object, Register, Registry};
use async_graphql::dynamic::DynamicRequestExt;
use async_graphql::{dynamic, Context, ID};
use serde::Deserialize;
use std::fmt::{Display, Formatter};

struct Query {
    foo: Foo,
}

#[derive(Deserialize)]
struct BarInput {
    bar: String,
}

impl Display for BarInput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "BarInput {{ bar: {} }}", self.bar)
    }
}

struct Foo;

impl Foo {
    // by object
    async fn resolve_by_objects(&self, input: Vec<BarInput>) -> String {
        format!(
            "bar-inputs: [{}]",
            input
                .iter()
                .map(|b| b.to_string())
                .collect::<Vec<_>>()
                .join(",")
        )
    }
    async fn resolve_by_strings(&self, input: Vec<String>) -> String {
        format!("strings: [{}]", input.join(","))
    }
    async fn resolve_by_ints(&self, input: Vec<i32>) -> String {
        format!(
            "i32s: [{}]",
            input
                .iter()
                .map(|b| b.to_string())
                .collect::<Vec<_>>()
                .join(",")
        )
    }
    async fn resolve_by_floats(&self, input: Vec<f32>) -> String {
        format!(
            "f32s: [{}]",
            input
                .iter()
                .map(|b| b.to_string())
                .collect::<Vec<_>>()
                .join(",")
        )
    }
    async fn resolve_by_bools(&self, input: Vec<bool>) -> String {
        format!(
            "bools: [{}]",
            input
                .iter()
                .map(|b| b.to_string())
                .collect::<Vec<_>>()
                .join(",")
        )
    }
    async fn resolve_by_ids(&self, input: Vec<ID>) -> String {
        format!(
            "ids: [{}]",
            input
                .iter()
                .map(|b| b.to_string())
                .collect::<Vec<_>>()
                .join(",")
        )
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
    fn resolve_owned(
        self,
        _ctx: &Context,
    ) -> async_graphql::Result<Option<dynamic::FieldValue<'a>>> {
        Ok(Some(dynamic::FieldValue::owned_any(self)))
    }
}

impl<'a> ResolveRef<'a> for Foo {
    fn resolve_ref(
        &'a self,
        _ctx: &Context,
    ) -> async_graphql::Result<Option<dynamic::FieldValue<'a>>> {
        Ok(Some(dynamic::FieldValue::borrowed_any(self)))
    }
}

impl Register for Foo {
    fn register(registry: Registry) -> Registry {
        // define Foo object
        let object_type = dynamic::Object::new(Self::NAME);

        // define by_object field
        let by_object_field =
            dynamic::Field::new("by_objects", dynamic::TypeRef::named_nn("String"), |ctx| {
                dynamic::FieldFuture::new(async move {
                    let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                    let arg_0 = ctx.args.try_get("input")?.deserialize()?;
                    let value = parent.resolve_by_objects(arg_0).await;
                    ResolveOwned::resolve_owned(value, &ctx)
                })
            });

        let by_object_field = by_object_field.argument(dynamic::InputValue::new(
            "input",
            dynamic::TypeRef::named_nn_list_nn("BarInput"),
        ));

        let object_type = object_type.field(by_object_field);

        // define by_string field
        let by_string_field = dynamic::Field::new(
            "by_strings",
            dynamic::TypeRef::named_nn(dynamic::TypeRef::STRING),
            |ctx| {
                dynamic::FieldFuture::new(async move {
                    let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                    let arg_0 = ctx.args.try_get("input")?.deserialize();

                    let value = parent.resolve_by_strings(arg_0?).await;
                    ResolveOwned::resolve_owned(value, &ctx)
                })
            },
        );
        let by_string_field = by_string_field.argument(dynamic::InputValue::new(
            "input",
            dynamic::TypeRef::named_nn_list_nn(dynamic::TypeRef::STRING),
        ));
        let object_type = object_type.field(by_string_field);

        // define by_int field

        let by_int_field = dynamic::Field::new(
            "by_ints",
            dynamic::TypeRef::named_nn(dynamic::TypeRef::INT),
            |ctx| {
                dynamic::FieldFuture::new(async move {
                    let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                    let arg_0 = ctx.args.try_get("input")?.deserialize()?;
                    let value = parent.resolve_by_ints(arg_0).await;
                    ResolveOwned::resolve_owned(value, &ctx)
                })
            },
        );

        let by_int_field = by_int_field.argument(dynamic::InputValue::new(
            "input",
            dynamic::TypeRef::named_nn_list_nn(dynamic::TypeRef::INT),
        ));

        let object_type = object_type.field(by_int_field);

        // define by_float field

        let by_float_field = dynamic::Field::new(
            "by_floats",
            dynamic::TypeRef::named_nn(dynamic::TypeRef::FLOAT),
            |ctx| {
                dynamic::FieldFuture::new(async move {
                    let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                    let arg_0 = ctx.args.try_get("input")?.deserialize()?;
                    let value = parent.resolve_by_floats(arg_0).await;
                    ResolveOwned::resolve_owned(value, &ctx)
                })
            },
        );

        let by_float_field = by_float_field.argument(dynamic::InputValue::new(
            "input",
            dynamic::TypeRef::named_nn_list_nn(dynamic::TypeRef::FLOAT),
        ));

        let object_type = object_type.field(by_float_field);

        // define by_bool field

        let by_bool_field = dynamic::Field::new(
            "by_bools",
            dynamic::TypeRef::named_nn(dynamic::TypeRef::BOOLEAN),
            |ctx| {
                dynamic::FieldFuture::new(async move {
                    let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                    let arg_0 = ctx.args.try_get("input")?.deserialize()?;
                    let value = parent.resolve_by_bools(arg_0).await;
                    ResolveOwned::resolve_owned(value, &ctx)
                })
            },
        );

        let by_bool_field = by_bool_field.argument(dynamic::InputValue::new(
            "input",
            dynamic::TypeRef::named_nn_list_nn(dynamic::TypeRef::BOOLEAN),
        ));

        let object_type = object_type.field(by_bool_field);

        // define by_id field

        let by_id_field = dynamic::Field::new(
            "by_ids",
            dynamic::TypeRef::named_nn(dynamic::TypeRef::ID),
            |ctx| {
                dynamic::FieldFuture::new(async move {
                    let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                    let arg_0 = ctx.args.try_get("input")?.deserialize()?;
                    let value = parent.resolve_by_ids(arg_0).await;
                    ResolveOwned::resolve_owned(value, &ctx)
                })
            },
        );

        let by_id_field = by_id_field.argument(dynamic::InputValue::new(
            "input",
            dynamic::TypeRef::named_nn_list_nn(dynamic::TypeRef::ID),
        ));

        let object_type = object_type.field(by_id_field);

        // register Foo object

        registry.register_object(object_type)
    }
}

pub fn create_schema() -> dynamic::Schema {
    let registry = Registry::new()
        .register::<Query>()
        .register::<Foo>()
        .register::<BarInput>();
    let schema = dynamic::Schema::build(Query::NAME, None, None);
    registry.build_schema(schema).finish().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema_utils::normalize_schema;
    use async_graphql::Variables;

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
                      by_objects(input: [BarInput!]!): String!
                      by_strings(input: [String!]!): String!
                      by_ints(input: [Int!]!): Int!
                      by_floats(input: [Float!]!): Float!
                      by_bools(input: [Boolean!]!): Boolean!
                      by_ids(input: [ID!]!): ID!
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
    async fn test_query_by_object() {
        let schema = create_schema();
        let query = r#"
            query($input: [BarInput!]!) {
                foo {
                    by_objects(input: $input)
                }
            }
        "#;

        let variables = Variables::from_json(serde_json::json!({
            "input": [
                { "bar": "world1" },
                { "bar": "world2" },
            ]
        }));
        let req = async_graphql::Request::new(query)
            .variables(variables)
            .root_value(dynamic::FieldValue::owned_any(create_root()));
        let res = schema.execute(req).await;
        let data = res.data.into_json().unwrap();
        assert_eq!(
            data,
            serde_json::json!({
                "foo": { "by_objects": "bar-inputs: [BarInput { bar: world1 },BarInput { bar: world2 }]" }
            }),
        );
    }

    #[tokio::test]
    async fn test_query_by_string() {
        let schema = create_schema();
        let query = r#"
            query($strings: [String!]!, $ids: [ID!]!) {
                foo {
                    by_strings(input: $strings)
                    by_ids(input: $ids)
                }
            }
        "#;

        let variables = Variables::from_json(serde_json::json!({
            "strings": ["item1", "item2"],
            "ids": ["id1", "id2"],
        }));

        let req = async_graphql::Request::new(query)
            .variables(variables)
            .root_value(dynamic::FieldValue::owned_any(create_root()));
        let res = schema.execute(req).await;
        let data = res.data.into_json().unwrap();
        assert_eq!(
            data,
            serde_json::json!({
                "foo": {
                    "by_strings": "strings: [item1,item2]",
                    "by_ids": "ids: [id1,id2]",
                }
            }),
        );
    }

    #[tokio::test]
    async fn test_query_by_number() {
        let schema = create_schema();
        let query = r#"
            query($bools: [Boolean!]!,$ints: [Int!]!, $floats: [Float!]!) {
                foo {
                    by_bools(input: $bools)
                    by_ints(input: $ints)
                    by_floats(input: $floats)
                }
            }
        "#;

        let variables = Variables::from_json(serde_json::json!({
            "bools": [true, false],
            "ints": [12, 13],
            "floats": [2.5, 3.25],
        }));

        let req = async_graphql::Request::new(query)
            .variables(variables)
            .root_value(dynamic::FieldValue::owned_any(create_root()));
        let res = schema.execute(req).await;
        let data = res.data.into_json().unwrap();
        assert_eq!(
            data,
            serde_json::json!({
                "foo": {
                    "by_bools": "bools: [true,false]",
                    "by_ints": "i32s: [12,13]",
                    "by_floats": "f32s: [2.5,3.25]",
                }
            }),
        );
    }

    #[tokio::test]
    async fn test_query_by_empty_object() {
        let schema = create_schema();
        let query = r#"
            query ($input: BarInput) {
                foo {
                    by_objects(input: $input)
                }
            }
        "#;
        let variables = Variables::from_json(serde_json::json!({
            "input": []
        }));
        let req = async_graphql::Request::new(query)
            .variables(variables)
            .root_value(dynamic::FieldValue::owned_any(create_root()));
        let res = schema.execute(req).await;
        let data = res.data.into_json().unwrap();
        assert_eq!(
            data,
            serde_json::json!({
                "foo": { "by_objects": "bar-inputs: []" }
            }),
        );
    }

    #[tokio::test]
    async fn test_query_by_empty_string() {
        let schema = create_schema();
        let query = r#"
            query($input: String) {
                foo {
                    by_strings(input: $input)
                    by_ids(input: $input)
                }
            }
        "#;

        let variables = Variables::from_json(serde_json::json!({
            "input": []
        }));

        let req = async_graphql::Request::new(query)
            .variables(variables)
            .root_value(dynamic::FieldValue::owned_any(create_root()));
        let res = schema.execute(req).await;
        let data = res.data.into_json().unwrap();
        assert_eq!(
            data,
            serde_json::json!({
                "foo": {
                    "by_strings": "strings: []",
                    "by_ids": "ids: []",
                }
            }),
        );
    }

    #[tokio::test]
    async fn test_query_by_empty_number() {
        let schema = create_schema();
        let query = r#"
            query($bools: [Int!]!, $ints: [Int!]!, $floats: [Float!]!) {
                foo {
                    by_bools(input: $bools)
                    by_ints(input: $ints)
                    by_floats(input: $floats)
                }
            }
        "#;

        let variables = Variables::from_json(serde_json::json!({
            "bools": [],
            "ints": [],
            "floats": [],
        }));

        let req = async_graphql::Request::new(query)
            .variables(variables)
            .root_value(dynamic::FieldValue::owned_any(create_root()));
        let res = schema.execute(req).await;
        let data = res.data.into_json().unwrap();
        assert_eq!(
            data,
            serde_json::json!({
                "foo": {
                    "by_bools": "bools: []",
                    "by_ints": "i32s: []",
                    "by_floats": "f32s: []",
                }
            }),
        );
    }
}
