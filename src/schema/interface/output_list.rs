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

struct Query {
    foo: FooNode,
    bar: BarNode,
}

impl Query {
    fn resolve_nodes(&self) -> Vec<NodeInterface> {
        vec![
            NodeInterface::new_borrowed(&self.foo),
            NodeInterface::new_borrowed(&self.bar),
        ]
    }
    fn resolve_new_nodes(&self) -> Vec<NodeInterface> {
        vec![
            NodeInterface::new_owned(FooNode {
                id: "foo-2".to_string(),
            }),
            NodeInterface::new_owned(BarNode {
                id: "bar-2".to_string(),
            }),
        ]
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
    fn new_borrowed<'a, T>(value: &'a T) -> NodeInterface<'a>
    where
        T: Node<'a> + Send + Sync + Object + 'static,
    {
        NodeInterface(AnyBox::new_borrowed(value, <T as Object>::NAME.to_string()))
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

        let node_field =
            dynamic::Field::new("nodes", dynamic::TypeRef::named_nn_list_nn("Node"), |ctx| {
                dynamic::FieldFuture::new(async move {
                    let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                    let value = parent.resolve_nodes();
                    ResolveOwned::resolve_owned(value, &ctx)
                })
            });

        let object_type = object_type.field(node_field);

        // new nodes field

        let nodes_field = dynamic::Field::new(
            "new_nodes",
            dynamic::TypeRef::named_nn_list_nn("Node"),
            |ctx| {
                dynamic::FieldFuture::new(async move {
                    let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                    let value = parent.resolve_new_nodes();
                    ResolveOwned::resolve_owned(value, &ctx)
                })
            },
        );

        let object_type = object_type.field(nodes_field);

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
                  nodes: [Node!]!
                  new_nodes: [Node!]!
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
            foo: FooNode {
                id: "foo-1".to_string(),
            },
            bar: BarNode {
                id: "bar-1".to_string(),
            },
        }
    }

    #[tokio::test]
    async fn test_nodes_query() {
        let schema = create_schema();
        let query = r#"
        query {
            nodes {
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
                "nodes": [
                    {
                        "__typename": "FooNode",
                        "id": "foo-1",
                        "name": "foo",
                        "foo": "foo",
                    },
                    {
                        "__typename": "BarNode",
                        "id": "bar-1",
                        "name": "bar",
                        "bar": "bar",
                    },
                ]
            }),
        );
    }

    #[tokio::test]
    async fn test_new_nodes_query() {
        let schema = create_schema();
        let query = r#"
        query {
            new_nodes {
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
                "new_nodes": [
                    {
                        "__typename": "FooNode",
                        "id": "foo-2",
                        "name": "foo",
                        "foo": "foo",
                    },
                    {
                        "__typename": "BarNode",
                        "id": "bar-2",
                        "name": "bar",
                        "bar": "bar",
                    },
                ]
            }),
        );
    }
}
