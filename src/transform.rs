use crate::error::{Error, Result};
use proc_macro2::TokenStream;
use quote::ToTokens;

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
    use crate::params::MacroParams;
    use syn::spanned::Spanned;

    let span = input.span();
    let params = MacroParams::parse(args)?;
    let implfn: syn::ItemFn = syn::parse2(input)?;
    let _basename = implfn.sig.ident.to_string();
    let argnames = parse_fn_arg_names(&implfn.sig).map_err(|s| syn::Error::new(span, s))?;
    todo!("dir: {:?}, argnames: {:?}", params.dir.display(), argnames);
}

fn parse_fn_arg_names(sig: &syn::Signature) -> std::result::Result<Vec<String>, String> {
    if let Some(receiver) = sig.receiver() {
        return Err(format!(
            "test functions may not take receiver: {}",
            receiver.into_token_stream(),
        ));
    }

    sig.inputs.iter().map(parse_fn_arg_name).collect()
}

fn parse_fn_arg_name(fnarg: &syn::FnArg) -> std::result::Result<String, String> {
    let syn::PatType { pat, .. } = match fnarg {
        syn::FnArg::Typed(pt) => pt,
        _ => unreachable!("receiver check post-condition failure"),
    };

    match &**pat {
        syn::Pat::Ident(syn::PatIdent { ident, .. }) => Ok(ident.to_string()),
        other => Err(format!(
            "expected arg identifier, found: {}",
            other.into_token_stream()
        )),
    }
}
