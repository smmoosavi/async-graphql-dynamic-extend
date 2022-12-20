use crate::schema::output_types::utils::{ResolveOwned, ResolveRef};
use crate::schema::registry::{Object, Register, Registry, Union};
use async_graphql::dynamic::{FieldFuture, FieldValue, Schema};
use async_graphql::{dynamic, Context};

struct Foo {
    foo: String,
}

struct Bar {
    bar: String,
}

enum FooBar {
    Foo(Foo),
    Bar(Bar),
}

struct Query {
    foobar: FooBar,
}

impl Query {
    async fn resolve_new_foobar(&self, _ctx: &Context<'_>) -> FooBar {
        FooBar::Bar(Bar {
            bar: "bar".to_string(),
        })
    }
}

// generated

impl Object for Foo {
    const NAME: &'static str = "Foo";
}

impl Object for Bar {
    const NAME: &'static str = "Bar";
}

impl Union for FooBar {
    const NAME: &'static str = "FooBar";
}

impl Object for Query {
    const NAME: &'static str = "Query";
}

impl<'a> ResolveRef<'a> for Foo {
    fn resolve_ref(&'a self, _ctx: &Context) -> async_graphql::Result<Option<FieldValue<'a>>> {
        Ok(Some(FieldValue::borrowed_any(self)))
    }
}

impl<'a> ResolveRef<'a> for Bar {
    fn resolve_ref(&'a self, _ctx: &Context) -> async_graphql::Result<Option<FieldValue<'a>>> {
        Ok(Some(FieldValue::borrowed_any(self)))
    }
}

impl<'a> ResolveOwned<'a> for Foo {
    fn resolve_owned(self, _ctx: &Context) -> async_graphql::Result<Option<FieldValue<'a>>> {
        Ok(Some(FieldValue::owned_any(self)))
    }
}

impl<'a> ResolveOwned<'a> for Bar {
    fn resolve_owned(self, _ctx: &Context) -> async_graphql::Result<Option<FieldValue<'a>>> {
        Ok(Some(FieldValue::owned_any(self)))
    }
}

impl<'a> ResolveRef<'a> for FooBar {
    fn resolve_ref(&'a self, _ctx: &Context) -> async_graphql::Result<Option<FieldValue<'a>>> {
        match self {
            FooBar::Foo(v) => Ok(Some(
                FieldValue::borrowed_any(v).with_type(<Foo as Object>::NAME),
            )),
            FooBar::Bar(v) => Ok(Some(
                FieldValue::borrowed_any(v).with_type(<Bar as Object>::NAME),
            )),
        }
    }
}

impl<'a> ResolveOwned<'a> for FooBar {
    fn resolve_owned(self, _ctx: &Context) -> async_graphql::Result<Option<FieldValue<'a>>> {
        match self {
            FooBar::Foo(v) => Ok(Some(
                FieldValue::owned_any(v).with_type(<Foo as Object>::NAME),
            )),
            FooBar::Bar(v) => Ok(Some(
                FieldValue::owned_any(v).with_type(<Bar as Object>::NAME),
            )),
        }
    }
}

// resolves
impl Foo {
    async fn resolve_foo(&self, _ctx: &Context<'_>) -> &String {
        &self.foo
    }
}

impl Bar {
    async fn resolve_bar(&self, _ctx: &Context<'_>) -> &String {
        &self.bar
    }
}

impl Query {
    async fn resolve_foobar(&self, _ctx: &Context<'_>) -> &FooBar {
        &self.foobar
    }
}

// register types

impl Register for Foo {
    fn register(registry: Registry) -> Registry {
        let object_type = dynamic::Object::new(<Foo as Object>::NAME);

        // foo field
        let foo_field = dynamic::Field::new(
            "foo",
            dynamic::TypeRef::named_nn(dynamic::TypeRef::STRING),
            |ctx| {
                FieldFuture::new(async move {
                    let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                    let value = parent.resolve_foo(&ctx).await;
                    ResolveRef::resolve_ref(value, &ctx)
                })
            },
        );
        let object_type = object_type.field(foo_field);

        registry.register_object(object_type)
    }
}

impl Register for Bar {
    fn register(registry: Registry) -> Registry {
        let object_type = dynamic::Object::new(<Bar as Object>::NAME);

        // bar field
        let bar_field = dynamic::Field::new(
            "bar",
            dynamic::TypeRef::named_nn(dynamic::TypeRef::STRING),
            |ctx| {
                FieldFuture::new(async move {
                    let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                    let value = parent.resolve_bar(&ctx).await;
                    ResolveRef::resolve_ref(value, &ctx)
                })
            },
        );
        let object_type = object_type.field(bar_field);

        registry.register_object(object_type)
    }
}

impl Register for FooBar {
    fn register(registry: Registry) -> Registry {
        let union_type = dynamic::Union::new(<FooBar as Union>::NAME);
        let union_type = union_type.possible_type(<Foo as Object>::NAME);
        let union_type = union_type.possible_type(<Bar as Object>::NAME);
        registry.register_union(union_type)
    }
}

impl Register for Query {
    fn register(registry: Registry) -> Registry {
        let object_type = dynamic::Object::new(<Query as Object>::NAME);

        // foobar field
        let foobar_field = dynamic::Field::new(
            "foobar",
            dynamic::TypeRef::named_nn(<FooBar as Union>::NAME),
            |ctx| {
                FieldFuture::new(async move {
                    let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                    let value = parent.resolve_foobar(&ctx).await;
                    ResolveRef::resolve_ref(value, &ctx)
                })
            },
        );
        let object_type = object_type.field(foobar_field);

        // new_foobar field
        let new_foobar_field = dynamic::Field::new(
            "new_foobar",
            dynamic::TypeRef::named_nn(<FooBar as Union>::NAME),
            |ctx| {
                FieldFuture::new(async move {
                    let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                    let value = parent.resolve_new_foobar(&ctx).await;
                    ResolveOwned::resolve_owned(value, &ctx)
                })
            },
        );

        let object_type = object_type.field(new_foobar_field);

        registry.register_object(object_type)
    }
}

// register schema

fn create_schema() -> Schema {
    let registry = Registry::new();
    let registry = registry
        .register::<Query>()
        .register::<Foo>()
        .register::<Bar>()
        .register::<FooBar>();
    let schema = dynamic::Schema::build(Query::NAME, None, None);
    registry.build_schema(schema).finish().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema_utils::normalize_schema;
    use async_graphql::dynamic::DynamicRequestExt;

    #[test]
    fn test_schema() {
        let schema = create_schema();
        let sdl = schema.sdl();
        assert_eq!(
            normalize_schema(&sdl),
            normalize_schema(
                r#"
            type Bar {
              bar: String!
            }
            type Foo {
              foo: String!
            }
            union FooBar = Foo | Bar
            type Query {
              foobar: FooBar!
              new_foobar: FooBar!
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
            foobar: FooBar::Foo(Foo {
                foo: "foo".to_string(),
            }),
        }
    }
    #[tokio::test]
    async fn test_query_ref() {
        let schema = create_schema();
        let query = r#"
            query {
                foobar {
                    ... on Foo {
                        foo
                    }
                    ... on Bar {
                        bar
                    }
                }
            }
        "#;

        let req =
            async_graphql::Request::new(query).root_value(FieldValue::owned_any(create_root()));
        let res = schema.execute(req).await;
        let data = res.data.into_json().unwrap();
        assert_eq!(
            data,
            serde_json::json!({
                "foobar": {
                    "foo": "foo"
                }
            }),
        );
    }

    #[tokio::test]
    async fn test_query_owned() {
        let schema = create_schema();
        let query = r#"
            query {
                new_foobar {
                    ... on Foo {
                        foo
                    }
                    ... on Bar {
                        bar
                    }
                }
            }
        "#;

        let req =
            async_graphql::Request::new(query).root_value(FieldValue::owned_any(create_root()));
        let res = schema.execute(req).await;
        let data = res.data.into_json().unwrap();
        assert_eq!(
            data,
            serde_json::json!({
                "new_foobar": {
                    "bar": "bar"
                }
            }),
        );
    }
}
