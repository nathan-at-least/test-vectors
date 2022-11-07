use quote::ToTokens;

pub(crate) fn parse_fn_arg_names(sig: &syn::Signature) -> Result<Vec<String>, String> {
    if let Some(receiver) = sig.receiver() {
        return Err(format!(
            "test functions may not take receiver: {}",
            receiver.into_token_stream(),
        ));
    }

    sig.inputs.iter().map(parse_fn_arg_name).collect()
}

fn parse_fn_arg_name(fnarg: &syn::FnArg) -> Result<String, String> {
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
