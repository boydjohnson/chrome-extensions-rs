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
    pub async fn send_request(
        extensionId: Option<::js_sys::JsString>,
        request: ::wasm_bindgen::JsValue,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Converts a relative path within an extension install directory to a fully-qualified URL."]
    #[wasm_bindgen(js_name = "extension.getURL")]
    pub fn get_url(path: ::js_sys::JsString) -> ::js_sys::JsString;
    #[doc = "Returns an array of the JavaScript 'window' objects for each of the pages running inside the current extension."]
    #[wasm_bindgen(js_name = "extension.getViews")]
    pub fn get_views(fetchProperties: Option<::js_sys::Object>) -> ::js_sys::Array;
    #[doc = "Returns the JavaScript 'window' object for the background page running inside the current extension. Returns null if the extension has no background page."]
    #[wasm_bindgen(js_name = "extension.getBackgroundPage")]
    pub fn get_background_page() -> ::js_sys::Object;
    #[doc = "Returns an array of the JavaScript 'window' objects for each of the tabs running inside the current extension. If <code>windowId</code> is specified, returns only the 'window' objects of tabs attached to the specified window."]
    #[wasm_bindgen(js_name = "extension.getExtensionTabs")]
    pub fn get_extension_tabs(windowId: Option<::js_sys::Number>) -> ::js_sys::Array;
    #[doc = "Retrieves the state of the extension's access to Incognito-mode. This corresponds to the user-controlled per-extension 'Allowed in Incognito' setting accessible via the chrome://extensions page."]
    #[wasm_bindgen(js_name = "extension.isAllowedIncognitoAccess", catch)]
    pub async fn is_allowed_incognito_access(
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Retrieves the state of the extension's access to the 'file://' scheme. This corresponds to the user-controlled per-extension 'Allow access to File URLs' setting accessible via the chrome://extensions page."]
    #[wasm_bindgen(js_name = "extension.isAllowedFileSchemeAccess", catch)]
    pub async fn is_allowed_file_scheme_access(
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
}
#[wasm_bindgen]
pub async fn extension_send_request(
    extensionId: Option<::js_sys::JsString>,
    request: ::wasm_bindgen::JsValue,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    send_request(extensionId, request).await
}
#[wasm_bindgen]
pub fn extension_get_url(path: ::js_sys::JsString) -> ::js_sys::JsString {
    get_url(path)
}
#[wasm_bindgen]
pub fn extension_get_views(fetchProperties: Option<::js_sys::Object>) -> ::js_sys::Array {
    get_views(fetchProperties)
}
#[wasm_bindgen]
pub fn extension_get_background_page() -> ::js_sys::Object {
    get_background_page()
}
#[wasm_bindgen]
pub fn extension_get_extension_tabs(windowId: Option<::js_sys::Number>) -> ::js_sys::Array {
    get_extension_tabs(windowId)
}
#[wasm_bindgen]
pub async fn extension_is_allowed_incognito_access(
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    is_allowed_incognito_access().await
}
#[wasm_bindgen]
pub async fn extension_is_allowed_file_scheme_access(
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    is_allowed_file_scheme_access().await
}
