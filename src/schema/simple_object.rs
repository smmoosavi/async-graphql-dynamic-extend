use crate::schema::registry::{Object, Registry};
use async_graphql::dynamic;
use async_graphql::dynamic::FieldValue;

// user
struct Query {
    user: User,
}

struct User {
    id: String,
    name: String,
    avatar: Option<Image>,
}

struct Image {
    url: String,
}

// generated

impl Object for Query {
    const NAME: &'static str = "Query";
    fn register(registry: Registry) -> Registry {
        // define Query object
        let query_object = dynamic::Object::new("Query");

        // define user field
        let user_field =
            dynamic::Field::new("user", dynamic::TypeRef::named_nn(User::NAME), |ctx| {
                dynamic::FieldFuture::new(async move {
                    let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
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
                    Ok(Some(FieldValue::borrowed_any(parent.resolve_id())))
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
                    Ok(Some(FieldValue::borrowed_any(parent.resolve_name())))
                })
            },
        );
        let object_type = object_type.field(name_field);

        // define avatar field
        let avatar_field =
            dynamic::Field::new("avatar", dynamic::TypeRef::named(Image::NAME), |ctx| {
                dynamic::FieldFuture::new(async move {
                    let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                    Ok(Some(FieldValue::borrowed_any(parent.resolve_avatar())))
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
                    Ok(Some(FieldValue::borrowed_any(parent.resolve_url())))
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

// tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema_utils::normalize_schema;

    #[test]
    fn test() {
        let registry = Registry::new();
        let schema = dynamic::Schema::build(Query::NAME, None, None);
        let registry = Query::register(registry);
        let registry = User::register(registry);
        let registry = Image::register(registry);
        let schema = registry.build_schema(schema).finish().unwrap();
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
}
