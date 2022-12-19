use crate::schema::output_types::utils::{ResolveOwned, ResolveRef};
use crate::schema::registry::{Interface, Object, Register, Registry};
use async_graphql::dynamic;
use async_graphql::dynamic::FieldValue;

// user
// mark as interface
#[allow(dead_code)]
pub struct Node {
    id: String,
}

// mark as interface, implements Node
#[allow(dead_code)]
pub struct NamedNode {
    name: String,
}

// mark as interface
#[allow(dead_code)]
pub struct Aged {
    age: i32,
}

pub struct Query {
    user: User,
}

// mark as object, implements Node, NamedNode, Aged
pub struct User {
    // mark as skip
    id: String,
    // mark as skip
    name: String,
    // mark as skip
    age: i32,
}

// generated

impl Object for Query {
    const NAME: &'static str = "Query";
}

impl Object for User {
    const NAME: &'static str = "User";
}

impl Interface for Node {
    const NAME: &'static str = "Node";

    fn register_fields(interface: dynamic::Interface) -> dynamic::Interface {
        let id_field = dynamic::InterfaceField::new(
            "id",
            dynamic::TypeRef::named_nn(dynamic::TypeRef::STRING),
        );
        interface.field(id_field)
    }
}

impl Register for Node {
    fn register(registry: Registry) -> Registry {
        let interface = dynamic::Interface::new(Self::NAME);
        let interface = Self::register_fields(interface);
        registry.register_interface(interface)
    }
}

impl Interface for NamedNode {
    const NAME: &'static str = "NamedNode";

    fn register_fields(interface: dynamic::Interface) -> dynamic::Interface {
        let name_field = dynamic::InterfaceField::new(
            "name",
            dynamic::TypeRef::named_nn(dynamic::TypeRef::STRING),
        );
        let interface = interface.field(name_field);

        // register parent interface
        Node::register_fields(interface)
    }
}

impl Register for NamedNode {
    fn register(registry: Registry) -> Registry {
        let interface = dynamic::Interface::new(Self::NAME);
        let interface = interface.implement(Node::NAME);
        let interface = Self::register_fields(interface);
        registry.register_interface(interface)
    }
}

impl Interface for Aged {
    const NAME: &'static str = "Aged";

    fn register_fields(interface: dynamic::Interface) -> dynamic::Interface {
        let age_field = dynamic::InterfaceField::new(
            "age",
            dynamic::TypeRef::named_nn(dynamic::TypeRef::STRING),
        );
        interface.field(age_field)
    }
}

impl Register for Aged {
    fn register(registry: Registry) -> Registry {
        let interface = dynamic::Interface::new(Self::NAME);
        let interface = Self::register_fields(interface);
        registry.register_interface(interface)
    }
}

// impl Query
impl Register for Query {
    fn register(registry: Registry) -> Registry {
        // define Query object
        let query_object = dynamic::Object::new("Query");

        // define user field
        let user_field =
            dynamic::Field::new("user", dynamic::TypeRef::named_nn(User::NAME), |ctx| {
                dynamic::FieldFuture::new(async move {
                    let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                    // use borrowed_any because Image is not value
                    Ok(Some(FieldValue::borrowed_any(parent.resolve_user())))
                })
            });
        let query_object = query_object.field(user_field);

        // register Query object
        registry.register_object(query_object)
    }
}

impl Query {
    fn resolve_user(&self) -> &User {
        &self.user
    }
}

// impl User

impl Register for User {
    fn register(registry: Registry) -> Registry {
        // define User object
        let object_type = dynamic::Object::new(Self::NAME);

        // define id field
        let id_field = dynamic::Field::new(
            "id",
            dynamic::TypeRef::named_nn(dynamic::TypeRef::STRING),
            |ctx| {
                dynamic::FieldFuture::new(async move {
                    let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                    let value = parent.resolve_id();
                    value.resolve_ref(&ctx)
                })
            },
        );

        let object_type = object_type.field(id_field);

        // define name field

        let name_field = dynamic::Field::new(
            "name",
            dynamic::TypeRef::named_nn(dynamic::TypeRef::STRING),
            |ctx| {
                dynamic::FieldFuture::new(async move {
                    let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                    let value = parent.resolve_name();
                    value.resolve_ref(&ctx)
                })
            },
        );

        let object_type = object_type.field(name_field);

        // define age field

        let age_field = dynamic::Field::new(
            "age",
            dynamic::TypeRef::named_nn(dynamic::TypeRef::STRING),
            |ctx| {
                dynamic::FieldFuture::new(async move {
                    let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                    let value = parent.resolve_age();
                    value.resolve_owned(&ctx)
                })
            },
        );

        let object_type = object_type.field(age_field);

        let object_type = object_type.implement(Node::NAME);
        let object_type = object_type.implement(NamedNode::NAME);
        let object_type = object_type.implement(Aged::NAME);

        // register User object
        registry.register_object(object_type)
    }
}

impl User {
    fn resolve_id(&self) -> &String {
        &self.id
    }

    fn resolve_name(&self) -> &String {
        &self.name
    }

    fn resolve_age(&self) -> &i32 {
        &self.age
    }
}

pub fn create_schema() -> dynamic::Schema {
    let registry = Registry::new()
        .register::<Query>()
        .register::<User>()
        .register::<Node>()
        .register::<NamedNode>()
        .register::<Aged>();

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

        // todo: interface NamedNode should be interface NamedNode implements Node
        assert_eq!(
            normalize_schema(&sdl),
            normalize_schema(
                r#"
                    interface Aged {
                      age: String!
                    }
                    interface NamedNode {
                      name: String!
                      id: String!
                    }
                    interface Node {
                      id: String!
                    }
                    type Query {
                      user: User!
                    }
                    type User implements Node & NamedNode & Aged {
                      id: String!
                      name: String!
                      age: String!
                    }
                    schema {
                      query: Query
                    }
                "#
            ),
        );
    }
}
