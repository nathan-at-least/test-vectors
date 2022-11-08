use std::cmp::PartialEq;
use std::ops::Deref;
use std::str::Utf8Error;

/// Convenience type for tests with string vectors
///
/// A `Utf8Str<'a>` should behave identically to `&'a str` except it implements
/// `TryFrom<&[u8]>` with UTF-8 decoding.  This provides a convenient short hand for
/// [test_vectors](crate::test_vectors) tests which operate on strings, rather than bytes.
///
/// # Example
///
/// ```
/// use test_vectors::{test_vectors, Utf8Str};
///
/// #[test_vectors(dir = "test-data")]
/// fn utf8str_test_replace_spaces(input: Utf8Str<'static>, expected: Utf8Str<'static>) {
///     let output = input.replace(' ', "_");
///     assert_eq!(expected, output);
/// }
/// ```
#[derive(Debug)]
pub struct Utf8Str<'a>(&'a str);

impl<'a> Deref for Utf8Str<'a> {
    type Target = str;

    fn deref(&self) -> &str {
        self.0
    }
}

impl<'a> TryFrom<&'a [u8]> for Utf8Str<'a> {
    type Error = Utf8Error;

    fn try_from(bytes: &'a [u8]) -> Result<Utf8Str<'a>, Utf8Error> {
        std::str::from_utf8(bytes).map(Utf8Str)
    }
}

impl<'a, Rhs> PartialEq<Rhs> for Utf8Str<'a>
where
    str: PartialEq<Rhs>,
{
    fn eq(&self, other: &Rhs) -> bool {
        self.0.eq(other)
    }
}
