// mod params;
mod transform;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn test_vectors(args: TokenStream, input: TokenStream) -> TokenStream {
    self::transform::test_vectors(args, input)
}
