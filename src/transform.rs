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

fn test_vectors_result(args: TokenStream, input: TokenStream) -> Result<TokenStream> {
    use crate::fnargs::parse_fn_arg_names;
    use crate::params::MacroParams;
    use syn::spanned::Spanned;

    let span = input.span();
    let params = MacroParams::parse(args)?;
    let implfn: syn::ItemFn = syn::parse2(input)?;
    let _basename = implfn.sig.ident.to_string();
    let argnames = parse_fn_arg_names(&implfn.sig).map_err(|s| syn::Error::new(span, s))?;
    todo!("dir: {:?}, argnames: {:?}", params.dir.display(), argnames);
}
