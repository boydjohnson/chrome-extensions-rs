#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>chrome.storage</code> API to store, retrieve, and track changes to user data."]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "AccessLevel" , typescript_type = "AccessLevel")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The storage area's access level."]
    pub type AccessLevel;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "StorageChange" , typescript_type = "StorageChange")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type StorageChange;
    # [wasm_bindgen (method , getter , js_class = StorageChange)]
    #[doc = "The new value of the item, if there is a new value."]
    pub fn newValue(this: &StorageChange) -> ::wasm_bindgen::JsValue;
    # [wasm_bindgen (method , getter , js_class = StorageChange)]
    #[doc = "The old value of the item, if there was an old value."]
    pub fn oldValue(this: &StorageChange) -> ::wasm_bindgen::JsValue;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "StorageArea" , typescript_type = "StorageArea")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type StorageArea;
}
