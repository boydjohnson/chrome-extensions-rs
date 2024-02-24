#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>chrome.tabGroups</code> API to interact with the browser's tab grouping system. You can use this API to modify and rearrange tab groups in the browser. To group and ungroup tabs, or to query what tabs are in groups, use the <code>chrome.tabs</code> API."]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "Color" , typescript_type = "Color")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The group's color."]
    pub type Color;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "TabGroup" , typescript_type = "TabGroup")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type TabGroup;
    # [wasm_bindgen (method , getter , js_class = TabGroup)]
    #[doc = "Whether the group is collapsed. A collapsed group is one whose tabs are hidden."]
    pub fn collapsed(this: &TabGroup) -> ::js_sys::Boolean;
    # [wasm_bindgen (method , getter , js_class = TabGroup)]
    #[doc = "The group's color."]
    pub fn color(this: &TabGroup) -> i32;
    # [wasm_bindgen (method , getter , js_class = TabGroup)]
    #[doc = "The ID of the group. Group IDs are unique within a browser session."]
    pub fn id(this: &TabGroup) -> ::js_sys::Number;
    # [wasm_bindgen (method , getter , js_class = TabGroup)]
    #[doc = "The title of the group."]
    pub fn title(this: &TabGroup) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = TabGroup)]
    #[doc = "The ID of the window that contains the group."]
    pub fn windowId(this: &TabGroup) -> ::js_sys::Number;
}
