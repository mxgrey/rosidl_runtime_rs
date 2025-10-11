use schemars::{JsonSchema, json_schema};

use super::{Sequence, BoundedSequence, SequenceAlloc};

impl<T: JsonSchema + SequenceAlloc> JsonSchema for Sequence<T> {
    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        json_schema!({
            "title": "Sequence",
            "description": "A sequence of ROS message data",
            "type": "array",
            "items": {
                "type": T::schema_id()
            }
        })
    }

    fn schema_name() -> std::borrow::Cow<'static, str> {
        format!("Sequence<{}>", T::schema_name()).into()
    }

    fn schema_id() -> std::borrow::Cow<'static, str> {
        format!(
            "{}::Sequence<{}>",
            module_path!(),
            T::schema_id()
        ).into()
    }
}

impl<T: JsonSchema + SequenceAlloc, const N: usize> JsonSchema for BoundedSequence<T, N> {
    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        json_schema!({
            "title": "Bounded Sequence",
            "description": "A bounded sequence of ROS message data",
            "type": "array",
            "items": {
                "type": T::schema_id()
            },
            "maxItems": N
        })
    }

    fn schema_name() -> std::borrow::Cow<'static, str> {
        format!("BoundedSequence<{}, {N}>", T::schema_name()).into()
    }

    fn schema_id() -> std::borrow::Cow<'static, str> {
        format!(
            "{}::BoundedSequence<{}, {N}>",
            module_path!(),
            T::schema_id(),
        ).into()
    }
}
