#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>chrome.tabGroups</code> API to interact with the browser's tab grouping system. You can use this API to modify and rearrange tab groups in the browser. To group and ungroup tabs, or to query what tabs are in groups, use the <code>chrome.tabs</code> API."]
# [wasm_bindgen (js_namespace = chrome)]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "tabGroups.Color" , typescript_type = "tabGroups.Color")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The group's color."]
    pub type Color;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "tabGroups.TabGroup" , typescript_type = "tabGroups.TabGroup")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type TabGroup;
    # [wasm_bindgen (method , getter , js_class = TabGroup)]
    #[doc = "Whether the group is collapsed. A collapsed group is one whose tabs are hidden."]
    pub fn collapsed(this: &TabGroup) -> ::js_sys::Boolean;
    # [wasm_bindgen (method , getter , js_class = TabGroup)]
    #[doc = "The group's color."]
    pub fn color(this: &TabGroup) -> Color;
    # [wasm_bindgen (method , getter , js_class = TabGroup)]
    #[doc = "The ID of the group. Group IDs are unique within a browser session."]
    pub fn id(this: &TabGroup) -> ::js_sys::Number;
    # [wasm_bindgen (method , getter , js_class = TabGroup)]
    #[doc = "The title of the group."]
    pub fn title(this: &TabGroup) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = TabGroup)]
    #[doc = "The ID of the window that contains the group."]
    pub fn windowId(this: &TabGroup) -> ::js_sys::Number;
    #[doc = "Retrieves details about the specified group."]
    #[wasm_bindgen(js_name = "tabGroups.get", catch)]
    pub async fn get(
        groupId: ::js_sys::Number,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Gets all groups that have the specified properties, or all groups if no properties are specified."]
    #[wasm_bindgen(js_name = "tabGroups.query", catch)]
    pub async fn query(
        queryInfo: ::js_sys::Object,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Modifies the properties of a group. Properties that are not specified in <var>updateProperties</var> are not modified."]
    #[wasm_bindgen(js_name = "tabGroups.update", catch)]
    pub async fn update(
        groupId: ::js_sys::Number,
        updateProperties: ::js_sys::Object,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Moves the group and all its tabs within its window, or to a new window."]
    #[wasm_bindgen(js_name = "tabGroups.move", catch)]
    pub async fn move_(
        groupId: ::js_sys::Number,
        moveProperties: ::js_sys::Object,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
}
