// user
use crate::schema::registry::{ExpandObject, ExpandObjectContext, Object, Register, Registry};
use async_graphql::dynamic::DynamicRequestExt;
use async_graphql::dynamic::FieldValue;
use async_graphql::{dynamic, Context};

// mark as root
// mark as object
struct Query;

// mark as object
struct User {
    id: String,
    name: String,
    avatar: Option<Image>,
}

struct MeQuery;

impl ExpandObject for MeQuery {
    type Target = Query;
}

impl MeQuery {
    // mark Query as Target
    async fn resolve_me(_parent: &Query, _ctx: &Context<'_>) -> Option<User> {
        Some(User {
            id: "1".to_string(),
            name: "John".to_string(),
            avatar: Some(Image {
                url: "https://example.com/avatar.png".to_string(),
            }),
        })
    }
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

        // register Query object
        registry.register_object(query_object)
    }
}

impl Register for MeQuery {
    fn register(registry: Registry) -> Registry {
        // define me field
        let me_field = dynamic::Field::new("me", dynamic::TypeRef::named(User::NAME), |ctx| {
            dynamic::FieldFuture::new(async move {
                // todo: feature request for execute with root
                // special case because Query is marked as root
                let parent = ctx
                    .parent_value
                    .try_downcast_ref::<<Self as ExpandObject>::Target>()?;

                Ok(Self::resolve_me(parent, &ctx)
                    .await
                    .map(FieldValue::owned_any))
            })
        });
        registry.update_object(
            <<Self as ExpandObject>::Target as Object>::NAME,
            |query_object| query_object.field(me_field),
            ExpandObjectContext::new("MeQuery", "me"),
        )
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
        .register::<MeQuery>()
        .register::<Image>();
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
                type Image {
                  url: String!
                }
                type Query {
                  me: User
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
                me {
                    id
                    name
                    avatar {
                        url
                    }
                }
            }
        "#;
        let req = async_graphql::Request::new(query).root_value(FieldValue::owned_any(Query));
        let res = schema.execute(req).await;
        let data = res.data.into_json().unwrap();
        assert_eq!(
            data,
            serde_json::json!({
                "me": {
                    "id": "1",
                    "name": "John",
                    "avatar": {
                        "url": "https://example.com/avatar.png",
                    },
                },
            })
        );
    }
}
