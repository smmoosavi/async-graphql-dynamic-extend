use crate::schema::output_types::utils::{ResolveOwned, ResolveRef};
use crate::schema::registry::{Object, Register, Registry};
use async_graphql::dynamic::DynamicRequestExt;
use async_graphql::dynamic::FieldValue;
use async_graphql::{dynamic, Context, ID};

struct Query {
    foo: Foo,
}

struct Foo {
    bar: Vec<Bar>,

    the_string: Vec<String>,
    the_str: Vec<&'static str>,
    the_i32: Vec<i32>,
    the_f32: Vec<f32>,
    the_bool: Vec<bool>,
    the_id: Vec<ID>,
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

impl<'a> ResolveOwned<'a> for Bar {
    fn resolve_owned(self, _ctx: &Context) -> async_graphql::Result<Option<FieldValue<'a>>> {
        Ok(Some(FieldValue::owned_any(self)))
    }
}

impl<'a> ResolveRef<'a> for Bar {
    fn resolve_ref(&'a self, _ctx: &Context) -> async_graphql::Result<Option<FieldValue<'a>>> {
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
                    let value = parent.resolve_value().await;
                    ResolveRef::resolve_ref(value, &ctx)
                })
            },
        );
        let object_type = object_type.field(value_field);

        // register Bar object
        registry.register_object(object_type)
    }
}

impl Bar {
    async fn resolve_value(&self) -> &String {
        &self.value
    }
}

impl<'a> ResolveOwned<'a> for Foo {
    fn resolve_owned(self, _ctx: &Context) -> async_graphql::Result<Option<FieldValue<'a>>> {
        Ok(Some(FieldValue::owned_any(self)))
    }
}

impl<'a> ResolveRef<'a> for Foo {
    fn resolve_ref(&'a self, _ctx: &Context) -> async_graphql::Result<Option<FieldValue<'a>>> {
        Ok(Some(FieldValue::borrowed_any(self)))
    }
}

impl Register for Foo {
    fn register(registry: Registry) -> Registry {
        // define Foo object
        let object_type = dynamic::Object::new(Self::NAME);

        // define bar field
        let bar_field = dynamic::Field::new(
            "bar",
            dynamic::TypeRef::named_nn_list_nn(Bar::NAME),
            |ctx| {
                dynamic::FieldFuture::new(async move {
                    let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                    let value = parent.resolve_bar().await;
                    ResolveRef::resolve_ref(value, &ctx)
                })
            },
        );
        let object_type = object_type.field(bar_field);

        // define the_string field
        let the_string_field = dynamic::Field::new(
            "the_string",
            dynamic::TypeRef::named_nn_list_nn(dynamic::TypeRef::STRING),
            |ctx| {
                dynamic::FieldFuture::new(async move {
                    let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                    let value = parent.resolve_the_string().await;
                    ResolveRef::resolve_ref(value, &ctx)
                })
            },
        );
        let object_type = object_type.field(the_string_field);

        // define the_str field
        let the_str_field = dynamic::Field::new(
            "the_str",
            dynamic::TypeRef::named_nn_list_nn(dynamic::TypeRef::STRING),
            |ctx| {
                dynamic::FieldFuture::new(async move {
                    let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                    let value = parent.resolve_the_str().await;
                    ResolveRef::resolve_ref(value, &ctx)
                })
            },
        );
        let object_type = object_type.field(the_str_field);

        // define the_i32 field
        let the_i32_field = dynamic::Field::new(
            "the_i32",
            dynamic::TypeRef::named_nn_list_nn(dynamic::TypeRef::INT),
            |ctx| {
                dynamic::FieldFuture::new(async move {
                    let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                    let value = parent.resolve_the_i32().await;
                    ResolveRef::resolve_ref(value, &ctx)
                })
            },
        );

        let object_type = object_type.field(the_i32_field);

        // define the_f32 field
        let the_f32_field = dynamic::Field::new(
            "the_f32",
            dynamic::TypeRef::named_nn_list_nn(dynamic::TypeRef::FLOAT),
            |ctx| {
                dynamic::FieldFuture::new(async move {
                    let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                    let value = parent.resolve_the_f32().await;
                    ResolveRef::resolve_ref(value, &ctx)
                })
            },
        );
        let object_type = object_type.field(the_f32_field);

        // define the_bool field
        let the_bool_field = dynamic::Field::new(
            "the_bool",
            dynamic::TypeRef::named_nn_list_nn(dynamic::TypeRef::BOOLEAN),
            |ctx| {
                dynamic::FieldFuture::new(async move {
                    let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                    let value = parent.resolve_the_bool().await;
                    ResolveRef::resolve_ref(value, &ctx)
                })
            },
        );
        let object_type = object_type.field(the_bool_field);

        // define the_id field
        let the_id_field = dynamic::Field::new(
            "the_id",
            dynamic::TypeRef::named_nn_list_nn(dynamic::TypeRef::ID),
            |ctx| {
                dynamic::FieldFuture::new(async move {
                    let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                    let value = parent.resolve_the_id().await;
                    ResolveRef::resolve_ref(value, &ctx)
                })
            },
        );
        let object_type = object_type.field(the_id_field);

        // register Foo object

        registry.register_object(object_type)
    }
}

impl Foo {
    async fn resolve_bar(&self) -> &Vec<Bar> {
        &self.bar
    }
    async fn resolve_the_string(&self) -> &Vec<String> {
        &self.the_string
    }
    async fn resolve_the_str(&self) -> &Vec<&'static str> {
        &self.the_str
    }
    async fn resolve_the_i32(&self) -> &Vec<i32> {
        &self.the_i32
    }
    async fn resolve_the_f32(&self) -> &Vec<f32> {
        &self.the_f32
    }
    async fn resolve_the_bool(&self) -> &Vec<bool> {
        &self.the_bool
    }
    async fn resolve_the_id(&self) -> &Vec<ID> {
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
                      bar: [Bar!]!
                      the_string: [String!]!
                      the_str: [String!]!
                      the_i32: [Int!]!
                      the_f32: [Float!]!
                      the_bool: [Boolean!]!
                      the_id: [ID!]!
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
                bar: vec![Bar {
                    value: "bar".to_owned(),
                }],
                the_string: vec!["the_string".to_owned()],
                the_str: vec!["the_str"],
                the_i32: vec![42],
                the_f32: vec![42.0],
                the_bool: vec![true],
                the_id: vec!["the_id".into()],
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
                "foo": { "bar": [{ "value": "bar" }] },
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
                    "the_string": ["the_string"],
                    "the_str": ["the_str"],
                    "the_id": ["the_id"],
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
                    "the_i32": [42],
                    "the_f32": [42.0],
                    "the_bool": [true],
                }
            }),
        );
    }
    // empty

    fn create_empty_root() -> Query {
        Query {
            foo: Foo {
                bar: vec![],
                the_string: vec![],
                the_str: vec![],
                the_i32: vec![],
                the_f32: vec![],
                the_bool: vec![],
                the_id: vec![],
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
                "foo": { "bar": [] },
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
                    "the_string": [],
                    "the_str": [],
                    "the_id": [],
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
                    "the_i32": [],
                    "the_f32": [],
                    "the_bool": [],
                }
            }),
        );
    }
}
