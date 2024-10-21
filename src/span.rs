use crate::prelude::*;

/// Position information of a token in the input string.
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(crate = "serde_renamed"))]
#[derive(Clone, Debug, PartialEq)]
pub struct Span {
    /// Byte offset of the span in the input string.
    pub offset: usize,
    /// Length of the span.
    pub len: usize,
    /// Line number of the span in the input string.
    pub line: usize,
    /// Column number of the span in the input string.
    pub column: usize,
}

/// Wrapper over any type with span information.
#[cfg(not(feature = "serde"))]
#[derive(Clone, Debug)]
pub struct Spanned<T: Debug + Clone> {
    /// The value of the span.
    pub value: T,
    /// The span information.
    span: Span,
}
/// Wrapper over any type with span information.
#[cfg(feature = "serde")]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(crate = "serde_renamed"))]
#[derive(Clone, Debug)]
pub struct Spanned<T: Serialize + Debug + Clone> {
    /// Span of the value.
    value: T,
    /// Span information.
    span: Span,
}


impl<
    #[cfg(not(feature = "serde"))] T: Debug + Clone,
    #[cfg(feature = "serde")] T: Serialize + Debug + Clone,
> Spanned<T>
{
    /// Creates a new Spanned value.
    pub fn new(value: T, span: Span) -> Self {
        Self { value, span }
    }
    /// Gets the span information.
    pub fn span(&self) -> &Span {
        &self.span
    }

    /// Converts the Spanned value into a tuple of the value and the span.
    pub fn into_components(self) -> (T, Span) {
        (self.value, self.span)
    }
    /// Gets the value of the span.
    pub fn value(&self) -> &T {
        &self.value
    }
    /// Converts the Spanned value into the value.
    pub fn into_value(self) -> T {
        self.value
    }
}

impl<
    #[cfg(not(feature = "serde"))] T: Debug + Clone,
    #[cfg(feature = "serde")] T: Serialize + Debug + Clone,
> Deref for Spanned<T>
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
