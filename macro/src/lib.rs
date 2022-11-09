pub(crate) mod error;
pub(crate) mod fnargs;
pub(crate) mod listdir;
pub(crate) mod params;
mod transform;

use proc_macro::TokenStream;

/// Generate multiple test cases from the annotated criterion test function based on input files
///
/// Usage: `test_vectors(dir = "<path to corpus directory>")`
///
/// See the `test-vectors` crate documentation for full documentation.
#[proc_macro_attribute]
pub fn test_vectors(args: TokenStream, input: TokenStream) -> TokenStream {
    self::transform::test_vectors(args, input)
}
