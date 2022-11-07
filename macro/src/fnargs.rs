use quote::ToTokens;
use syn::Type;

pub(crate) fn parse_fn_args(sig: &syn::Signature) -> Result<(Vec<String>, Vec<Type>), String> {
    if let Some(receiver) = sig.receiver() {
        return Err(format!(
            "test functions may not take receiver: {}",
            receiver.into_token_stream(),
        ));
    }

    let zipped = sig
        .inputs
        .iter()
        .map(parse_fn_arg_name)
        .collect::<Result<Vec<(String, Type)>, _>>()?;

    Ok(zipped.into_iter().unzip())
}

fn parse_fn_arg_name(fnarg: &syn::FnArg) -> Result<(String, Type), String> {
    let syn::PatType { pat, ty, .. } = match fnarg {
        syn::FnArg::Typed(pt) => pt,
        _ => unreachable!("receiver check post-condition failure"),
    };

    match &**pat {
        syn::Pat::Ident(syn::PatIdent { ident, .. }) => Ok((ident.to_string(), (**ty).clone())),
        other => Err(format!(
            "expected arg identifier, found: {}",
            other.into_token_stream()
        )),
    }
}
