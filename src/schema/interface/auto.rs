use crate::schema::output_types::utils::ResolveOwned;
use crate::schema::registry::{
    ExpandObject, ExpandObjectContext, Interface, Object, Register, Registry,
};
use async_graphql::dynamic;
use async_graphql::dynamic::FieldValue;

// user
// mark as interface
pub trait Node {
    fn resolve_id(&self) -> String;
}

// mark as interface
pub trait NamedNode: Node {
    fn resolve_name(&self) -> String;
}

// mark as interface
pub trait Aged {
    fn resolve_age(&self) -> i32;
}

pub struct Query {
    user: User,
}

// mark as object
pub struct User {
    // mark as skip
    id: String,
    // mark as skip
    name: String,
    // mark as skip
    age: i32,
}

// impl Node

impl Node for User {
    fn resolve_id(&self) -> String {
        self.id.clone()
    }
}

// impl NamedNode

impl NamedNode for User {
    fn resolve_name(&self) -> String {
        self.name.clone()
    }
}

// impl Aged

impl Aged for User {
    fn resolve_age(&self) -> i32 {
        self.age
    }
}

// generated

impl Object for Query {
    const NAME: &'static str = "Query";
}

impl Object for User {
    const NAME: &'static str = "User";
}

struct NodeInterface;

impl Interface for NodeInterface {
    const NAME: &'static str = "Node";

    fn register_fields(interface: dynamic::Interface) -> dynamic::Interface {
        let id_field = dynamic::InterfaceField::new(
            "id",
            dynamic::TypeRef::named_nn(dynamic::TypeRef::STRING),
        );
        interface.field(id_field)
    }
}

impl Register for NodeInterface {
    fn register(registry: Registry) -> Registry {
        let interface = dynamic::Interface::new(Self::NAME);
        let interface = Self::register_fields(interface);
        registry.register_interface(interface)
    }
}

struct NamedNodeInterface;

impl Interface for NamedNodeInterface {
    const NAME: &'static str = "NamedNode";

    fn register_fields(interface: dynamic::Interface) -> dynamic::Interface {
        let name_field = dynamic::InterfaceField::new(
            "name",
            dynamic::TypeRef::named_nn(dynamic::TypeRef::STRING),
        );
        let interface = interface.field(name_field);

        // register parent interface
        NodeInterface::register_fields(interface)
    }
}

impl Register for NamedNodeInterface {
    fn register(registry: Registry) -> Registry {
        let interface = dynamic::Interface::new(Self::NAME);
        let interface = interface.implement(NodeInterface::NAME);
        let interface = Self::register_fields(interface);
        registry.register_interface(interface)
    }
}

struct AgedInterface;

impl Interface for AgedInterface {
    const NAME: &'static str = "Aged";

    fn register_fields(interface: dynamic::Interface) -> dynamic::Interface {
        let age_field = dynamic::InterfaceField::new(
            "age",
            dynamic::TypeRef::named_nn(dynamic::TypeRef::STRING),
        );
        interface.field(age_field)
    }
}

impl Register for AgedInterface {
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

        // automatically register call these functions:
        // NodeExpandUser::register(registry)
        // NamedNodeExpandUser::register(registry)
        // AgedExpandUser::register(registry)
        let mut user_register: Vec<_> = inventory::iter::<UserRegister>.into_iter().collect();
        user_register.sort_by(|a, b| a.order.cmp(&b.order));
        let registry = user_register
            .into_iter()
            .fold(registry, |registry, register| register.register(registry));

        // register User
        registry.register_object(object_type)
    }
}

struct UserRegister {
    order: i32,
    register: fn(Registry) -> Registry,
}

impl UserRegister {
    const fn new<T: Register + 'static>(order: i32) -> Self {
        Self {
            order,
            register: T::register,
        }
    }
}

impl UserRegister {
    fn register(&self, registry: Registry) -> Registry {
        (self.register)(registry)
    }
}

inventory::collect!(UserRegister);

// impl Node for User

struct NodeExpandUser;

impl ExpandObject for NodeExpandUser {
    type Target = User;
}

impl Register for NodeExpandUser {
    fn register(registry: Registry) -> Registry {
        println!("register NodeExpandUser for User");
        // define id field
        let id_field = dynamic::Field::new(
            "id",
            dynamic::TypeRef::named_nn(dynamic::TypeRef::STRING),
            |ctx| {
                dynamic::FieldFuture::new(async move {
                    let parent = ctx
                        .parent_value
                        .try_downcast_ref::<<Self as ExpandObject>::Target>()?;
                    let value = parent.resolve_id();
                    value.resolve_owned(&ctx)
                })
            },
        );

        // register Node object
        registry.update_object(
            <<Self as ExpandObject>::Target as Object>::NAME,
            |query_object| query_object.field(id_field).implement(NodeInterface::NAME),
            ExpandObjectContext::new("NodeExpandUser", "id"),
        )
    }
}

inventory::submit!(UserRegister::new::<NodeExpandUser>(0));

// impl NamedNode for User
struct NamedNodeExpandUser;

impl ExpandObject for NamedNodeExpandUser {
    type Target = User;
}

impl Register for NamedNodeExpandUser {
    fn register(registry: Registry) -> Registry {
        println!("register NamedNode for User");
        // define name field
        let name_field = dynamic::Field::new(
            "name",
            dynamic::TypeRef::named_nn(dynamic::TypeRef::STRING),
            |ctx| {
                dynamic::FieldFuture::new(async move {
                    let parent = ctx
                        .parent_value
                        .try_downcast_ref::<<Self as ExpandObject>::Target>()?;
                    let value = parent.resolve_name();
                    value.resolve_owned(&ctx)
                })
            },
        );

        // register NamedNode object
        registry.update_object(
            <<Self as ExpandObject>::Target as Object>::NAME,
            |query_object| {
                query_object
                    .field(name_field)
                    .implement(NamedNodeInterface::NAME)
            },
            ExpandObjectContext::new("NamedNodeExpandUser", "name"),
        )
    }
}

inventory::submit!(UserRegister::new::<NamedNodeExpandUser>(1));

// impl Aged for User
struct AgedExpandUser;

impl ExpandObject for AgedExpandUser {
    type Target = User;
}

impl Register for AgedExpandUser {
    fn register(registry: Registry) -> Registry {
        println!("register Aged for User");
        // define age field
        let age_field = dynamic::Field::new(
            "age",
            dynamic::TypeRef::named_nn(dynamic::TypeRef::STRING),
            |ctx| {
                dynamic::FieldFuture::new(async move {
                    let parent = ctx
                        .parent_value
                        .try_downcast_ref::<<Self as ExpandObject>::Target>()?;
                    let value = parent.resolve_age();
                    value.resolve_owned(&ctx)
                })
            },
        );

        // register Aged object
        registry.update_object(
            <<Self as ExpandObject>::Target as Object>::NAME,
            |query_object| query_object.field(age_field).implement(AgedInterface::NAME),
            ExpandObjectContext::new("AgedExpandUser", "age"),
        )
    }
}

inventory::submit!(UserRegister::new::<AgedExpandUser>(2));

pub fn create_schema() -> dynamic::Schema {
    let registry = Registry::new()
        .register::<Query>()
        .register::<User>()
        .register::<NodeInterface>()
        .register::<NamedNodeInterface>()
        .register::<AgedInterface>();

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
