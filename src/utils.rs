#[allow(unused_imports)]
use crate::prelude::*;


/// We're using `colored` crate, which use OS specific features to colorize the output.
/// Since those features are not available in WebAssembly, we create a mock implementation
/// for the `colored` methods we use.
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


/// Serializes a map of regex tokens to compiled regex objects.
#[cfg(feature = "serde")]
pub fn serialize_regex_token_to_regex_map<S>(
    regex_token_to_regex: &IndexMap<RegexToken, Regex>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut map_serializer = serializer.serialize_map(Some(regex_token_to_regex.len()))?;
    for (key, value) in regex_token_to_regex {
        map_serializer.serialize_entry(key, &value.to_string())?;
    }
    map_serializer.end()
}


/// Counts the number of new lines in a slice and computes the offset after the last new line.
pub fn count_new_lines(slice: &str) -> (usize, Option<usize>) {
    let mut offset_after_newline = None;

    let mut count = 0;
    for (offset, byte) in slice.bytes().enumerate() {
        if byte == b'\n' {
            offset_after_newline = Some(offset + 1);
            count += 1;
        }
    }

    (count, offset_after_newline)
}
