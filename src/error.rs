#[derive(Debug, derive_more::From)]
pub(crate) enum Error {
    Syn(syn::Error),
    Darling(darling::Error),
}

pub(crate) type Result<T> = std::result::Result<T, Error>;

impl Error {
    pub(crate) fn into_compile_error(self) -> proc_macro2::TokenStream {
        use Error::*;

        match self {
            Syn(e) => e.into_compile_error(),
            Darling(e) => e.write_errors(),
        }
    }
}
