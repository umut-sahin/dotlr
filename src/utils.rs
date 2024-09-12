#[allow(unused_imports)]
use crate::prelude::*;


/// The `colored` crate uses OS specific features to colorize the output, which are not available in
/// the WASM target. This trait provides a mock implementation of the `colored` crate for the WASM target.
#[cfg(target_family = "wasm")]
pub trait MockColored {
    fn green(&self) -> String;
    fn cyan(&self) -> String;
    fn bold(&self) -> String;
}

#[cfg(target_family = "wasm")]
impl<T: AsRef<str>> MockColored for T {
    fn green(&self) -> String {
        self.as_ref().to_owned()
    }
    fn cyan(&self) -> String {
        self.as_ref().to_owned()
    }
    fn bold(&self) -> String {
        self.as_ref().to_owned()
    }
}

/// Serialize a map of regex objects to a map of regex strings.
#[cfg(feature = "serde")]
pub fn serialize_regex_map<S>(
    map: &IndexMap<RegexToken, Regex>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut map_serializer = serializer.serialize_map(Some(map.len()))?;
    for (key, value) in map {
        map_serializer.serialize_entry(key, &value.to_string())?;
    }
    map_serializer.end()
}
