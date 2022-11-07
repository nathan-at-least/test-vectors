use super::MacroParams;
use quote::quote;

#[test]
fn test_parse_foo() {
    let input = quote! {
        dir = "foo"
    };

    let mp = MacroParams::parse(input).unwrap();

    assert_eq!(mp.dir.file_name().and_then(|s| s.to_str()), Some("foo"));
}
