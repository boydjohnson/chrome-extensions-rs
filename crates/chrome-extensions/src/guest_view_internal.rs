#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "none"]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "Size" , typescript_type = "Size")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type Size;
    # [wasm_bindgen (method , getter , js_class = Size)]
    #[doc = ""]
    pub fn height(this: &Size) -> ::js_sys::Number;
    # [wasm_bindgen (method , getter , js_class = Size)]
    #[doc = ""]
    pub fn width(this: &Size) -> ::js_sys::Number;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "SizeParams" , typescript_type = "SizeParams")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Size parameters."]
    pub type SizeParams;
    # [wasm_bindgen (method , getter , js_class = SizeParams)]
    #[doc = ""]
    pub fn enableAutoSize(this: &SizeParams) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = SizeParams)]
    #[doc = ""]
    pub fn max(this: &SizeParams) -> Option<i32>;
    # [wasm_bindgen (method , getter , js_class = SizeParams)]
    #[doc = ""]
    pub fn min(this: &SizeParams) -> Option<i32>;
    # [wasm_bindgen (method , getter , js_class = SizeParams)]
    #[doc = ""]
    pub fn normal(this: &SizeParams) -> Option<i32>;
}
