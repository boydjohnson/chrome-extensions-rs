#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>chrome.sessions</code> API to query and restore tabs and windows from a browsing session."]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "Filter" , typescript_type = "Filter")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type Filter;
    # [wasm_bindgen (method , getter , js_class = Filter)]
    #[doc = "The maximum number of entries to be fetched in the requested list. Omit this parameter to fetch the maximum number of entries ($(ref:sessions.MAX_SESSION_RESULTS))."]
    pub fn maxResults(this: &Filter) -> Option<::js_sys::Number>;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "Session" , typescript_type = "Session")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type Session;
    # [wasm_bindgen (method , getter , js_class = Session)]
    #[doc = "The time when the window or tab was closed or modified, represented in milliseconds since the epoch."]
    pub fn lastModified(this: &Session) -> ::js_sys::Number;
    # [wasm_bindgen (method , getter , js_class = Session)]
    #[doc = "The $(ref:tabs.Tab), if this entry describes a tab. Either this or $(ref:sessions.Session.window) will be set."]
    pub fn tab(this: &Session) -> Option<crate::tabs::Tab>;
    # [wasm_bindgen (method , getter , js_class = Session)]
    #[doc = "The $(ref:windows.Window), if this entry describes a window. Either this or $(ref:sessions.Session.tab) will be set."]
    pub fn window(this: &Session) -> Option<crate::windows::Window>;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "Device" , typescript_type = "Device")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type Device;
    # [wasm_bindgen (method , getter , js_class = Device)]
    #[doc = "The name of the foreign device."]
    pub fn deviceName(this: &Device) -> ::js_sys::JsString;
    # [wasm_bindgen (method , getter , js_class = Device)]
    #[doc = ""]
    pub fn info(this: &Device) -> ::js_sys::JsString;
    # [wasm_bindgen (method , getter , js_class = Device)]
    #[doc = "A list of open window sessions for the foreign device, sorted from most recently to least recently modified session."]
    pub fn sessions(this: &Device) -> ::js_sys::Array;
}
