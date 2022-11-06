use crate::error::{Error, Result};
use proc_macro2::TokenStream;

pub(crate) fn test_vectors<TS>(args: TS, input: TS) -> TS
where
    TokenStream: From<TS>,
    TS: From<TokenStream>,
{
    TS::from(
        test_vectors_result(TokenStream::from(args), TokenStream::from(input))
            .unwrap_or_else(Error::into_compile_error),
    )
}

fn test_vectors_result(args: TokenStream, _input: TokenStream) -> Result<TokenStream> {
    use crate::params::MacroParams;

    let params = MacroParams::parse(args)?;

    todo!("{:?}", params.dir);
}
