use serde::de::IntoDeserializer;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(untagged)]
enum FuzzyNonString<T> {
    NoString(T),
    String(String),
}

pub fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    let opt = Option::<FuzzyNonString<T>>::deserialize(de)?;

    match opt {
        Some(FuzzyNonString::String(s)) => {
            if s.is_empty() {
                Ok(None)
            } else {
                T::deserialize(s.into_deserializer()).map(Some)
            }
        }
        Some(FuzzyNonString::NoString(n)) => Ok(Some(n)),
        None => Ok(None),
    }
}
