use async_graphql::{dynamic::*, Value};
// struct
struct Dog;
struct Cat;
struct Snake;
// enum
#[allow(dead_code)]
enum Animal {
    Dog(Dog),
    Cat(Cat),
    Snake(Snake),
}
struct Query {
    pet: Animal,
}

impl Animal {
    fn to_field_value(&self) -> FieldValue {
        match self {
            Animal::Dog(dog) => FieldValue::borrowed_any(dog).with_type("Dog"),
            Animal::Cat(cat) => FieldValue::borrowed_any(cat).with_type("Cat"),
            Animal::Snake(snake) => FieldValue::borrowed_any(snake).with_type("Snake"),
        }
    }
}

pub fn create_schema() -> Schema {
    // interface
    let named = Interface::new("Named");
    let named = named.field(InterfaceField::new(
        "name",
        TypeRef::named_nn(TypeRef::STRING),
    ));
    // dog
    let dog = Object::new("Dog");
    let dog = dog.field(Field::new(
        "name",
        TypeRef::named_nn(TypeRef::STRING),
        |_ctx| FieldFuture::new(async move { Ok(Some(Value::from("dog"))) }),
    ));
    let dog = dog.field(Field::new(
        "power",
        TypeRef::named_nn(TypeRef::INT),
        |_ctx| FieldFuture::new(async move { Ok(Some(Value::from(100))) }),
    ));
    let dog = dog.implement("Named");
    // cat
    let cat = Object::new("Cat");
    let cat = cat.field(Field::new(
        "name",
        TypeRef::named_nn(TypeRef::STRING),
        |_ctx| FieldFuture::new(async move { Ok(Some(Value::from("cat"))) }),
    ));
    let cat = cat.field(Field::new(
        "life",
        TypeRef::named_nn(TypeRef::INT),
        |_ctx| FieldFuture::new(async move { Ok(Some(Value::from(9))) }),
    ));
    let cat = cat.implement("Named");
    // snake
    let snake = Object::new("Snake");
    let snake = snake.field(Field::new(
        "length",
        TypeRef::named_nn(TypeRef::INT),
        |_ctx| FieldFuture::new(async move { Ok(Some(Value::from(200))) }),
    ));
    // animal
    let animal = Union::new("Animal");
    let animal = animal.possible_type("Dog");
    let animal = animal.possible_type("Cat");
    let animal = animal.possible_type("Snake");
    // query

    let query = Object::new("Query");
    let query = query.field(Field::new("pet", TypeRef::named_nn("Animal"), |ctx| {
        FieldFuture::new(async move {
            let query = ctx.parent_value.try_downcast_ref::<Query>()?;
            Ok(Some(query.pet.to_field_value()))
        })
    }));

    let schema = Schema::build(query.type_name(), None, None);
    let schema = schema
        .register(query)
        .register(named)
        .register(dog)
        .register(cat)
        .register(snake)
        .register(animal);

    schema.finish().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_graphql::Request;

    #[tokio::test]
    async fn test_schema() {
        let schema = create_schema();
        let sdl = schema.sdl();
        assert_eq!(
            sdl,
            "union Animal = Dog | Cat | Snake


type Cat implements Named {
	name: String!
	life: Int!
}

type Dog implements Named {
	name: String!
	power: Int!
}




interface Named {
	name: String!
}

type Query {
	pet: Animal!
}

type Snake {
	length: Int!
}


schema {
	query: Query
}
"
        );
    }
    #[tokio::test]
    #[ignore]
    async fn test_query() {
        let schema = create_schema();
        let query = r#"
            query {
                dog: pet {
                    ... on Dog {
                        __dog_typename: __typename
                        name
                        power
                    }
                }
                named: pet {
                    ... on Named {
                        __named_typename: __typename
                        name
                    }
                }
            }
        "#;
        let root = Query {
            pet: Animal::Dog(Dog),
        };
        let req = Request::new(query).root_value(FieldValue::owned_any(root));
        let res = schema.execute(req).await;

        println!("{}", serde_json::to_string_pretty(&res.data).unwrap());
        // output:
        // {
        //   "dog": {
        //     "__dog_typename": "Dog",
        //     "name": "dog",
        //     "power": 100
        //   },
        //   "named": {}
        // }

        assert_eq!(
            res.data.into_json().unwrap(),
            serde_json::json!({
                "dog": {
                    "__dog_typename": "Dog",
                    "name": "dog",
                    "power": 100
                },
                "named": {
                    "__named_typename": "Dog",
                    "name": "dog"
                }
            })
        );
    }
}
