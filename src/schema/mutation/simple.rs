use crate::schema::output_types::utils::ResolveOwned;
use crate::schema::registry::{
    ExpandObject, ExpandObjectContext, Mutation, Object, Register, Registry,
};
use async_graphql::dynamic::TypeRef;
use async_graphql::{dynamic, Context};
use std::sync::{Arc, RwLock};

#[derive(Clone)]
struct DB {
    count: Arc<RwLock<i32>>,
}

impl DB {
    fn new() -> Self {
        Self {
            count: Arc::new(RwLock::new(0)),
        }
    }
    fn increment(&self) {
        let mut count = self.count.write().unwrap();
        *count += 1;
    }
    fn decrement(&self) {
        let mut count = self.count.write().unwrap();
        *count -= 1;
    }
    fn get_count(&self) -> i32 {
        *self.count.read().unwrap()
    }
}

// marked as root, renamed as Query in schema
#[derive(Clone)]
pub struct Root {
    db: DB,
}

impl Root {
    pub fn new() -> Self {
        Self { db: DB::new() }
    }
}

impl Default for Root {
    fn default() -> Self {
        Self::new()
    }
}

type Query = Root;

impl Object for Query {
    const NAME: &'static str = "Query";
}

impl Register for Query {
    fn register(registry: Registry) -> Registry {
        // define Query object
        let query_object = dynamic::Object::new(<Self as Object>::NAME);

        // register Query object
        registry.register_object(query_object)
    }
}

struct CountQuery;

impl CountQuery {
    async fn resolve_count(parent: &Query, _ctx: &Context<'_>) -> i32 {
        parent.db.get_count()
    }
}

impl ExpandObject for CountQuery {
    type Target = Query;
}

impl Register for CountQuery {
    fn register(registry: Registry) -> Registry {
        // define count field
        let count_field =
            dynamic::Field::new("count", dynamic::TypeRef::named_nn(TypeRef::INT), |ctx| {
                dynamic::FieldFuture::new(async move {
                    let parent = ctx.parent_value.try_downcast_ref::<Query>()?;
                    let value = Self::resolve_count(parent, &ctx).await;
                    ResolveOwned::resolve_owned(value, &ctx)
                })
            });
        registry.update_object(
            <<Self as ExpandObject>::Target as Object>::NAME,
            |query_object| query_object.field(count_field),
            ExpandObjectContext::new("CountQuery", "count"),
        )
    }
}

// mutation

// marked as mutation(root=Root)
struct MutationRoot;

impl Object for MutationRoot {
    const NAME: &'static str = "Mutation";
}

impl Mutation for MutationRoot {
    type Root = Root;
}

impl Register for MutationRoot {
    fn register(registry: Registry) -> Registry {
        // define Mutation object
        let mutation_object = dynamic::Object::new(<Self as Object>::NAME);

        // register Mutation object
        registry.register_object(mutation_object)
    }
}

struct CountMutations;

impl ExpandObject for CountMutations {
    type Target = MutationRoot;
}

impl CountMutations {
    async fn resolve_increment(parent: &Root, _ctx: &Context<'_>) -> i32 {
        parent.db.increment();
        parent.db.get_count()
    }
    async fn resolve_decrement(parent: &Root, _ctx: &Context<'_>) -> i32 {
        parent.db.decrement();
        parent.db.get_count()
    }
}

impl Register for CountMutations {
    fn register(registry: Registry) -> Registry {
        // define increment field
        let increment_field = dynamic::Field::new(
            "increment",
            dynamic::TypeRef::named_nn(TypeRef::INT),
            |ctx| {
                dynamic::FieldFuture::new(async move {
                    let parent = ctx
                        .parent_value
                        .try_downcast_ref::<<<Self as ExpandObject>::Target as Mutation>::Root>(
                    )?;
                    let value = Self::resolve_increment(parent, &ctx).await;
                    ResolveOwned::resolve_owned(value, &ctx)
                })
            },
        );
        // define decrement field
        let decrement_field = dynamic::Field::new(
            "decrement",
            dynamic::TypeRef::named_nn(TypeRef::INT),
            |ctx| {
                dynamic::FieldFuture::new(async move {
                    let parent = ctx.parent_value.try_downcast_ref::<Root>()?;
                    let value = Self::resolve_decrement(parent, &ctx).await;
                    ResolveOwned::resolve_owned(value, &ctx)
                })
            },
        );
        registry.update_object(
            <<Self as ExpandObject>::Target as Object>::NAME,
            |mutation_object| {
                mutation_object
                    .field(increment_field)
                    .field(decrement_field)
            },
            ExpandObjectContext::new("CountMutations", "count"),
        )
    }
}

pub fn create_schema() -> dynamic::Schema {
    let registry = Registry::new()
        .register::<Query>()
        .register::<CountQuery>()
        .register::<MutationRoot>()
        .register::<CountMutations>();
    let schema = dynamic::Schema::build(Query::NAME, Some(MutationRoot::NAME), None);
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
                type Mutation {
                  increment: Int!
                  decrement: Int!
                }
                type Query {
                  count: Int!
                }
                schema {
                  query: Query
                  mutation: Mutation
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
                count
            }
        "#;
        let root = Query::new();
        let req =
            async_graphql::Request::new(query).root_value(dynamic::FieldValue::owned_any(root));
        let res = schema.execute(req).await;
        let data = res.data.into_json().unwrap();
        assert_eq!(
            data,
            serde_json::json!({
                "count": 0
            })
        );
    }

    #[tokio::test]
    async fn test_mutation() {
        let schema = create_schema();
        let query = r#" query { count } "#;
        let mutation = r#" mutation { increment } "#;
        let root = Query::new();
        let query_req = async_graphql::Request::new(query)
            .root_value(dynamic::FieldValue::owned_any(root.clone()));
        let res = schema.execute(query_req).await;
        let data = res.data.into_json().unwrap();
        assert_eq!(data, serde_json::json!({ "count": 0 }));
        let mutation_req = async_graphql::Request::new(mutation)
            .root_value(dynamic::FieldValue::owned_any(root.clone()));
        let res = schema.execute(mutation_req).await;
        let data = res.data.into_json().unwrap();
        assert_eq!(data, serde_json::json!({ "increment": 1 }));
        // query again
        let query_req =
            async_graphql::Request::new(query).root_value(dynamic::FieldValue::owned_any(root));
        let res = schema.execute(query_req).await;
        let data = res.data.into_json().unwrap();
        assert_eq!(data, serde_json::json!({ "count": 1 }));
    }
}
