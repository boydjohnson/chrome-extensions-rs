#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "none"]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "ContextMenuItem" , typescript_type = "ContextMenuItem")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "An item in the context menu."]
    pub type ContextMenuItem;
    # [wasm_bindgen (method , getter , js_class = ContextMenuItem)]
    #[doc = "id of the input item"]
    pub fn commandId(this: &ContextMenuItem) -> ::js_sys::Number;
    # [wasm_bindgen (method , getter , js_class = ContextMenuItem)]
    #[doc = "label of the item"]
    pub fn label(this: &ContextMenuItem) -> Option<::js_sys::JsString>;
}
