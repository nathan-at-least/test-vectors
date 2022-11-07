use std::cmp::PartialEq;
use std::ops::Deref;
use std::str::Utf8Error;
use test_vectors::test_vectors;

#[test_vectors(dir = "tests/basic")]
fn replace_spaces_with_underscore(input: &[u8], expected: &[u8]) {
    let instr = std::str::from_utf8(input).unwrap();
    let expstr = std::str::from_utf8(expected).unwrap();
    let output = instr.replace(' ', "_");
    assert_eq!(expstr, &output);
}

#[test_vectors(dir = "tests/basic")]
fn replace_spaces_with_underscore_result(input: &[u8], expected: &[u8]) -> Result<(), Utf8Error> {
    let instr = std::str::from_utf8(input)?;
    let expstr = std::str::from_utf8(expected)?;
    let output = instr.replace(' ', "_");
    assert_eq!(expstr, &output);
    Ok(())
}

#[derive(Debug)]
struct Utf8Str<'a>(&'a str);

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

    fn ne(&self, other: &Rhs) -> bool {
        self.0.ne(other)
    }
}

#[test_vectors(dir = "tests/basic")]
fn utf8str_replace_spaces_with_underscore(input: Utf8Str<'static>, expected: Utf8Str<'static>) {
    let output = input.replace(' ', "_");
    assert_eq!(expected, output);
}
