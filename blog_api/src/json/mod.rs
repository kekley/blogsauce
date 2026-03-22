pub const HEAPLESS_STRING_LEN: usize = 24;

#[derive(Debug, thiserror::Error)]
pub enum JsonFieldError {
    #[error("Missing field: {0}")]
    MissingField(heapless::String<HEAPLESS_STRING_LEN>),
    #[error("Incorrect type on field `{field}`: expected {expected}, got {got}")]
    IncorrectType {
        field: heapless::String<HEAPLESS_STRING_LEN>,
        expected: heapless::String<HEAPLESS_STRING_LEN>,
        got: heapless::String<HEAPLESS_STRING_LEN>,
    },
}

pub trait MaybeJson<'a>: Sized {
    const TYPE_STRING: &'static str;
    fn try_extract(json: &'a json::JsonValue, key: &str) -> Result<Self, JsonFieldError>;
}

macro_rules! impl_maybe_json {
    ($($type:ty, $name:ident, $method:ident),* $(,)?) => {
        $(
        impl<'a> MaybeJson<'a> for $type {
            const TYPE_STRING: &'static str = stringify!($name($type));

            fn try_extract(json: &'a json::JsonValue, key: &str) -> Result<Self, JsonFieldError> {
                let value = &json[key];
                if value.is_null(){
                    Err($crate::json::JsonFieldError::MissingField(key.try_into().unwrap()))
                }else if let Some(value) = value.$method() {
                    Ok(value)
                } else {
                    Err($crate::json::JsonFieldError::IncorrectType {
                        field: key.try_into().expect("Key names should be less than the heapless string size"),
                        expected: Self::TYPE_STRING.try_into().unwrap(),
                        got: get_json_type_as_str(json).try_into().unwrap(),
                    })
                }
            }
        }
        )*
    };
}

impl_maybe_json! {
    i64, Number, as_i64,
    &'a str,String,as_str,
    bool,Boolean,as_bool,
}

fn get_json_type_as_str(value: &json::JsonValue) -> &'static str {
    match value {
        json::JsonValue::Null => "Null",
        json::JsonValue::Short(_) | json::JsonValue::String(_) => "String",
        json::JsonValue::Number(_) => "Number",
        json::JsonValue::Boolean(_) => "Boolean",
        json::JsonValue::Object(_) => "Object",
        json::JsonValue::Array(_) => "Array",
    }
}

pub fn extract_json_field<'a, T>(key: &str, json: &'a json::JsonValue) -> Result<T, JsonFieldError>
where
    T: MaybeJson<'a>,
{
    T::try_extract(json, key)
}
