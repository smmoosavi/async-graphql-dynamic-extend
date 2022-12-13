use crate::schema::output_types::utils::OutputValue;
use crate::schema::registry::{Object, Register, Registry};
use async_graphql::dynamic::DynamicRequestExt;
use async_graphql::dynamic::FieldValue;
use async_graphql::{dynamic, Context, ID};

struct Query {
    foo: Foo,
}

struct Foo {
    bar: Option<Bar>,

    the_string: Option<String>,
    the_str: Option<&'static str>,
    the_i32: Option<i32>,
    the_f32: Option<f32>,
    the_bool: Option<bool>,
    the_id: Option<ID>,
}

struct Bar {
    value: String,
}

// generated

impl Object for Query {
    const NAME: &'static str = "Query";
}

impl Object for Foo {
    const NAME: &'static str = "Foo";
}

impl Object for Bar {
    const NAME: &'static str = "Bar";
}

impl Register for Query {
    fn register(registry: Registry) -> Registry {
        // define Query object
        let query_object = dynamic::Object::new("Query");

        // define foo field
        let foo_field = dynamic::Field::new("foo", dynamic::TypeRef::named_nn(Foo::NAME), |ctx| {
            dynamic::FieldFuture::new(async move {
                let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                let value = parent.resolve_foo();
                value.resolve(&ctx)
            })
        });
        let query_object = query_object.field(foo_field);

        // register Query object
        registry.register_object(query_object)
    }
}

impl Query {
    fn resolve_foo(&self) -> &Foo {
        &self.foo
    }
}

impl OutputValue for Bar {
    fn resolve(&self, _ctx: &Context) -> async_graphql::Result<Option<FieldValue>> {
        Ok(Some(FieldValue::borrowed_any(self)))
    }
}

impl Register for Bar {
    fn register(registry: Registry) -> Registry {
        // define Bar object
        let object_type = dynamic::Object::new(Self::NAME);

        // define value field
        let value_field = dynamic::Field::new(
            "value",
            dynamic::TypeRef::named_nn(dynamic::TypeRef::STRING),
            |ctx| {
                dynamic::FieldFuture::new(async move {
                    let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                    let value = parent.resolve_value();
                    value.resolve(&ctx)
                })
            },
        );
        let object_type = object_type.field(value_field);

        // register Bar object
        registry.register_object(object_type)
    }
}

impl Bar {
    fn resolve_value(&self) -> &String {
        &self.value
    }
}

impl OutputValue for Foo {
    fn resolve(&self, _ctx: &Context) -> async_graphql::Result<Option<FieldValue>> {
        Ok(Some(FieldValue::borrowed_any(self)))
    }
}

impl Register for Foo {
    fn register(registry: Registry) -> Registry {
        // define Foo object
        let object_type = dynamic::Object::new(Self::NAME);

        // define bar field
        let bar_field = dynamic::Field::new("bar", dynamic::TypeRef::named(Bar::NAME), |ctx| {
            dynamic::FieldFuture::new(async move {
                let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                let value = parent.resolve_bar();
                value.resolve(&ctx)
            })
        });
        let object_type = object_type.field(bar_field);

        // define the_string field
        let the_string_field = dynamic::Field::new(
            "the_string",
            dynamic::TypeRef::named(dynamic::TypeRef::STRING),
            |ctx| {
                dynamic::FieldFuture::new(async move {
                    let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                    let value = parent.resolve_the_string();
                    value.resolve(&ctx)
                })
            },
        );
        let object_type = object_type.field(the_string_field);

        // define the_str field
        let the_str_field = dynamic::Field::new(
            "the_str",
            dynamic::TypeRef::named(dynamic::TypeRef::STRING),
            |ctx| {
                dynamic::FieldFuture::new(async move {
                    let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                    let value = parent.resolve_the_str();
                    value.resolve(&ctx)
                })
            },
        );
        let object_type = object_type.field(the_str_field);

        // define the_i32 field
        let the_i32_field = dynamic::Field::new(
            "the_i32",
            dynamic::TypeRef::named(dynamic::TypeRef::INT),
            |ctx| {
                dynamic::FieldFuture::new(async move {
                    let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                    let value = parent.resolve_the_i32();
                    value.resolve(&ctx)
                })
            },
        );

        let object_type = object_type.field(the_i32_field);

        // define the_f32 field
        let the_f32_field = dynamic::Field::new(
            "the_f32",
            dynamic::TypeRef::named(dynamic::TypeRef::FLOAT),
            |ctx| {
                dynamic::FieldFuture::new(async move {
                    let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                    let value = parent.resolve_the_f32();
                    value.resolve(&ctx)
                })
            },
        );
        let object_type = object_type.field(the_f32_field);

        // define the_bool field
        let the_bool_field = dynamic::Field::new(
            "the_bool",
            dynamic::TypeRef::named(dynamic::TypeRef::BOOLEAN),
            |ctx| {
                dynamic::FieldFuture::new(async move {
                    let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                    let value = parent.resolve_the_bool();
                    value.resolve(&ctx)
                })
            },
        );
        let object_type = object_type.field(the_bool_field);

        // define the_id field
        let the_id_field = dynamic::Field::new(
            "the_id",
            dynamic::TypeRef::named(dynamic::TypeRef::ID),
            |ctx| {
                dynamic::FieldFuture::new(async move {
                    let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                    let value = parent.resolve_the_id();
                    value.resolve(&ctx)
                })
            },
        );
        let object_type = object_type.field(the_id_field);

        // register Foo object

        registry.register_object(object_type)
    }
}

impl Foo {
    fn resolve_bar(&self) -> &Option<Bar> {
        &self.bar
    }
    fn resolve_the_string(&self) -> &Option<String> {
        &self.the_string
    }
    fn resolve_the_str(&self) -> &Option<&'static str> {
        &self.the_str
    }
    fn resolve_the_i32(&self) -> &Option<i32> {
        &self.the_i32
    }
    fn resolve_the_f32(&self) -> &Option<f32> {
        &self.the_f32
    }
    fn resolve_the_bool(&self) -> &Option<bool> {
        &self.the_bool
    }
    fn resolve_the_id(&self) -> &Option<ID> {
        &self.the_id
    }
}

pub fn create_schema() -> dynamic::Schema {
    let registry = Registry::new()
        .register::<Query>()
        .register::<Foo>()
        .register::<Bar>();
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
                    type Bar {
                      value: String!
                    }
                    type Foo {
                      bar: Bar
                      the_string: String
                      the_str: String
                      the_i32: Int
                      the_f32: Float
                      the_bool: Boolean
                      the_id: ID
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
        Query {
            foo: Foo {
                bar: Some(Bar {
                    value: "bar".to_owned(),
                }),
                the_string: Some("the_string".to_owned()),
                the_str: Some("the_str"),
                the_i32: Some(42),
                the_f32: Some(42.0),
                the_bool: Some(true),
                the_id: Some("the_id".into()),
            },
        }
    }

    #[tokio::test]
    async fn test_query_object() {
        let schema = create_schema();
        let query = r#"
            query {
              foo { bar { value } }
            }
        "#;

        let req =
            async_graphql::Request::new(query).root_value(FieldValue::owned_any(create_root()));
        let res = schema.execute(req).await;
        let data = res.data.into_json().unwrap();
        assert_eq!(
            data,
            serde_json::json!({
                "foo": { "bar": { "value": "bar" } },
            }),
        );
    }
    #[tokio::test]
    async fn test_query_strings() {
        let schema = create_schema();
        let query = r#"
            query {
              foo { the_string the_str the_id }
            }
        "#;
        let req =
            async_graphql::Request::new(query).root_value(FieldValue::owned_any(create_root()));
        let res = schema.execute(req).await;
        let data = res.data.into_json().unwrap();
        assert_eq!(
            data,
            serde_json::json!({
                "foo": {
                    "the_string": "the_string",
                    "the_str": "the_str",
                    "the_id": "the_id",
                }
            }),
        );
    }

    #[tokio::test]
    async fn test_query_scalars() {
        let schema = create_schema();
        let query = r#"
            query {
              foo { the_i32 the_f32 the_bool }
            }
        "#;
        let req =
            async_graphql::Request::new(query).root_value(FieldValue::owned_any(create_root()));
        let res = schema.execute(req).await;
        let data = res.data.into_json().unwrap();
        assert_eq!(
            data,
            serde_json::json!({
                "foo": {
                    "the_i32": 42,
                    "the_f32": 42.0,
                    "the_bool": true,
                }
            }),
        );
    }
    // empty

    fn create_empty_root() -> Query {
        Query {
            foo: Foo {
                bar: None,
                the_string: None,
                the_str: None,
                the_i32: None,
                the_f32: None,
                the_bool: None,
                the_id: None,
            },
        }
    }

    #[tokio::test]
    async fn test_query_empty_object() {
        let schema = create_schema();
        let query = r#"
            query {
              foo { bar { value } }
            }
        "#;

        let req = async_graphql::Request::new(query)
            .root_value(FieldValue::owned_any(create_empty_root()));
        let res = schema.execute(req).await;
        let data = res.data.into_json().unwrap();
        assert_eq!(
            data,
            serde_json::json!({
                "foo": { "bar": null },
            }),
        );
    }
    #[tokio::test]
    async fn test_query_empty_strings() {
        let schema = create_schema();
        let query = r#"
            query {
              foo { the_string the_str the_id }
            }
        "#;
        let req = async_graphql::Request::new(query)
            .root_value(FieldValue::owned_any(create_empty_root()));
        let res = schema.execute(req).await;
        let data = res.data.into_json().unwrap();
        assert_eq!(
            data,
            serde_json::json!({
                "foo": {
                    "the_string": null,
                    "the_str": null,
                    "the_id": null,
                }
            }),
        );
    }

    #[tokio::test]
    async fn test_query_empty_scalars() {
        let schema = create_schema();
        let query = r#"
            query {
              foo { the_i32 the_f32 the_bool }
            }
        "#;
        let req = async_graphql::Request::new(query)
            .root_value(FieldValue::owned_any(create_empty_root()));
        let res = schema.execute(req).await;
        let data = res.data.into_json().unwrap();
        assert_eq!(
            data,
            serde_json::json!({
                "foo": {
                    "the_i32": null,
                    "the_f32": null,
                    "the_bool": null,
                }
            }),
        );
    }
}
