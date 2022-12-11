use async_graphql::dynamic;
use async_graphql::dynamic::SchemaBuilder;
use std::collections::{HashMap, VecDeque};

pub trait Register {
    fn register(registry: Registry) -> Registry;
}

pub trait InjectField {
    type Target: Object;
}

pub trait Object {
    const NAME: &'static str;
}

pub struct InjectContext {
    definition: String,
    field: String,
}

struct PendingInjection {
    name: String,
    map: Box<dyn FnOnce(dynamic::Object) -> dynamic::Object>,
    ctx: InjectContext,
}

impl InjectContext {
    pub fn new(definition: &str, field: &str) -> Self {
        Self {
            definition: definition.to_string(),
            field: field.to_string(),
        }
    }
}

pub struct Registry {
    types: HashMap<String, dynamic::Object>,
    pending_injections: VecDeque<PendingInjection>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            types: Default::default(),
            pending_injections: Default::default(),
        }
    }
    pub fn register<T: Register>(self) -> Self {
        T::register(self)
    }
    pub fn register_object(mut self, object: dynamic::Object) -> Self {
        self.types.insert(object.type_name().to_string(), object);
        self
    }

    pub fn update_object<F>(mut self, name: &str, f: F, ctx: InjectContext) -> Self
    where
        F: FnOnce(dynamic::Object) -> dynamic::Object + 'static,
    {
        self.pending_injections.push_back(PendingInjection {
            name: name.to_string(),
            map: Box::new(f),
            ctx,
        });
        self
    }

    fn apply_pending(&mut self) {
        loop {
            if self.pending_injections.is_empty() {
                break;
            }
            let mut changed = false;
            loop {
                let pending = match self.pending_injections.pop_front() {
                    Some(v) => v,
                    None => break,
                };
                let PendingInjection { name, map: f, ctx } = pending;
                if let Some(object) = self.types.remove(&name) {
                    let object = f(object);
                    self.types.insert(name, object);
                    changed = true;
                } else {
                    self.pending_injections
                        .push_back(PendingInjection { name, map: f, ctx });
                }
            }
            if !changed {
                let keys = self
                    .pending_injections
                    .iter()
                    .map(|p| {
                        format!(
                            "Can't find {} when defining {} in {}",
                            p.name, p.ctx.field, p.ctx.definition
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                panic!("Can't find object: {:?}", keys);
            }
        }
    }

    pub fn build_schema(mut self, schema_builder: SchemaBuilder) -> SchemaBuilder {
        self.apply_pending();
        self.types
            .into_iter()
            .fold(schema_builder, |schema_builder, (_, object)| {
                schema_builder.register(object)
            })
    }
}
