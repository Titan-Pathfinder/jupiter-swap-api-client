use {
    serde::{de, Deserializer, Serializer},
    serde::{Deserialize, Serialize},
    std::str::FromStr,
    std::fmt::Display,
};


pub fn serialize<T, S>(t: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    T: ToString,
    S: Serializer,
{
    if let Some(t) = t {
        t.to_string().serialize(serializer)
    } else {
        serializer.serialize_none()
    }
}

pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: FromStr,
    D: Deserializer<'de>,
    // Use Display for better error messages
    <T as FromStr>::Err: Display,
{
    // Attempt to deserialize an Option<String>
    let opt_s: Option<String> = Option::deserialize(deserializer)?;
    match opt_s {
        Some(s) => {
            // Attempt to parse the string into T
            match s.parse::<T>() {
                Ok(value) => Ok(Some(value)),
                Err(e) => Err(de::Error::custom(format!("Parse error: {}", e))),
            }
        }
        None => Ok(None),
    }
}