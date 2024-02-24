#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>chrome.pageAction</code> API to put icons in the main Google Chrome toolbar, to the right of the address bar. Page actions represent actions that can be taken on the current page, but that aren't applicable to all pages. Page actions appear grayed out when inactive."]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "ImageDataType" , typescript_type = "ImageDataType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Pixel data for an image. Must be an ImageData object (for example, from a <code>canvas</code> element)."]
    pub type ImageDataType;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "TabDetails" , typescript_type = "TabDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type TabDetails;
    # [wasm_bindgen (method , getter , js_class = TabDetails)]
    #[doc = "The ID of the tab to query state for. If no tab is specified, the non-tab-specific state is returned."]
    pub fn tabId(this: &TabDetails) -> Option<::js_sys::Number>;
}
