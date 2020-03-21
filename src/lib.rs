use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use std::fmt::Display;

fn stringify_helper(result: impl ToTokens) -> TokenStream2 {
    quote!(
        macro_rules! result {
            () => { #result }
        }
    )
}
fn stringify_all_helper(open: char, close: &'static str, attr: impl Display, item: impl Display) -> TokenStream2 {
    let result = format!("#[stringify_all{}{}{}] {}", open, attr, close, item);
    stringify_helper(result)
}

/// Creates a macro `result` which expands to the same string literal as
/// `stringify!(attr)`
#[proc_macro_attribute]
pub fn stringify_attr(attr: TokenStream, _item: TokenStream) -> TokenStream {
    stringify_helper(attr.to_string()).into()
}

/// Creates a macro `result` which expands to the same string literal as
/// `stringify!(item)`
#[proc_macro_attribute]
pub fn stringify_item(_attr: TokenStream, item: TokenStream) -> TokenStream {
    stringify_helper(item.to_string()).into()
}

/// An alias for `stringify_parens`
#[proc_macro_attribute]
pub fn stringify_all(attr: TokenStream, item: TokenStream) -> TokenStream {
    stringify_parens(attr, item)
}

/// Creates a macro `result` which expands to the same string literal as
/// `concat!("#[stringify_all(", stringify!(attr), ")]", stringify!(item))`
#[proc_macro_attribute]
pub fn stringify_parens(attr: TokenStream, item: TokenStream) -> TokenStream {
    stringify_all_helper('(', ")", attr, item).into()
}

/// Creates a macro `result` which expands to the same string literal as
/// `concat!("#[stringify_all{", stringify!(attr), "}]", stringify!(item))`
#[proc_macro_attribute]
pub fn stringify_braces(attr: TokenStream, item: TokenStream) -> TokenStream {
    stringify_all_helper('{', "}", attr, item).into()
}

/// Creates a macro `result` which expands to the same string literal as
/// `concat!("#[stringify_all[", stringify!(attr), "]]", stringify!(item))`
#[proc_macro_attribute]
pub fn stringify_brackets(attr: TokenStream, item: TokenStream) -> TokenStream {
    stringify_all_helper('[', "]", attr, item).into()
}

/// Creates a macro `result` which expands to the same string literal as
/// `concat!("#[stringify_all=", stringify!(attr), "]", stringify!(item))`
#[doc(hidden)]
#[proc_macro_attribute]
pub fn stringify_eq(attr: TokenStream, item: TokenStream) -> TokenStream {
    stringify_all_helper('=', "", attr, item).into()
}

#[cfg(test)]
mod test {
    use super::{stringify_helper, stringify_all_helper};
    use quote::quote;

    #[test]
    fn stringify_helper_simple() {
        assert_eq!(
            stringify_helper("foo").to_string(),
            quote!( macro_rules! result { () => { "foo" } } ).to_string(),
        );
    }

    #[test]
    fn stringify_all_simple() {
        assert_eq!(
            stringify_all_helper('{', "}", "foo", "bar").to_string(),
            quote!( macro_rules! result { () => { "#[stringify_all{foo}] bar" } } ).to_string(),
        );
    }
}