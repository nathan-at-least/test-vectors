use super::MacroParams;
use quote::quote;

#[test]
fn test_parse_empty_string() {
    let input = quote! {
        dir = ""
    };

    let mp = MacroParams::parse(input).unwrap();

    assert_eq!(mp.dir, "");
}
