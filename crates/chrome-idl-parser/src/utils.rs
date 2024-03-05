use {
    proc_macro2::{Ident, Span},
    quote::quote,
    syn::{punctuated::Punctuated, Path, PathSegment},
    to_snake_case::ToSnakeCase,
};

pub fn generate_js_class(s: &str) -> proc_macro2::TokenStream {
    match s {
        "object" => quote!(::js_sys::Object),
        "string" => quote!(::js_sys::JsString),
        "array" => quote!(::js_sys::Array),
        "binary" => quote!(::web_sys::Blob),
        "integer" => quote!(::js_sys::Number),
        "number" => quote!(::js_sys::Number),
        "boolean" => quote!(::js_sys::Boolean),
        "any" => quote!(::wasm_bindgen::JsValue),
        "function" => quote!(::js_sys::Function),
        e => {
            println!("{}", e);
            panic!("found something other than string and object")
        }
    }
}

pub fn create_fully_qualified(s: &str) -> proc_macro2::TokenStream {
    let mut both = s.split('.');

    let first = both.next().unwrap().to_snake_case();
    let second = both.next().unwrap();

    let c = Ident::new("crate", Span::call_site());
    let first = Ident::new(&first, Span::call_site());
    let second = Ident::new(second, Span::call_site());

    let mut segments = Punctuated::new();
    segments.push(PathSegment::from(c));
    segments.push(PathSegment::from(first));
    segments.push(PathSegment::from(second));

    let path = Path {
        leading_colon: None,
        segments,
    };

    quote!(#path)
}
