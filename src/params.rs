use crate::error::Result;
use darling::FromMeta;
use proc_macro2::TokenStream;

#[derive(Debug, FromMeta)]
pub(crate) struct MacroParams {
    pub(crate) dir: String,
}

impl MacroParams {
    pub(crate) fn parse(tokens: TokenStream) -> Result<Self> {
        let args = parse_attribute_args(tokens)?;
        let params = Self::from_list(&args)?;
        Ok(params)
    }
}

/// We cannot use `parse_macro_input!` which returns a `TokenStream` and `AttributeArgs` does not
/// impl `syn::parse::Parse`, so we do this by hand. :-<
fn parse_attribute_args(tokens: TokenStream) -> Result<syn::AttributeArgs> {
    use syn::Token;

    enum State {
        Empty,
        Path(syn::Path),
        PathEq(syn::Path, Token![=]),
        ExpectingComma,
    }
    use State::*;

    let mut metas = vec![];
    let mut state = Empty;

    fn parse_tree<T>(tt: proc_macro2::TokenTree) -> syn::Result<T>
    where
        T: syn::parse::Parse,
    {
        syn::parse2(tt.into())
    }

    for tt in tokens {
        match state {
            Empty => {
                let p: syn::Path = parse_tree(tt)?;
                state = Path(p);
            }
            Path(p) => {
                let eq: Token![=] = parse_tree(tt)?;
                state = PathEq(p, eq);
            }
            PathEq(path, eq_token) => {
                let lit: syn::Lit = parse_tree(tt)?;
                metas.push(syn::NestedMeta::Meta(syn::Meta::NameValue(
                    syn::MetaNameValue {
                        path,
                        eq_token,
                        lit,
                    },
                )));
                state = ExpectingComma;
            }
            ExpectingComma => {
                let _: Token![,] = parse_tree(tt)?;
                state = Empty;
            }
        }
    }

    Ok(metas)
}

#[cfg(test)]
mod tests;
