use async_graphql::{EmptyMutation, EmptySubscription, Schema, SimpleObject};

#[derive(SimpleObject)]
struct Query {
    user: User,
}

#[derive(SimpleObject)]
struct User {
    id: String,
    name: String,
    avatar: Option<Image>,
}

#[derive(SimpleObject)]
struct Image {
    url: String,
}

fn create_schema(query: Query) -> Schema<Query, EmptyMutation, EmptySubscription> {
    Schema::build(query, EmptyMutation, EmptySubscription).finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema_utils::normalize_schema;

    fn create_user() -> User {
        User {
            id: "1".to_string(),
            name: "John".to_string(),
            avatar: Some(Image {
                url: "https://example.com/avatar.png".to_string(),
            }),
        }
    }

    #[test]
    fn test() {
        let schema = create_schema(Query {
            user: create_user(),
        });
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
        let schema = create_schema(Query {
            user: create_user(),
        });
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
        let res = schema.execute(query).await;
        assert_eq!(
            res.data.into_json().unwrap(),
            serde_json::json!({
                "user": {
                    "id": "1",
                    "name": "John",
                    "avatar": {
                        "url": "https://example.com/avatar.png"
                    }
                }
            })
        );
    }
}
