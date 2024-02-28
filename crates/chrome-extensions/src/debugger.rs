#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "The <code>chrome.debugger</code> API serves as an alternate transport for Chrome's <a href='https://developer.chrome.com/devtools/docs/debugger-protocol'>remote debugging protocol</a>. Use <code>chrome.debugger</code> to attach to one or more tabs to instrument network interaction, debug JavaScript, mutate the DOM and CSS, etc. Use the Debuggee <code>tabId</code> to target tabs with sendCommand and route events by <code>tabId</code> from onEvent callbacks."]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "Debuggee" , typescript_type = "Debuggee")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Debuggee identifier. Either tabId or extensionId must be specified"]
    pub type Debuggee;
    # [wasm_bindgen (method , getter , js_class = Debuggee)]
    #[doc = "The id of the extension which you intend to debug. Attaching to an extension background page is only possible when the <code>--silent-debugger-extension-api</code> command-line switch is used."]
    pub fn extensionId(this: &Debuggee) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = Debuggee)]
    #[doc = "The id of the tab which you intend to debug."]
    pub fn tabId(this: &Debuggee) -> Option<::js_sys::Number>;
    # [wasm_bindgen (method , getter , js_class = Debuggee)]
    #[doc = "The opaque id of the debug target."]
    pub fn targetId(this: &Debuggee) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "TargetInfoType" , typescript_type = "TargetInfoType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Target type."]
    pub type TargetInfoType;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "DetachReason" , typescript_type = "DetachReason")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Connection termination reason."]
    pub type DetachReason;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "TargetInfo" , typescript_type = "TargetInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Debug target information"]
    pub type TargetInfo;
    # [wasm_bindgen (method , getter , js_class = TargetInfo)]
    #[doc = "True if debugger is already attached."]
    pub fn attached(this: &TargetInfo) -> ::js_sys::Boolean;
    # [wasm_bindgen (method , getter , js_class = TargetInfo)]
    #[doc = "The extension id, defined if type = 'background_page'."]
    pub fn extensionId(this: &TargetInfo) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = TargetInfo)]
    #[doc = "Target favicon URL."]
    pub fn faviconUrl(this: &TargetInfo) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = TargetInfo)]
    #[doc = "Target id."]
    pub fn id(this: &TargetInfo) -> ::js_sys::JsString;
    # [wasm_bindgen (method , getter , js_class = TargetInfo)]
    #[doc = "The tab id, defined if type == 'page'."]
    pub fn tabId(this: &TargetInfo) -> Option<::js_sys::Number>;
    # [wasm_bindgen (method , getter , js_class = TargetInfo)]
    #[doc = "Target page title."]
    pub fn title(this: &TargetInfo) -> ::js_sys::JsString;
    # [wasm_bindgen (method , getter , js_class = TargetInfo)]
    #[doc = "Target type."]
    pub fn type_(this: &TargetInfo) -> TargetInfoType;
    # [wasm_bindgen (method , getter , js_class = TargetInfo)]
    #[doc = "Target URL."]
    pub fn url(this: &TargetInfo) -> ::js_sys::JsString;
}
