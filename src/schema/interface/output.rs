use crate::schema::interface::utils::AnyBox;
use crate::schema::output_types::utils::{ResolveOwned, ResolveRef};
use crate::schema::registry::{
    ExpandObject, ExpandObjectContext, Interface, Object, Register, Registry,
};
use async_graphql::dynamic::FieldValue;
use async_graphql::{dynamic, Context};

// mark as interface
trait Node<'a>: ResolveOwned<'a> + ResolveRef<'a> {
    fn resolve_id(&self) -> &str;
}

struct FooNode {
    id: String,
}

impl FooNode {
    async fn resolve_name(&self) -> String {
        "foo".to_string()
    }
    async fn resolve_foo(&self) -> String {
        "foo".to_string()
    }
}

impl Node<'_> for FooNode {
    fn resolve_id(&self) -> &str {
        &self.id
    }
}

struct BarNode {
    id: String,
}

impl BarNode {
    async fn resolve_name(&self) -> String {
        "bar".to_string()
    }
    async fn resolve_bar(&self) -> String {
        "bar".to_string()
    }
}

impl Node<'_> for BarNode {
    fn resolve_id(&self) -> &str {
        &self.id
    }
}

struct Query;

impl Query {
    fn resolve_node(&self, id: &str) -> Option<NodeInterface> {
        match id {
            "foo-1" => Some(NodeInterface::new_owned(FooNode {
                id: "foo-1".to_string(),
            })),
            "bar-1" => Some(NodeInterface::new_owned(BarNode {
                id: "bar-1".to_string(),
            })),
            _ => None,
        }
    }
}

// generate

impl<'a> ResolveRef<'a> for FooNode {
    fn resolve_ref(&'a self, _ctx: &Context) -> async_graphql::Result<Option<FieldValue<'a>>> {
        Ok(Some(FieldValue::borrowed_any(self)))
    }
}

impl<'a> ResolveOwned<'a> for FooNode {
    fn resolve_owned(self, _ctx: &Context) -> async_graphql::Result<Option<FieldValue<'a>>> {
        Ok(Some(FieldValue::owned_any(self)))
    }
}

impl<'a> ResolveRef<'a> for BarNode {
    fn resolve_ref(&'a self, _ctx: &Context) -> async_graphql::Result<Option<FieldValue<'a>>> {
        Ok(Some(FieldValue::borrowed_any(self)))
    }
}

impl<'a> ResolveOwned<'a> for BarNode {
    fn resolve_owned(self, _ctx: &Context) -> async_graphql::Result<Option<FieldValue<'a>>> {
        Ok(Some(FieldValue::owned_any(self)))
    }
}

struct NodeInterface<'a>(AnyBox<'a>);

impl NodeInterface<'_> {
    fn new_owned<'a, T>(value: T) -> NodeInterface<'a>
    where
        T: Node<'a> + Object + Send + Sync + 'static,
    {
        NodeInterface(AnyBox::new_owned(value, <T as Object>::NAME.to_string()))
    }
}

impl NodeInterface<'_> {
    #[allow(dead_code)]
    fn new_borrowed<'a, T>(_value: &'a T) -> NodeInterface<'a>
    where
        T: Node<'a> + Send + Sync + Object,
    {
        todo!("NodeInterface::new_borrowed")
    }
}

impl<'a> ResolveOwned<'a> for NodeInterface<'a> {
    fn resolve_owned(self, _ctx: &Context) -> async_graphql::Result<Option<FieldValue<'a>>> {
        self.0.resolve_owned(_ctx)
    }
}

impl Interface for NodeInterface<'_> {
    const NAME: &'static str = "Node";

    fn register_fields(interface: dynamic::Interface) -> dynamic::Interface {
        interface.field(dynamic::InterfaceField::new(
            "id",
            dynamic::TypeRef::named_nn("String"),
        ))
    }
}

impl Register for NodeInterface<'_> {
    fn register(registry: Registry) -> Registry {
        let interface = dynamic::Interface::new(Self::NAME);
        let interface = Self::register_fields(interface);
        registry.register_interface(interface)
    }
}

// FooNode

impl Object for FooNode {
    const NAME: &'static str = "FooNode";
}

impl Register for FooNode {
    fn register(registry: Registry) -> Registry {
        let object_type = dynamic::Object::new(<Self as Object>::NAME);
        let object_type = object_type.implement("Node");

        // name field
        let name_field = dynamic::Field::new("name", dynamic::TypeRef::named_nn("String"), |ctx| {
            dynamic::FieldFuture::new(async move {
                let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                let value = parent.resolve_name().await;
                value.resolve_owned(&ctx)
            })
        });
        let object_type = object_type.field(name_field);

        // foo field
        let foo_field = dynamic::Field::new("foo", dynamic::TypeRef::named_nn("String"), |ctx| {
            dynamic::FieldFuture::new(async move {
                let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                let value = parent.resolve_foo().await;
                value.resolve_owned(&ctx)
            })
        });
        let object_type = object_type.field(foo_field);

        registry.register_object(object_type)
    }
}

struct NodeExpandFooNode;

impl ExpandObject for NodeExpandFooNode {
    type Target = FooNode;
}

impl Register for NodeExpandFooNode {
    fn register(registry: Registry) -> Registry {
        let id_field = dynamic::Field::new("id", dynamic::TypeRef::named_nn("String"), |ctx| {
            dynamic::FieldFuture::new(async move {
                let parent = ctx
                    .parent_value
                    .try_downcast_ref::<<Self as ExpandObject>::Target>()?;
                let value = parent.resolve_id();
                ResolveOwned::resolve_owned(value, &ctx)
            })
        });
        registry.update_object(
            <<Self as ExpandObject>::Target as Object>::NAME,
            |object| object.field(id_field),
            ExpandObjectContext::new("NodeExpandFooNode", "id"),
        )
    }
}

// BarNode

impl Object for BarNode {
    const NAME: &'static str = "BarNode";
}

impl Register for BarNode {
    fn register(registry: Registry) -> Registry {
        let object_type = dynamic::Object::new(<Self as Object>::NAME);
        let object_type = object_type.implement("Node");

        // name field
        let name_field = dynamic::Field::new("name", dynamic::TypeRef::named_nn("String"), |ctx| {
            dynamic::FieldFuture::new(async move {
                let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                let value = parent.resolve_name().await;
                value.resolve_owned(&ctx)
            })
        });
        let object_type = object_type.field(name_field);

        // bar field

        let bar_field = dynamic::Field::new("bar", dynamic::TypeRef::named_nn("String"), |ctx| {
            dynamic::FieldFuture::new(async move {
                let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                let value = parent.resolve_bar().await;
                value.resolve_owned(&ctx)
            })
        });

        let object_type = object_type.field(bar_field);

        registry.register_object(object_type)
    }
}

struct NodeExpandBarNode;

impl ExpandObject for NodeExpandBarNode {
    type Target = BarNode;
}

impl Register for NodeExpandBarNode {
    fn register(registry: Registry) -> Registry {
        let id_field = dynamic::Field::new("id", dynamic::TypeRef::named_nn("String"), |ctx| {
            dynamic::FieldFuture::new(async move {
                let parent = ctx
                    .parent_value
                    .try_downcast_ref::<<Self as ExpandObject>::Target>()?;
                let value = parent.resolve_id();
                ResolveOwned::resolve_owned(value, &ctx)
            })
        });
        registry.update_object(
            <<Self as ExpandObject>::Target as Object>::NAME,
            |object| object.field(id_field),
            ExpandObjectContext::new("NodeExpandBarNode", "id"),
        )
    }
}

// Query

impl Object for Query {
    const NAME: &'static str = "Query";
}

impl Register for Query {
    fn register(registry: Registry) -> Registry {
        let object_type = dynamic::Object::new(<Self as Object>::NAME);

        let node_field = dynamic::Field::new("node", dynamic::TypeRef::named("Node"), |ctx| {
            dynamic::FieldFuture::new(async move {
                let id: String = ctx.args.try_get("id")?.deserialize()?;
                let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                let value = parent.resolve_node(&id);
                ResolveOwned::resolve_owned(value, &ctx)
            })
        });
        // id input
        let id_arg = dynamic::InputValue::new("id", dynamic::TypeRef::named_nn("String"));
        let node_field = node_field.argument(id_arg);

        let object_type = object_type.field(node_field);

        registry.register_object(object_type)
    }
}

// Schema
pub fn create_schema() -> dynamic::Schema {
    let registry = Registry::new()
        .register::<Query>()
        .register::<NodeInterface>()
        .register::<FooNode>()
        .register::<BarNode>()
        .register::<NodeExpandFooNode>()
        .register::<NodeExpandBarNode>();
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
                type BarNode implements Node {
                  name: String!
                  bar: String!
                  id: String!
                }
                type FooNode implements Node {
                  name: String!
                  foo: String!
                  id: String!
                }
                interface Node {
                  id: String!
                }
                type Query {
                  node(id: String!): Node
                }
                schema {
                  query: Query
                }

            "#
            ),
        );
    }

    fn create_root() -> Query {
        Query
    }

    #[tokio::test]
    async fn test_foo_query() {
        let schema = create_schema();
        let query = r#"
        query {
            node(id: "foo-1") {
                __typename
                id
                ... on FooNode {
                    name
                    foo
                }
                ... on BarNode {
                    name
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
                "node": {
                    "__typename": "FooNode",
                    "id": "foo-1",
                    "name": "foo",
                    "foo": "foo",
                }
            }),
        );
    }
    #[tokio::test]
    async fn test_bar_query() {
        let schema = create_schema();
        let query = r#"
        query {
            node(id: "bar-1") {
                __typename
                id
                ... on FooNode {
                    name
                    foo
                }
                ... on BarNode {
                    name
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
                "node": {
                    "__typename": "BarNode",
                    "id": "bar-1",
                    "name": "bar",
                    "bar": "bar",
                }
            }),
        );
    }

    #[tokio::test]
    async fn test_null_query() {
        let schema = create_schema();
        let query = r#"
        query {
            node(id: "bar-2") {
                __typename
                id
                ... on FooNode {
                    name
                    foo
                }
                ... on BarNode {
                    name
                    bar
                }
            }
        }
       "#;
        let req =
            async_graphql::Request::new(query).root_value(FieldValue::owned_any(create_root()));
        let res = schema.execute(req).await;
        let data = res.data.into_json().unwrap();
        println!("error: {:?}", res.errors);
        assert_eq!(
            data,
            serde_json::json!({
                "node": null,
            }),
        );
    }
}
