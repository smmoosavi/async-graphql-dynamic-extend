use async_graphql::dynamic;
use async_graphql::dynamic::SchemaBuilder;
use std::collections::{HashMap, VecDeque};

pub trait Register {
    fn register(registry: Registry) -> Registry;
}

/// add field to object
pub trait ExpandObject {
    type Target: Object;
}

/// define new object marked as extend (not supported yet)
pub trait ExtendObject {
    type Target: Object;
}

pub trait Enum {
    const NAME: &'static str;
}

pub trait Object {
    const NAME: &'static str;
}

pub trait Union {
    const NAME: &'static str;
}

pub trait Interface {
    const NAME: &'static str;

    fn register_fields(interface: dynamic::Interface) -> dynamic::Interface;
}

pub trait InputObject {
    const NAME: &'static str;
}

pub struct ExpandObjectContext {
    definition: String,
    field: String,
}

struct PendingExpandObject {
    name: String,
    map: Box<dyn FnOnce(dynamic::Object) -> dynamic::Object>,
    ctx: ExpandObjectContext,
}

impl ExpandObjectContext {
    pub fn new(definition: &str, field: &str) -> Self {
        Self {
            definition: definition.to_string(),
            field: field.to_string(),
        }
    }
}

pub struct Registry {
    types: HashMap<String, dynamic::Object>,
    extend_types: Vec<dynamic::Object>,
    enums: HashMap<String, dynamic::Enum>,
    unions: HashMap<String, dynamic::Union>,
    interfaces: HashMap<String, dynamic::Interface>,
    input_types: HashMap<String, dynamic::InputObject>,
    pending_expand_objects: VecDeque<PendingExpandObject>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            types: Default::default(),
            extend_types: Default::default(),
            enums: Default::default(),
            unions: Default::default(),
            interfaces: Default::default(),
            input_types: Default::default(),
            pending_expand_objects: Default::default(),
        }
    }
    pub fn register<T: Register>(self) -> Self {
        T::register(self)
    }
    pub fn register_object(mut self, object: dynamic::Object) -> Self {
        self.types.insert(object.type_name().to_string(), object);
        self
    }
    pub fn register_extend_object(mut self, object: dynamic::Object) -> Self {
        self.extend_types.push(object);
        self
    }

    pub fn register_enum(mut self, enum_: dynamic::Enum) -> Self {
        self.enums.insert(enum_.type_name().to_string(), enum_);
        self
    }

    pub fn register_interface(mut self, interface: dynamic::Interface) -> Self {
        self.interfaces
            .insert(interface.type_name().to_string(), interface);
        self
    }

    pub fn register_union(mut self, union: dynamic::Union) -> Self {
        self.unions.insert(union.type_name().to_string(), union);
        self
    }

    pub fn register_input_object(mut self, object: dynamic::InputObject) -> Self {
        self.input_types
            .insert(object.type_name().to_string(), object);
        self
    }

    pub fn update_object<F>(mut self, name: &str, f: F, ctx: ExpandObjectContext) -> Self
    where
        F: FnOnce(dynamic::Object) -> dynamic::Object + 'static,
    {
        self.pending_expand_objects.push_back(PendingExpandObject {
            name: name.to_string(),
            map: Box::new(f),
            ctx,
        });
        self
    }

    fn apply_pending(&mut self) {
        loop {
            if self.pending_expand_objects.is_empty() {
                break;
            }
            let mut changed = false;
            loop {
                let pending = match self.pending_expand_objects.pop_front() {
                    Some(v) => v,
                    None => break,
                };
                let PendingExpandObject { name, map: f, ctx } = pending;
                if let Some(object) = self.types.remove(&name) {
                    let object = f(object);
                    self.types.insert(name, object);
                    changed = true;
                } else {
                    self.pending_expand_objects.push_back(PendingExpandObject {
                        name,
                        map: f,
                        ctx,
                    });
                }
            }
            if !changed {
                let keys = self
                    .pending_expand_objects
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
        let schema_builder = self
            .enums
            .into_iter()
            .fold(schema_builder, |schema_builder, (_, enum_)| {
                schema_builder.register(enum_)
            });
        let schema_builder = self
            .unions
            .into_iter()
            .fold(schema_builder, |schema_builder, (_, union)| {
                schema_builder.register(union)
            });
        let schema_builder = self
            .interfaces
            .into_iter()
            .fold(schema_builder, |schema_builder, (_, enum_)| {
                schema_builder.register(enum_)
            });
        let schema_builder = self
            .input_types
            .into_iter()
            .fold(schema_builder, |schema_builder, (_, object)| {
                schema_builder.register(object)
            });
        let schema_builder = self
            .types
            .into_iter()
            .fold(schema_builder, |schema_builder, (_, object)| {
                schema_builder.register(object)
            });
        self.extend_types
            .into_iter()
            .fold(schema_builder, |schema_builder, object| {
                schema_builder.register(object)
            })
    }
}
