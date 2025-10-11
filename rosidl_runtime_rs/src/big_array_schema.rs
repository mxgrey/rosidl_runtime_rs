use schemars::{JsonSchema, json_schema};

/// This is a dummy struct that helps us to generate schemas for large arrays.
/// This is part of a workaround for https://github.com/GREsau/schemars/issues/89
pub struct BigArraySchema<T, const N: usize> {
    _data: [T; N],
}

impl<T:JsonSchema, const N: usize> JsonSchema for BigArraySchema<T, N> {
    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        json_schema!({
            "title": "big fixed-size array",
            "description": format!("an array of {} with {} elements", T::schema_name(), N),
            "type": "array",
            "items": {
                "type": T::schema_id()
            },
            "minItems": N,
            "maxItems": N
        })
    }

    fn schema_name() -> std::borrow::Cow<'static, str> {
        format!("BigArray<{}, {N}", T::schema_name()).into()
    }

    fn schema_id() -> std::borrow::Cow<'static, str> {
        format!(
            "{}::BigArray<{}, {N}>",
            module_path!(),
            T::schema_id(),
        ).into()
    }
}
