use async_graphql::{Context, Value};
use async_graphql::dynamic::*;
use async_graphql::parser::types::Selection;

struct Baz;

struct Bar;

struct Foo;

impl Foo {
    fn bar(ctx: &Context) -> Bar {
        // println!("bar {}", ctx.item.node.alias.as_ref().map(|ref a| a.node.as_str()).unwrap_or("no alias"));
        // print parent
        // println!("parent {}", ctx.path_node.as_ref().map(|ref a| a.to_string()).unwrap_or("no parent".to_string()));
        let field = ctx.field();
        let selection_set = field.selection_set();
        for item in selection_set {
            println!("item {}, {:?}, {:?}",item.name(), item.alias(), item)
            // match &item {
            //     Selection::Field(field) => {
            //         println!("field {}, {:?}", field.node.name, field.node.alias)
            //     }
            //     Selection::FragmentSpread(f) => {
            //         let name = f.node.fragment_name.node.as_str()
            //
            //     }
            //     Selection::InlineFragment(_) => {
            //         println!("inline fragment")
            //     }
            // }
        }
        Bar
    }
}

pub fn create_schema() -> Schema {
    let baz = Object::new("Baz");
    let baz = baz.field(Field::new(
        "a",
        TypeRef::named_nn(TypeRef::STRING),
        |_ctx| FieldFuture::new(async move { Ok(Some(Value::from("baz"))) }),
    ));
    let baz = baz.field(Field::new(
        "b",
        TypeRef::named_nn(TypeRef::STRING),
        |_ctx| FieldFuture::new(async move { Ok(Some(Value::from("baz"))) }),
    ));


    let bar = Object::new("Bar");
    let bar = bar.field(Field::new(
        "baz",
        TypeRef::named_nn("Baz"),
        |_ctx| FieldFuture::new(async move { Ok(Some(FieldValue::owned_any(Baz))) }),
    ));

    let foo = Object::new("Foo");
    let foo = foo.field(Field::new(
        "bar",
        TypeRef::named_nn("Bar"),
        |ctx| FieldFuture::new(async move {
            Ok(Some(
                FieldValue::owned_any(Foo::bar(&ctx))
            ))
        }),
    ));
    let query = Object::new("Query");
    let query = query.field(Field::new(
        "foo",
        TypeRef::named_nn("Foo"),
        |_ctx| FieldFuture::new(async move {
            Ok(Some(
                FieldValue::owned_any(Foo)
            ))
        }),
    ));
    let schema = Schema::build("Query", None, None);
    let schema = schema.register(baz);
    let schema = schema.register(bar);
    let schema = schema.register(foo);
    let schema = schema.register(query);
    schema.finish().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema_utils::normalize_schema;

    #[tokio::test]
    async fn test_schema() {
        let schema = create_schema();
        let sdl = schema.sdl();
        assert_eq!(
            normalize_schema(&sdl),
            normalize_schema(
                "
                type Bar {
                  baz: Baz!
                }
                type Baz {
                  a: String!
                  b: String!
                }
                type Foo {
                  bar: Bar!
                }
                type Query {
                  foo: Foo!
                }
                schema {
                  query: Query
                }
"
            )
        );
    }

    #[tokio::test]
    async fn test_query() {
        let schema = create_schema();
        let query = r#"
        query {
          foo {
            bar {
              baz {
                a
              }
              ...bar
              ... on Bar {
                on_baz: baz {
                  b
                }
              }

            }
          }
        }
        fragment bar on Bar {
          f_baz: baz {
            f_a: a
            f_b: b
          }
        }
        "#;
        let res = schema.execute(query).await;
        println!("errors: {:?}", res.errors);
        println!("{}", serde_json::to_string_pretty(&res.data).unwrap());
    }
}