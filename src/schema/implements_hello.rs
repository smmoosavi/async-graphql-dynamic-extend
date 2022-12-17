use async_graphql::{dynamic::*, Value};

pub fn create_schema() -> Schema {
    let schema = Schema::build("Query", None, None);

    // interface Node { id: ID! }
    let node = Interface::new("Node");
    let node = node.field(InterfaceField::new(
        "id",
        TypeRef::named_nn(TypeRef::STRING),
    ));
    let schema = schema.register(node);

    // interface NamedNode implements Node { name: String! }
    let named_node = Interface::new("NamedNode");
    let named_node = named_node.field(InterfaceField::new(
        "id",
        TypeRef::named_nn(TypeRef::STRING),
    ));
    let named_node = named_node.field(InterfaceField::new(
        "name",
        TypeRef::named_nn(TypeRef::STRING),
    ));
    let named_node = named_node.implement("Node");
    let schema = schema.register(named_node);

    // type User implements NamedNode { id: ID! name: String! }
    let user = Object::new("User");
    let user = user.field(Field::new(
        "id",
        TypeRef::named_nn(TypeRef::STRING),
        |_ctx| FieldFuture::new(async move { Ok(Some(Value::from("u5"))) }),
    ));
    let user = user.field(Field::new(
        "name",
        TypeRef::named_nn(TypeRef::STRING),
        |_ctx| FieldFuture::new(async move { Ok(Some(Value::from("bob"))) }),
    ));
    let user = user.implement("NamedNode");
    let schema = schema.register(user);

    // type Query { user: User }
    let query = Object::new("Query");

    let user_field = Field::new("user", TypeRef::named_nn("User"), |_ctx| todo!());
    let query = query.field(user_field);

    let schema = schema.register(query);

    // finish schema
    schema.finish().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema_utils::normalize_schema;

    #[ignore]
    #[tokio::test]
    async fn test_schema() {
        let schema = create_schema();
        let sdl = schema.sdl();
        println!("{}", sdl);
        assert_eq!(
            normalize_schema(&sdl),
            normalize_schema(
                "
                    interface NamedNode implements Node {
                      id: String!
                      name: String!
                    }
                    interface Node {
                      id: String!
                    }
                    type Query {
                      user: User!
                    }
                    type User implements NamedNode {
                      id: String!
                      name: String!
                    }
                    schema {
                      query: Query
                    }

"
            )
        );
    }
}
