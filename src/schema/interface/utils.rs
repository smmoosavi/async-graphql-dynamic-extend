use crate::schema::output_types::utils::ResolveOwned;
use async_graphql::dynamic::FieldValue;
use async_graphql::Context;
use std::any::Any;

pub enum AnyBox<'a> {
    Owned(Box<dyn Any + Send + Sync>, String),
    Borrowed(&'a (dyn Any + Send + Sync), String),
}

impl<'a> AnyBox<'a> {
    pub fn new_owned<T: Any + Send + Sync>(value: T, ty: String) -> Self {
        Self::Owned(Box::new(value), ty)
    }
    pub fn new_borrowed<T: Any + Send + Sync + Sized>(value: &'a T, ty: String) -> Self {
        Self::Borrowed(value, ty)
    }
}

impl<'a> ResolveOwned<'a> for AnyBox<'a> {
    fn resolve_owned(self, _ctx: &Context) -> async_graphql::Result<Option<FieldValue<'a>>> {
        match self {
            AnyBox::Owned(obj, name) => Ok(Some(FieldValue::boxed_any(obj).with_type(name))),
            AnyBox::Borrowed(obj, name) => Ok(Some(FieldValue::borrowed_any(obj).with_type(name))),
        }
    }
}
