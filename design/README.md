# Define simple object

```rust
#[Object]
struct User {
    id: String,
    name: String,
}
```

generated code:

```rust
impl Object for User {
    const NAME: String = "User";

    fn register(registry: Registry) -> Result<()> {
        // define User type
        let object_type = Object::new(Self::NAME);
        registry.add(&Self::NAME, object_type)?;
        // define id field
        let field = Field::new("id", TypeRef::named_nn(TypeRef::STRING), |ctx| {
            FieldFuture::new(async move {
                let parent = ctx.parent_value.downcast_ref::<Self>()?;
                parent.resolve_id()
            })
        });
        let object_type = object_type.field(field); 
        // define name field
    }
}

impl User {
    #[inline]
    async fn resolve_id(&self) -> &String {
        self.id
    }
    #[inline]
    async fn resolve_name(&self) -> &String {
        self.id
    }
}
```