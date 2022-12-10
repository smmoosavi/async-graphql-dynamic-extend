use async_graphql::dynamic;
use async_graphql::dynamic::SchemaBuilder;
use async_graphql::indexmap::IndexMap;

pub trait Object {
    const NAME: &'static str;
    fn register(registry: Registry) -> Registry;
}

pub struct Registry {
    types: IndexMap<String, dynamic::Object>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            types: IndexMap::new(),
        }
    }
    pub fn register_object(mut self, object: dynamic::Object) -> Self {
        self.types.insert(object.type_name().to_string(), object);
        self
    }

    pub fn build_schema(self, schema_builder: SchemaBuilder) -> SchemaBuilder {
        self.types
            .into_iter()
            .fold(schema_builder, |schema_builder, (_, object)| {
                schema_builder.register(object)
            })
    }
}
