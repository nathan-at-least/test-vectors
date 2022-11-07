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
    use crate::listdir::list_dir;
    use crate::params::MacroParams;
    use quote::quote;
    use syn::spanned::Spanned;

    let spanargs = args.span();
    let spaninput = input.span();
    let params = MacroParams::parse(args)?;
    let mut implfn: syn::ItemFn = syn::parse2(input)?;

    // Save the impl fn name and rename it:
    let basename = implfn.sig.ident.to_string();
    implfn.sig.ident = syn::Ident::new(&format!("impl_{}", &basename), implfn.sig.ident.span());
    let implname = &implfn.sig.ident;

    // Save the return type to propagate it:
    let tyret = &implfn.sig.output;

    let argnames = parse_fn_arg_names(&implfn.sig).map_err(|s| syn::Error::new(spaninput, s))?;
    let casenames = list_dir(&params.dir).map_err(|e| syn::Error::new(spanargs, e.to_string()))?;

    let mut casefns = vec![];
    for casename in casenames {
        let casefnname = syn::Ident::new(&format!("{}_{}", &basename, &casename), spanargs);
        let argpaths = argnames
            .iter()
            .map(|arg| params.dir.join(&casename).join(arg).display().to_string());

        casefns.push(quote! {
            #[test]
            fn #casefnname() #tyret {
                #implname(
                    #( include_bytes!( #argpaths ) ),*
                )
            }
        });
    }

    Ok(quote! {
        #implfn

        #( #casefns )*
    })
}
