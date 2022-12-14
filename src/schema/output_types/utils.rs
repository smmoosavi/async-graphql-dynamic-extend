use async_graphql::dynamic::FieldValue;
use async_graphql::Context;

pub trait OutputValue<'a> {
    fn resolve(self, ctx: &Context) -> async_graphql::Result<Option<FieldValue<'a>>>;
    fn resolve_ref(&'a self, ctx: &Context) -> async_graphql::Result<Option<FieldValue<'a>>>;
}

impl<'a, T: OutputValue<'a> + Sync> OutputValue<'a> for Option<T> {
    #[inline]
    fn resolve(self, ctx: &Context) -> async_graphql::Result<Option<FieldValue<'a>>> {
        match self {
            None => Ok(None),
            Some(value) => value.resolve(ctx),
        }
    }

    fn resolve_ref(&'a self, ctx: &Context) -> async_graphql::Result<Option<FieldValue<'a>>> {
        match self {
            None => Ok(None),
            Some(value) => value.resolve_ref(ctx),
        }
    }
}

impl<'a, T, E> OutputValue<'a> for Result<T, E>
where
    T: OutputValue<'a> + Sync,
    E: Into<async_graphql::Error> + Send + Sync + Clone,
{
    #[inline]
    fn resolve(self, ctx: &Context) -> async_graphql::Result<Option<FieldValue<'a>>> {
        match self {
            Ok(value) => value.resolve(ctx),
            Err(err) => Err(err.into()),
        }
    }

    #[inline]
    fn resolve_ref(&'a self, ctx: &Context) -> async_graphql::Result<Option<FieldValue<'a>>> {
        match self {
            Ok(value) => value.resolve_ref(ctx),
            Err(err) => Err(err.clone().into()),
        }
    }
}

impl<'a, T: OutputValue<'a>> OutputValue<'a> for Vec<T> {
    fn resolve(self, ctx: &Context) -> async_graphql::Result<Option<FieldValue<'a>>> {
        let iter = self.into_iter();
        let items = iter.enumerate().map(|(index, item)| {
            let ctx_idx = ctx.with_index(index);
            match item.resolve(&ctx_idx) {
                Ok(Some(value)) => value,
                Ok(None) => FieldValue::NULL,
                Err(err) => {
                    let server_error = err.into_server_error(ctx_idx.item.pos);
                    ctx_idx.add_error(server_error);
                    FieldValue::NULL
                }
            }
        });
        Ok(Some(FieldValue::list(items)))
    }
    fn resolve_ref(&'a self, ctx: &Context) -> async_graphql::Result<Option<FieldValue<'a>>> {
        let iter = self.iter();
        let items = iter.enumerate().map(|(index, item)| {
            let ctx_idx = ctx.with_index(index);
            match item.resolve_ref(&ctx_idx) {
                Ok(Some(value)) => value,
                Ok(None) => FieldValue::NULL,
                Err(err) => {
                    let server_error = err.into_server_error(ctx_idx.item.pos);
                    ctx_idx.add_error(server_error);
                    FieldValue::NULL
                }
            }
        });
        Ok(Some(FieldValue::list(items)))
    }
}

impl<'a> OutputValue<'a> for async_graphql::ID {
    #[inline]
    fn resolve(self, _ctx: &Context) -> async_graphql::Result<Option<FieldValue<'a>>> {
        Ok(Some(FieldValue::value(self.0)))
    }
    #[inline]
    fn resolve_ref(&'a self, _ctx: &Context) -> async_graphql::Result<Option<FieldValue<'a>>> {
        Ok(Some(FieldValue::value(self.0.to_owned())))
    }
}

impl<'a> OutputValue<'a> for &str {
    #[inline]
    fn resolve(self, _ctx: &Context) -> async_graphql::Result<Option<FieldValue<'a>>> {
        Ok(Some(FieldValue::value(self.to_string())))
    }
    #[inline]
    fn resolve_ref(&'a self, _ctx: &Context) -> async_graphql::Result<Option<FieldValue<'a>>> {
        Ok(Some(FieldValue::value(self.to_owned())))
    }
}

macro_rules! output_value {
    ($($ty:ident),*) => {
        $(
            impl <'a> OutputValue<'a> for $ty {
                #[inline]
                fn resolve(self, _ctx: &Context) -> async_graphql::Result<Option<FieldValue<'a>>> {
                    Ok(Some(FieldValue::value(self)))
                }
                fn resolve_ref(&'a self, _ctx: &Context) -> async_graphql::Result<Option<FieldValue<'a>>> {
                    Ok(Some(FieldValue::value(self.to_owned())))
                }
            }
        )*
    };
}

output_value!(String, i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, bool, f32, f64);
