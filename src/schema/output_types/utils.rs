use async_graphql::dynamic::FieldValue;
use async_graphql::Context;

pub trait OutputValue {
    fn resolve(&self, ctx: &Context) -> async_graphql::Result<Option<FieldValue>>;
}

impl<T: OutputValue + ?Sized> OutputValue for &T {
    fn resolve(&self, ctx: &Context) -> async_graphql::Result<Option<FieldValue>> {
        T::resolve(*self, ctx)
    }
}

impl<T: OutputValue + Sync> OutputValue for Option<T> {
    #[inline]
    fn resolve(&self, ctx: &Context) -> async_graphql::Result<Option<FieldValue>> {
        match self {
            None => Ok(None),
            Some(value) => value.resolve(ctx),
        }
    }
}

impl<T, E> OutputValue for Result<T, E>
where
    T: OutputValue + Sync,
    E: Into<async_graphql::Error> + Send + Sync + Clone,
{
    #[inline]
    fn resolve(&self, ctx: &Context) -> async_graphql::Result<Option<FieldValue>> {
        match self {
            Ok(value) => value.resolve(ctx),
            Err(err) => Err(err.clone().into()),
        }
    }
}

impl<'a, T: OutputValue + 'a> OutputValue for &'a [T] {
    fn resolve(&self, ctx: &Context) -> async_graphql::Result<Option<FieldValue>> {
        let iter = self.iter();
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
}

impl<T: OutputValue> OutputValue for Vec<T> {
    fn resolve(&self, ctx: &Context) -> async_graphql::Result<Option<FieldValue>> {
        let iter = self.iter();
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
}

impl OutputValue for async_graphql::ID {
    #[inline]
    fn resolve(&self, _ctx: &Context) -> async_graphql::Result<Option<FieldValue>> {
        Ok(Some(FieldValue::value(self.0.to_owned())))
    }
}

macro_rules! output_value {
    ($($ty:ident),*) => {
        $(
            impl OutputValue for $ty {
                #[inline]
                fn resolve(&self, _ctx: &Context) -> async_graphql::Result<Option<FieldValue>> {
                    Ok(Some(FieldValue::value(self.to_owned())))
                }
            }
        )*
    };
}

output_value!(String, str, i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, bool, f32, f64);
