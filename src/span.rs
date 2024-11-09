use crate::prelude::*;


/// Position of a token in the input string.
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(crate = "serde_renamed"))]
#[derive(Clone, Debug, PartialEq)]
pub struct Span {
    /// Byte offset of the span in the input string.
    pub offset: usize,
    /// Length of the span in terms of bytes.
    pub length: usize,
    /// Line number of the span in the input string.
    pub line: usize,
    /// Column number of the span in the input string.
    pub column: usize,
}


/// Wrapper for objects with spans.
#[cfg(not(feature = "serde"))]
#[derive(Clone, Debug)]
pub struct Spanned<T: Debug + Clone> {
    /// Spanned object.
    object: T,
    /// Span of the object.
    span: Span,
}

/// Wrapper for objects with spans.
#[cfg(feature = "serde")]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(crate = "serde_renamed"))]
#[derive(Clone, Debug)]
pub struct Spanned<T: Serialize + Debug + Clone> {
    /// Spanned object.
    object: T,
    /// Span of the object.
    span: Span,
}

impl<
    #[cfg(not(feature = "serde"))] T: Debug + Clone,
    #[cfg(feature = "serde")] T: Serialize + Debug + Clone,
> Spanned<T>
{
    /// Creates a new spanned object.
    pub fn new(object: T, span: Span) -> Spanned<T> {
        Spanned { object, span }
    }

    /// Gets the spanned object.
    pub fn object(&self) -> &T {
        &self.object
    }

    /// Gets the span of the object.
    pub fn span(&self) -> &Span {
        &self.span
    }

    /// Splits the spanned object into the object and the span.
    pub fn into_components(self) -> (T, Span) {
        (self.object, self.span)
    }

    /// Extracts the object destroys the span.
    pub fn into_object(self) -> T {
        self.object
    }
}

impl<
    #[cfg(not(feature = "serde"))] T: Debug + Clone,
    #[cfg(feature = "serde")] T: Serialize + Debug + Clone,
> Deref for Spanned<T>
{
    type Target = T;

    fn deref(&self) -> &T {
        &self.object
    }
}
