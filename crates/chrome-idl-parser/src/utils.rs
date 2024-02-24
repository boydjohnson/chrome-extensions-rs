use quote::quote;

pub fn generate_js_class(s: &str) -> proc_macro2::TokenStream {
    match s {
        "object" => quote!(::js_sys::Object),
        "string" => quote!(::js_sys::JsString),
        "array" => quote!(::js_sys::Array),
        "binary" => quote!(::js_sys::Int8Array),
        e => {
            println!("{}", e);
            panic!("found something other than string and object")
        }
    }
}
