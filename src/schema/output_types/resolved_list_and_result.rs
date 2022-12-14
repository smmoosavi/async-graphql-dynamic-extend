use crate::schema::output_types::utils::{ResolveOwned, ResolveRef};
use crate::schema::registry::{Object, Register, Registry};
use async_graphql::dynamic::DynamicRequestExt;
use async_graphql::dynamic::FieldValue;
use async_graphql::{dynamic, Context};

struct Query {
    foo: Foo,
}

struct Foo {
    error: bool,
    bars: Vec<Bar>,
}

#[derive(thiserror::Error, Debug)]
enum MyError {
    #[error("bar error")]
    Bar,
}

impl Foo {
    async fn resolve_bars(&self) -> Result<Vec<Bar>, MyError> {
        if self.error {
            Err(MyError::Bar)
        } else {
            Ok(self.bars.clone())
        }
    }
}

#[derive(Clone)]
struct Bar {
    error: bool,
    value: String,
}

impl Bar {
    async fn resolve_value(&self) -> Result<String, MyError> {
        if self.error {
            Err(MyError::Bar)
        } else {
            Ok(self.value.clone())
        }
    }
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
                    ResolveOwned::resolve_owned(value, &ctx)
                })
            },
        );
        let object_type = object_type.field(value_field);

        // register Bar object
        registry.register_object(object_type)
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

        // define bars field
        let bars_field =
            dynamic::Field::new("bars", dynamic::TypeRef::named_list_nn(Bar::NAME), |ctx| {
                dynamic::FieldFuture::new(async move {
                    let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                    let value = parent.resolve_bars().await;
                    ResolveOwned::resolve_owned(value, &ctx)
                })
            });
        let object_type = object_type.field(bars_field);

        // register Foo object

        registry.register_object(object_type)
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
                      bars: [Bar]!
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
                error: false,
                bars: vec![
                    Bar {
                        error: false,
                        value: "bar1".to_string(),
                    },
                    Bar {
                        error: false,
                        value: "bar2".to_string(),
                    },
                ],
            },
        }
    }

    #[tokio::test]
    async fn test_query_object() {
        let schema = create_schema();
        let query = r#"
            query {
              foo { bars { value } }
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
                    "bars":
                    [{ "value": "bar1" }, { "value": "bar2" } ]
                }
            }),
        );
    }

    fn create_error_root() -> Query {
        Query {
            foo: Foo {
                error: false,
                bars: vec![
                    Bar {
                        error: false,
                        value: "bar1".to_string(),
                    },
                    Bar {
                        error: true,
                        value: "bar2".to_string(),
                    },
                ],
            },
        }
    }

    #[tokio::test]
    async fn test_query_error_object() {
        let schema = create_schema();
        let query = r#"
            query {
              foo { bars { value } }
            }
        "#;

        let req = async_graphql::Request::new(query)
            .root_value(FieldValue::owned_any(create_error_root()));
        let res = schema.execute(req).await;
        let data = res.data.into_json().unwrap();
        let errors = serde_json::to_value(res.errors).unwrap();
        assert_eq!(
            errors,
            serde_json::json!([
                {
                    "locations": [{ "column": 28, "line": 3 }],
                    "message": "bar error",
                }
            ]),
        );
        assert_eq!(data, serde_json::json!(null),);
    }
}
