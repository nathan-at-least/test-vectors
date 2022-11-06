use crate::error::Result;
use darling::FromMeta;
use proc_macro2::TokenStream;

#[derive(Debug, FromMeta)]
pub(crate) struct MacroParams {
    pub(crate) dir: String,
}

impl MacroParams {
    pub(crate) fn parse(tokens: TokenStream) -> Result<Self> {
        let meta: syn::Meta = syn::parse2(tokens)?;
        let params = Self::from_meta(&meta)?;
        Ok(params)
    }
}
