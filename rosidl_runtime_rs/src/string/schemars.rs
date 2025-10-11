use schemars::{JsonSchema, json_schema};

use super::{String, WString, BoundedString, BoundedWString};

impl JsonSchema for String {
    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        json_schema!({
            "title": "string",
            "description": "a string",
            "type": "string"
        })
    }

    fn schema_name() -> std::borrow::Cow<'static, str> {
        format!("string").into()
    }

    fn schema_id() -> std::borrow::Cow<'static, str> {
        format!("{}::String", module_path!()).into()
    }
}

impl JsonSchema for WString {
    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        json_schema!({
            "title": "wstring",
            "description": "a wide string, nevertheless transmitted as utf-8 in JSON",
            "type": "string"
        })
    }

    fn schema_name() -> std::borrow::Cow<'static, str> {
        "wide string".into()
    }

    fn schema_id() -> std::borrow::Cow<'static, str> {
        format!("{}::WString", module_path!()).into()
    }
}

impl<const N: usize> JsonSchema for BoundedString<N> {
    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        json_schema!({
            "title": format!("{N}-bounded string"),
            "description": format!("a string with a maximum of {N} entries"),
            "type": "string",
            "maxLength": N
        })
    }

    fn schema_name() -> std::borrow::Cow<'static, str> {
        format!("{N}-bounded string").into()
    }

    fn schema_id() -> std::borrow::Cow<'static, str> {
        format!("{}::BoundedString<{N}>", module_path!()).into()
    }
}

impl<const N: usize> JsonSchema for BoundedWString<N> {
    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        json_schema!({
            "title": format!("{N}-bounded wide string"),
            "description": format!("a wide string with a maximum of {N} entries, which is nevertheless transmitted as utf-8 in JSON"),
            "type": "string",
            "maxLength": N
        })
    }

    fn schema_name() -> std::borrow::Cow<'static, str> {
        format!("{N}-bounded wide string").into()
    }

    fn schema_id() -> std::borrow::Cow<'static, str> {
        format!("{}::BoundedWString<{N}>", module_path!()).into()
    }
}
