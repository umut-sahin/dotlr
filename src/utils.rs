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


#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(crate = "serde_renamed"))]
#[derive(Clone, Debug)]
pub struct Span {
    pub offset: usize,
    pub len: usize,
    pub column: usize,
    pub line: usize,
}


#[cfg(not(feature = "serde"))]
#[derive(Clone, Debug)]
pub struct Spanned<T: Debug + Clone> {
    pub value: T,
    span: Span,
}
#[cfg(feature = "serde")]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(crate = "serde_renamed"))]
#[derive(Clone, Debug)]
pub struct Spanned<T: Serialize + Debug + Clone> {
    pub value: T,
    span: Span,
}


impl<
    #[cfg(not(feature = "serde"))] T: Debug + Clone,
    #[cfg(feature = "serde")] T: Serialize + Debug + Clone,
> Spanned<T>
{
    pub fn new(value: T, span: Span) -> Self {
        Self { value, span }
    }
    pub fn get_span(&self) -> &Span {
        &self.span
    }

    pub fn into_tuple(self) -> (T, Span) {
        (self.value, self.span)
    }
    pub fn get_span_value(&self) -> &T {
        &self.value
    }
    pub fn into_span_value(self) -> T {
        self.value
    }
}
