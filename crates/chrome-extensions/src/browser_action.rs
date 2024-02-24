#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use browser actions to put icons in the main Google Chrome toolbar, to the right of the address bar. In addition to its <a href='browserAction#icon'>icon</a>, a browser action can have a <a href='browserAction#tooltip'>tooltip</a>, a <a href='browserAction#badge'>badge</a>, and a <a href='browserAction#popup'>popup</a>."]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Array , js_name = "ColorArray" , typescript_type = "ColorArray")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type ColorArray;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "ImageDataType" , typescript_type = "ImageDataType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Pixel data for an image. Must be an ImageData object; for example, from a <code>canvas</code> element."]
    pub type ImageDataType;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "TabDetails" , typescript_type = "TabDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type TabDetails;
    # [wasm_bindgen (method , getter , js_class = TabDetails)]
    #[doc = "The ID of the tab to query state for. If no tab is specified, the non-tab-specific state is returned."]
    pub fn tabId(this: &TabDetails) -> Option<::js_sys::Number>;
}
