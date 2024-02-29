#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "The <code>chrome.extension</code> API has utilities that can be used by any extension page. It includes support for exchanging messages between an extension and its content scripts or between extensions, as described in detail in <a href='messaging'>Message Passing</a>."]
# [wasm_bindgen (js_namespace = chrome)]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "extension.ViewType" , typescript_type = "extension.ViewType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The type of extension view."]
    pub type ViewType;
    #[doc = "Sends a single request to other listeners within the extension. Similar to $(ref:runtime.connect), but only sends a single request with an optional response. The $(ref:extension.onRequest) event is fired in each page of the extension."]
    #[wasm_bindgen(js_name = "extension.sendRequest", catch)]
    pub async fn sendRequest(
        extensionId: Option<::js_sys::JsString>,
        request: ::wasm_bindgen::JsValue,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Converts a relative path within an extension install directory to a fully-qualified URL."]
    #[wasm_bindgen(js_name = "extension.getURL")]
    pub fn getURL(path: ::js_sys::JsString) -> ::js_sys::JsString;
    #[doc = "Returns an array of the JavaScript 'window' objects for each of the pages running inside the current extension."]
    #[wasm_bindgen(js_name = "extension.getViews")]
    pub fn getViews(fetchProperties: Option<::js_sys::Object>) -> ::js_sys::Array;
    #[doc = "Returns the JavaScript 'window' object for the background page running inside the current extension. Returns null if the extension has no background page."]
    #[wasm_bindgen(js_name = "extension.getBackgroundPage")]
    pub fn getBackgroundPage() -> ::js_sys::Object;
    #[doc = "Returns an array of the JavaScript 'window' objects for each of the tabs running inside the current extension. If <code>windowId</code> is specified, returns only the 'window' objects of tabs attached to the specified window."]
    #[wasm_bindgen(js_name = "extension.getExtensionTabs")]
    pub fn getExtensionTabs(windowId: Option<::js_sys::Number>) -> ::js_sys::Array;
    #[doc = "Retrieves the state of the extension's access to Incognito-mode. This corresponds to the user-controlled per-extension 'Allowed in Incognito' setting accessible via the chrome://extensions page."]
    #[wasm_bindgen(js_name = "extension.isAllowedIncognitoAccess", catch)]
    pub async fn isAllowedIncognitoAccess(
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Retrieves the state of the extension's access to the 'file://' scheme. This corresponds to the user-controlled per-extension 'Allow access to File URLs' setting accessible via the chrome://extensions page."]
    #[wasm_bindgen(js_name = "extension.isAllowedFileSchemeAccess", catch)]
    pub async fn isAllowedFileSchemeAccess(
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
}
