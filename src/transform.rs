use proc_macro2::TokenStream;

pub(crate) fn test_vectors<TS>(args: TS, input: TS) -> TS
where
    TokenStream: From<TS>,
    TS: From<TokenStream>,
{
    TS::from(
        test_vectors_result(TokenStream::from(args), TokenStream::from(input))
            .unwrap_or_else(syn::Error::into_compile_error),
    )
}

fn test_vectors_result(
    _args: TokenStream,
    _input: TokenStream,
) -> Result<TokenStream, syn::parse::Error> {
    todo!();
}
