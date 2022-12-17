use crate::schema::output_types::utils::{ResolveOwned, ResolveRef};
use crate::schema::registry::Object;
use crate::schema::registry::{Enum, Register, Registry};
use async_graphql::dynamic::{EnumItem, FieldValue};
use async_graphql::{dynamic, Context};
use serde::Deserialize;

// user
#[derive(Deserialize, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

struct Query {}

impl Query {
    async fn resolve_next(&self, _ctx: &Context<'_>, direction: Direction) -> Direction {
        match direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

// generated

impl Enum for Direction {
    const NAME: &'static str = "Direction";
}

impl Register for Direction {
    fn register(registry: Registry) -> Registry {
        // define Direction enum
        let direction_enum = async_graphql::dynamic::Enum::new("Direction");

        let direction_enum = direction_enum.item(EnumItem::new("North"));
        let direction_enum = direction_enum.item(EnumItem::new("East"));
        let direction_enum = direction_enum.item(EnumItem::new("South"));
        let direction_enum = direction_enum.item(EnumItem::new("West"));

        // register Direction enum
        registry.register_enum(direction_enum)
    }
}

impl From<&Direction> for async_graphql::Value {
    fn from(direction: &Direction) -> Self {
        match direction {
            Direction::North => async_graphql::Value::Enum(async_graphql::Name::new("North")),
            Direction::East => async_graphql::Value::Enum(async_graphql::Name::new("East")),
            Direction::South => async_graphql::Value::Enum(async_graphql::Name::new("South")),
            Direction::West => async_graphql::Value::Enum(async_graphql::Name::new("West")),
        }
    }
}

impl<'a> ResolveOwned<'a> for Direction {
    fn resolve_owned(self, _ctx: &Context) -> async_graphql::Result<Option<FieldValue<'a>>> {
        Ok(Some(FieldValue::value(&self)))
    }
}

impl<'a> ResolveRef<'a> for Direction {
    fn resolve_ref(&'a self, _ctx: &Context) -> async_graphql::Result<Option<FieldValue<'a>>> {
        Ok(Some(FieldValue::value(self)))
    }
}

impl Object for Query {
    const NAME: &'static str = "Query";
}

impl Register for Query {
    fn register(registry: Registry) -> Registry {
        // define Query object
        let query_object = async_graphql::dynamic::Object::new("Query");

        // define next field
        let next_field = async_graphql::dynamic::Field::new(
            "next",
            async_graphql::dynamic::TypeRef::named_nn(Direction::NAME),
            |ctx| {
                async_graphql::dynamic::FieldFuture::new(async move {
                    let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                    let arg_0 = ctx.args.try_get("direction")?.deserialize()?;
                    // use borrowed_any because Direction is not value
                    let value = parent.resolve_next(&ctx, arg_0).await;
                    value.resolve_owned(&ctx)
                })
            },
        );

        let next_field = next_field.argument(async_graphql::dynamic::InputValue::new(
            "direction",
            async_graphql::dynamic::TypeRef::named_nn(Direction::NAME),
        ));

        let query_object = query_object.field(next_field);

        // register Query object
        registry.register_object(query_object)
    }
}

fn create_schema() -> async_graphql::dynamic::Schema {
    let registry = Registry::new().register::<Query>().register::<Direction>();
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
                enum Direction {
                    North
                    East
                    South
                    West
                }

                type Query {
                    next(direction: Direction!): Direction!
                }
                schema {
                  query: Query
                }

                "#
            )
        );
    }

    #[tokio::test]
    async fn test_query() {
        let schema = create_schema();
        let query = r#"
            query {
                next(direction: North)
            }
        "#;

        let req = async_graphql::Request::new(query).root_value(FieldValue::owned_any(Query {}));
        let res = schema.execute(req).await;
        let data = res.data.into_json().unwrap();
        assert_eq!(
            data,
            serde_json::json!({
                "next": "East"
            }),
        );
    }
}
