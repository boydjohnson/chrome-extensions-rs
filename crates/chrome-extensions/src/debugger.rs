#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "The <code>chrome.debugger</code> API serves as an alternate transport for Chrome's <a href='https://developer.chrome.com/devtools/docs/debugger-protocol'>remote debugging protocol</a>. Use <code>chrome.debugger</code> to attach to one or more tabs to instrument network interaction, debug JavaScript, mutate the DOM and CSS, etc. Use the Debuggee <code>tabId</code> to target tabs with sendCommand and route events by <code>tabId</code> from onEvent callbacks."]
# [wasm_bindgen (js_namespace = chrome)]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "debugger.Debuggee" , typescript_type = "debugger.Debuggee")]
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
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "debugger.TargetInfoType" , typescript_type = "debugger.TargetInfoType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Target type."]
    pub type TargetInfoType;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "debugger.DetachReason" , typescript_type = "debugger.DetachReason")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Connection termination reason."]
    pub type DetachReason;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "debugger.TargetInfo" , typescript_type = "debugger.TargetInfo")]
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
    #[doc = "Attaches debugger to the given target."]
    #[wasm_bindgen(js_name = "debugger.attach", catch)]
    pub async fn attach(
        target: Debuggee,
        required_version: ::js_sys::JsString,
    ) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Attaches debugger to the given target."]
    #[wasm_bindgen(js_name = "debugger.attach")]
    pub fn attach_callback(
        target: Debuggee,
        required_version: ::js_sys::JsString,
        callback: &::js_sys::Function,
    );
    #[doc = "Detaches debugger from the given target."]
    #[wasm_bindgen(js_name = "debugger.detach", catch)]
    pub async fn detach(target: Debuggee) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Detaches debugger from the given target."]
    #[wasm_bindgen(js_name = "debugger.detach")]
    pub fn detach_callback(target: Debuggee, callback: &::js_sys::Function);
    #[doc = "Sends given command to the debugging target."]
    #[wasm_bindgen(js_name = "debugger.sendCommand", catch)]
    pub async fn send_command(
        target: Debuggee,
        method: ::js_sys::JsString,
        command_params: Option<::js_sys::Object>,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Sends given command to the debugging target."]
    #[wasm_bindgen(js_name = "debugger.sendCommand")]
    pub fn send_command_callback(
        target: Debuggee,
        method: ::js_sys::JsString,
        command_params: Option<::js_sys::Object>,
        callback: &::js_sys::Function,
    );
    #[doc = "Returns the list of available debug targets."]
    #[wasm_bindgen(js_name = "debugger.getTargets", catch)]
    pub async fn get_targets() -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Returns the list of available debug targets."]
    #[wasm_bindgen(js_name = "debugger.getTargets")]
    pub fn get_targets_callback(callback: &::js_sys::Function);
}
#[wasm_bindgen]
pub async fn debugger_attach(
    target: Debuggee,
    required_version: ::js_sys::JsString,
) -> Result<(), ::wasm_bindgen::JsValue> {
    attach(target, required_version).await
}
#[wasm_bindgen]
pub fn debugger_attach_callback(
    target: Debuggee,
    required_version: ::js_sys::JsString,
    callback: &::js_sys::Function,
) {
    attach_callback(target, required_version, callback);
}
#[wasm_bindgen]
pub async fn debugger_detach(target: Debuggee) -> Result<(), ::wasm_bindgen::JsValue> {
    detach(target).await
}
#[wasm_bindgen]
pub fn debugger_detach_callback(target: Debuggee, callback: &::js_sys::Function) {
    detach_callback(target, callback);
}
#[wasm_bindgen]
pub async fn debugger_send_command(
    target: Debuggee,
    method: ::js_sys::JsString,
    command_params: Option<::js_sys::Object>,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    send_command(target, method, command_params).await
}
#[wasm_bindgen]
pub fn debugger_send_command_callback(
    target: Debuggee,
    method: ::js_sys::JsString,
    command_params: Option<::js_sys::Object>,
    callback: &::js_sys::Function,
) {
    send_command_callback(target, method, command_params, callback);
}
#[wasm_bindgen]
pub async fn debugger_get_targets() -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    get_targets().await
}
#[wasm_bindgen]
pub fn debugger_get_targets_callback(callback: &::js_sys::Function) {
    get_targets_callback(callback);
}
