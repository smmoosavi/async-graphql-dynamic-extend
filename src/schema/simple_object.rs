mod static_schema;

use crate::schema::registry::{Object, Register, Registry};
use async_graphql::dynamic;
use async_graphql::dynamic::DynamicRequestExt;
use async_graphql::dynamic::FieldValue;

// user

// mark as root
// mark as object
struct Query {
    user: User,
}

// mark as object
struct User {
    id: String,
    name: String,
    avatar: Option<Image>,
}

// mark as object
struct Image {
    url: String,
}

// generated
struct Root(Query);

//deref
impl std::ops::Deref for Root {
    type Target = Query;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Object for Query {
    const NAME: &'static str = "Query";
}

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

impl Object for User {
    const NAME: &'static str = "User";
}

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
                    Ok(Some(FieldValue::value(parent.resolve_id().to_owned())))
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
                    // use value because it's a String
                    Ok(Some(FieldValue::value(parent.resolve_name().to_owned())))
                })
            },
        );
        let object_type = object_type.field(name_field);

        // define avatar field
        let avatar_field =
            dynamic::Field::new("avatar", dynamic::TypeRef::named(Image::NAME), |ctx| {
                dynamic::FieldFuture::new(async move {
                    let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                    // use map because avatar is optional
                    // use borrowed_any because Image is not value
                    Ok(parent
                        .resolve_avatar()
                        .as_ref()
                        .map(|v| FieldValue::borrowed_any(v)))
                })
            });
        let object_type = object_type.field(avatar_field);

        // register object
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
    fn resolve_avatar(&self) -> &Option<Image> {
        &self.avatar
    }
}

impl Object for Image {
    const NAME: &'static str = "Image";
}

impl Register for Image {
    fn register(registry: Registry) -> Registry {
        // define Image object
        let object_type = dynamic::Object::new(Self::NAME);

        // define url field
        let url_field = dynamic::Field::new(
            "url",
            dynamic::TypeRef::named_nn(dynamic::TypeRef::STRING),
            |ctx| {
                dynamic::FieldFuture::new(async move {
                    let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                    Ok(Some(FieldValue::value(parent.resolve_url().to_owned())))
                })
            },
        );
        let object_type = object_type.field(url_field);

        // register object
        registry.register_object(object_type)
    }
}

impl Image {
    fn resolve_url(&self) -> &String {
        &self.url
    }
}

pub fn create_schema() -> dynamic::Schema {
    let registry = Registry::new()
        .register::<Query>()
        .register::<User>()
        .register::<Image>();

    let schema = dynamic::Schema::build(Query::NAME, None, None);
    registry.build_schema(schema).finish().unwrap()
}

// tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema_utils::normalize_schema;

    #[test]
    fn test() {
        let schema = create_schema();
        let sdl = schema.sdl();
        assert_eq!(
            normalize_schema(&sdl),
            normalize_schema(
                r#"
                type Image {
                  url: String!
                }
                type Query {
                  user: User!
                }
                type User {
                  id: String!
                  name: String!
                  avatar: Image
                }
                schema {
                  query: Query
                }
                "#
            ),
        );
    }

    #[tokio::test]
    async fn test_query() {
        let schema = create_schema();
        let query = r#"
        query {
            user {
                id
                name
                avatar {
                    url
                }
            }
        }
        "#;
        let user = User {
            id: "1".to_string(),
            name: "John".to_string(),
            avatar: Some(Image {
                url: "https://example.com/avatar.png".to_string(),
            }),
        };
        let req =
            async_graphql::Request::new(query).root_value(FieldValue::owned_any(Query { user }));
        let res = schema.execute(req).await;
        let data = res.data.into_json().unwrap();
        assert_eq!(
            data,
            serde_json::json!({
                "user": {
                    "id": "1",
                    "name": "John",
                    "avatar": {
                        "url": "https://example.com/avatar.png",
                    }
                }
            })
        );
    }
}
