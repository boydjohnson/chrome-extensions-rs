#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>chrome.sessions</code> API to query and restore tabs and windows from a browsing session."]
# [wasm_bindgen (js_namespace = chrome)]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "sessions.Filter" , typescript_type = "sessions.Filter")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type Filter;
    # [wasm_bindgen (method , getter , js_class = Filter)]
    #[doc = "The maximum number of entries to be fetched in the requested list. Omit this parameter to fetch the maximum number of entries ($(ref:sessions.MAX_SESSION_RESULTS))."]
    pub fn maxResults(this: &Filter) -> Option<::js_sys::Number>;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "sessions.Session" , typescript_type = "sessions.Session")]
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
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "sessions.Device" , typescript_type = "sessions.Device")]
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
    #[doc = "Gets the list of recently closed tabs and/or windows."]
    #[wasm_bindgen(js_name = "sessions.getRecentlyClosed", catch)]
    pub async fn getRecentlyClosed(
        filter: Option<Filter>,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Retrieves all devices with synced sessions."]
    #[wasm_bindgen(js_name = "sessions.getDevices", catch)]
    pub async fn getDevices(
        filter: Option<Filter>,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Reopens a $(ref:windows.Window) or $(ref:tabs.Tab), with an optional callback to run when the entry has been restored."]
    #[wasm_bindgen(js_name = "sessions.restore", catch)]
    pub async fn restore(
        sessionId: Option<::js_sys::JsString>,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
}
#[wasm_bindgen]
pub async fn sessions_get_recently_closed(
    filter: Option<Filter>,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    getRecentlyClosed(filter).await
}
#[wasm_bindgen]
pub async fn sessions_get_devices(
    filter: Option<Filter>,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    getDevices(filter).await
}
#[wasm_bindgen]
pub async fn sessions_restore(
    sessionId: Option<::js_sys::JsString>,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    restore(sessionId).await
}
