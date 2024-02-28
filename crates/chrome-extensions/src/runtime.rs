#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>chrome.runtime</code> API to retrieve the service worker, return details about the manifest, and listen for and respond to events in the app or extension lifecycle. You can also use this API to convert the relative path of URLs to fully-qualified URLs."]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "Port" , typescript_type = "Port")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "An object which allows two way communication with other pages. See <a href=\"messaging#connect\">Long-lived connections</a> for more information."]
    pub type Port;
    # [wasm_bindgen (method , getter , js_class = Port)]
    #[doc = "The name of the port, as specified in the call to $(ref:runtime.connect)."]
    pub fn name(this: &Port) -> ::js_sys::JsString;
    # [wasm_bindgen (method , getter , js_class = Port)]
    #[doc = "This property will <b>only</b> be present on ports passed to $(ref:runtime.onConnect onConnect) / $(ref:runtime.onConnectExternal onConnectExternal) / $(ref:runtime.onConnectExternal onConnectNative) listeners."]
    pub fn sender(this: &Port) -> Option<MessageSender>;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "MessageSender" , typescript_type = "MessageSender")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "An object containing information about the script context that sent a message or request."]
    pub type MessageSender;
    # [wasm_bindgen (method , getter , js_class = MessageSender)]
    #[doc = "A UUID of the document that opened the connection."]
    pub fn documentId(this: &MessageSender) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = MessageSender)]
    #[doc = "The lifecycle the document that opened the connection is in at the time the port was created. Note that the lifecycle state of the document may have changed since port creation."]
    pub fn documentLifecycle(this: &MessageSender) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = MessageSender)]
    #[doc = "The <a href='webNavigation#frame_ids'>frame</a> that opened the connection. 0 for top-level frames, positive for child frames. This will only be set when <code>tab</code> is set."]
    pub fn frameId(this: &MessageSender) -> Option<::js_sys::Number>;
    # [wasm_bindgen (method , getter , js_class = MessageSender)]
    #[doc = "The guest process id of the requesting webview, if available. Only available for component extensions."]
    pub fn guestProcessId(this: &MessageSender) -> Option<::js_sys::Number>;
    # [wasm_bindgen (method , getter , js_class = MessageSender)]
    #[doc = "The guest render frame routing id of the requesting webview, if available. Only available for component extensions."]
    pub fn guestRenderFrameRoutingId(this: &MessageSender) -> Option<::js_sys::Number>;
    # [wasm_bindgen (method , getter , js_class = MessageSender)]
    #[doc = "The ID of the extension or app that opened the connection, if any."]
    pub fn id(this: &MessageSender) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = MessageSender)]
    #[doc = "The name of the native application that opened the connection, if any."]
    pub fn nativeApplication(this: &MessageSender) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = MessageSender)]
    #[doc = "The origin of the page or frame that opened the connection. It can vary from the url property (e.g., about:blank) or can be opaque (e.g., sandboxed iframes). This is useful for identifying if the origin can be trusted if we can't immediately tell from the URL."]
    pub fn origin(this: &MessageSender) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = MessageSender)]
    #[doc = "The $(ref:tabs.Tab) which opened the connection, if any. This property will <strong>only</strong> be present when the connection was opened from a tab (including content scripts), and <strong>only</strong> if the receiver is an extension, not an app."]
    pub fn tab(this: &MessageSender) -> Option<crate::tabs::Tab>;
    # [wasm_bindgen (method , getter , js_class = MessageSender)]
    #[doc = "The TLS channel ID of the page or frame that opened the connection, if requested by the extension or app, and if available."]
    pub fn tlsChannelId(this: &MessageSender) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = MessageSender)]
    #[doc = "The URL of the page or frame that opened the connection. If the sender is in an iframe, it will be iframe's URL not the URL of the page which hosts it."]
    pub fn url(this: &MessageSender) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "PlatformOs" , typescript_type = "PlatformOs")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The operating system Chrome is running on."]
    pub type PlatformOs;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "PlatformArch" , typescript_type = "PlatformArch")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The machine's processor architecture."]
    pub type PlatformArch;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "PlatformNaclArch" , typescript_type = "PlatformNaclArch")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The native client architecture. This may be different from arch on some platforms."]
    pub type PlatformNaclArch;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "PlatformInfo" , typescript_type = "PlatformInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "An object containing information about the current platform."]
    pub type PlatformInfo;
    # [wasm_bindgen (method , getter , js_class = PlatformInfo)]
    #[doc = "The machine's processor architecture."]
    pub fn arch(this: &PlatformInfo) -> PlatformArch;
    # [wasm_bindgen (method , getter , js_class = PlatformInfo)]
    #[doc = "The native client architecture. This may be different from arch on some platforms."]
    pub fn nacl_arch(this: &PlatformInfo) -> PlatformNaclArch;
    # [wasm_bindgen (method , getter , js_class = PlatformInfo)]
    #[doc = "The operating system Chrome is running on."]
    pub fn os(this: &PlatformInfo) -> PlatformOs;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "RequestUpdateCheckStatus" , typescript_type = "RequestUpdateCheckStatus")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Result of the update check."]
    pub type RequestUpdateCheckStatus;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "OnInstalledReason" , typescript_type = "OnInstalledReason")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The reason that this event is being dispatched."]
    pub type OnInstalledReason;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "OnRestartRequiredReason" , typescript_type = "OnRestartRequiredReason")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The reason that the event is being dispatched. 'app_update' is used when the restart is needed because the application is updated to a newer version. 'os_update' is used when the restart is needed because the browser/OS is updated to a newer version. 'periodic' is used when the system runs for more than the permitted uptime set in the enterprise policy."]
    pub type OnRestartRequiredReason;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "ContextType" , typescript_type = "ContextType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type ContextType;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "ExtensionContext" , typescript_type = "ExtensionContext")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "A context hosting extension content."]
    pub type ExtensionContext;
    # [wasm_bindgen (method , getter , js_class = ExtensionContext)]
    #[doc = "A unique identifier for this context"]
    pub fn contextId(this: &ExtensionContext) -> ::js_sys::JsString;
    # [wasm_bindgen (method , getter , js_class = ExtensionContext)]
    #[doc = "The type of context this corresponds to."]
    pub fn contextType(this: &ExtensionContext) -> ContextType;
    # [wasm_bindgen (method , getter , js_class = ExtensionContext)]
    #[doc = "A UUID for the document associated with this context, or undefined if this context is hosted not in a document."]
    pub fn documentId(this: &ExtensionContext) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = ExtensionContext)]
    #[doc = "The origin of the document associated with this context, or undefined if the context is not hosted in a document."]
    pub fn documentOrigin(this: &ExtensionContext) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = ExtensionContext)]
    #[doc = "The URL of the document associated with this context, or undefined if the context is not hosted in a document."]
    pub fn documentUrl(this: &ExtensionContext) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = ExtensionContext)]
    #[doc = "The ID of the frame for this context, or -1 if this context is not hosted in a frame."]
    pub fn frameId(this: &ExtensionContext) -> ::js_sys::Number;
    # [wasm_bindgen (method , getter , js_class = ExtensionContext)]
    #[doc = "Whether the context is associated with an incognito profile."]
    pub fn incognito(this: &ExtensionContext) -> ::js_sys::Boolean;
    # [wasm_bindgen (method , getter , js_class = ExtensionContext)]
    #[doc = "The ID of the tab for this context, or -1 if this context is not hosted in a tab."]
    pub fn tabId(this: &ExtensionContext) -> ::js_sys::Number;
    # [wasm_bindgen (method , getter , js_class = ExtensionContext)]
    #[doc = "The ID of the window for this context, or -1 if this context is not hosted in a window."]
    pub fn windowId(this: &ExtensionContext) -> ::js_sys::Number;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "ContextFilter" , typescript_type = "ContextFilter")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "A filter to match against certain extension contexts. Matching contexts must match all specified filters; any filter that is not specified matches all available contexts. Thus, a filter of `{}` will match all available contexts."]
    pub type ContextFilter;
    # [wasm_bindgen (method , getter , js_class = ContextFilter)]
    #[doc = ""]
    pub fn contextIds(this: &ContextFilter) -> Option<::js_sys::Array>;
    # [wasm_bindgen (method , getter , js_class = ContextFilter)]
    #[doc = ""]
    pub fn contextTypes(this: &ContextFilter) -> Option<::js_sys::Array>;
    # [wasm_bindgen (method , getter , js_class = ContextFilter)]
    #[doc = ""]
    pub fn documentIds(this: &ContextFilter) -> Option<::js_sys::Array>;
    # [wasm_bindgen (method , getter , js_class = ContextFilter)]
    #[doc = ""]
    pub fn documentOrigins(this: &ContextFilter) -> Option<::js_sys::Array>;
    # [wasm_bindgen (method , getter , js_class = ContextFilter)]
    #[doc = ""]
    pub fn documentUrls(this: &ContextFilter) -> Option<::js_sys::Array>;
    # [wasm_bindgen (method , getter , js_class = ContextFilter)]
    #[doc = ""]
    pub fn frameIds(this: &ContextFilter) -> Option<::js_sys::Array>;
    # [wasm_bindgen (method , getter , js_class = ContextFilter)]
    #[doc = ""]
    pub fn incognito(this: &ContextFilter) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = ContextFilter)]
    #[doc = ""]
    pub fn tabIds(this: &ContextFilter) -> Option<::js_sys::Array>;
    # [wasm_bindgen (method , getter , js_class = ContextFilter)]
    #[doc = ""]
    pub fn windowIds(this: &ContextFilter) -> Option<::js_sys::Array>;
}
