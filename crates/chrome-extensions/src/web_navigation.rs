#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>chrome.webNavigation</code> API to receive notifications about the status of navigation requests in-flight."]
# [wasm_bindgen (js_namespace = chrome)]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "webNavigation.TransitionType" , typescript_type = "webNavigation.TransitionType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Cause of the navigation. The same transition types as defined in the history API are used. These are the same transition types as defined in the <a href=\"history#transition_types\">history API</a> except with <code>\"start_page\"</code> in place of <code>\"auto_toplevel\"</code> (for backwards compatibility)."]
    pub type TransitionType;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "webNavigation.TransitionQualifier" , typescript_type = "webNavigation.TransitionQualifier")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type TransitionQualifier;
    #[doc = "Retrieves information about the given frame. A frame refers to an &lt;iframe&gt; or a &lt;frame&gt; of a web page and is identified by a tab ID and a frame ID."]
    #[wasm_bindgen(js_name = "webNavigation.getFrame", catch)]
    pub async fn get_frame(
        details: ::js_sys::Object,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Retrieves information about the given frame. A frame refers to an &lt;iframe&gt; or a &lt;frame&gt; of a web page and is identified by a tab ID and a frame ID."]
    #[wasm_bindgen(js_name = "webNavigation.getFrame")]
    pub fn get_frame_callback(details: ::js_sys::Object, callback: &::js_sys::Function);
    #[doc = "Retrieves information about all frames of a given tab."]
    #[wasm_bindgen(js_name = "webNavigation.getAllFrames", catch)]
    pub async fn get_all_frames(
        details: ::js_sys::Object,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Retrieves information about all frames of a given tab."]
    #[wasm_bindgen(js_name = "webNavigation.getAllFrames")]
    pub fn get_all_frames_callback(details: ::js_sys::Object, callback: &::js_sys::Function);
}
#[wasm_bindgen]
pub async fn web_navigation_get_frame(
    details: ::js_sys::Object,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    get_frame(details).await
}
#[wasm_bindgen]
pub fn web_navigation_get_frame_callback(details: ::js_sys::Object, callback: &::js_sys::Function) {
    get_frame_callback(details, callback);
}
#[wasm_bindgen]
pub async fn web_navigation_get_all_frames(
    details: ::js_sys::Object,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    get_all_frames(details).await
}
#[wasm_bindgen]
pub fn web_navigation_get_all_frames_callback(
    details: ::js_sys::Object,
    callback: &::js_sys::Function,
) {
    get_all_frames_callback(details, callback);
}
