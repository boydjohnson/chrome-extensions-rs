#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>chrome.runtime</code> API to retrieve the service worker, return details about the manifest, and listen for and respond to events in the app or extension lifecycle. You can also use this API to convert the relative path of URLs to fully-qualified URLs."]
# [wasm_bindgen (js_namespace = chrome)]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "runtime.Port" , typescript_type = "runtime.Port")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "An object which allows two way communication with other pages. See <a href=\"messaging#connect\">Long-lived connections</a> for more information."]
    pub type Port;
    # [wasm_bindgen (method , getter , js_class = Port)]
    #[doc = "The name of the port, as specified in the call to $(ref:runtime.connect)."]
    pub fn name(this: &Port) -> ::js_sys::JsString;
    # [wasm_bindgen (method , getter , js_class = Port)]
    #[doc = "This property will <b>only</b> be present on ports passed to $(ref:runtime.onConnect onConnect) / $(ref:runtime.onConnectExternal onConnectExternal) / $(ref:runtime.onConnectExternal onConnectNative) listeners."]
    pub fn sender(this: &Port) -> Option<MessageSender>;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "runtime.MessageSender" , typescript_type = "runtime.MessageSender")]
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
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "runtime.PlatformOs" , typescript_type = "runtime.PlatformOs")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The operating system Chrome is running on."]
    pub type PlatformOs;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "runtime.PlatformArch" , typescript_type = "runtime.PlatformArch")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The machine's processor architecture."]
    pub type PlatformArch;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "runtime.PlatformNaclArch" , typescript_type = "runtime.PlatformNaclArch")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The native client architecture. This may be different from arch on some platforms."]
    pub type PlatformNaclArch;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "runtime.PlatformInfo" , typescript_type = "runtime.PlatformInfo")]
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
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "runtime.RequestUpdateCheckStatus" , typescript_type = "runtime.RequestUpdateCheckStatus")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Result of the update check."]
    pub type RequestUpdateCheckStatus;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "runtime.OnInstalledReason" , typescript_type = "runtime.OnInstalledReason")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The reason that this event is being dispatched."]
    pub type OnInstalledReason;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "runtime.OnRestartRequiredReason" , typescript_type = "runtime.OnRestartRequiredReason")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The reason that the event is being dispatched. 'app_update' is used when the restart is needed because the application is updated to a newer version. 'os_update' is used when the restart is needed because the browser/OS is updated to a newer version. 'periodic' is used when the system runs for more than the permitted uptime set in the enterprise policy."]
    pub type OnRestartRequiredReason;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "runtime.ContextType" , typescript_type = "runtime.ContextType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type ContextType;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "runtime.ExtensionContext" , typescript_type = "runtime.ExtensionContext")]
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
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "runtime.ContextFilter" , typescript_type = "runtime.ContextFilter")]
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
    #[doc = "Retrieves the JavaScript 'window' object for the background page running inside the current extension/app. If the background page is an event page, the system will ensure it is loaded before calling the callback. If there is no background page, an error is set."]
    #[wasm_bindgen(js_name = "runtime.getBackgroundPage", catch)]
    pub async fn get_background_page() -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Retrieves the JavaScript 'window' object for the background page running inside the current extension/app. If the background page is an event page, the system will ensure it is loaded before calling the callback. If there is no background page, an error is set."]
    #[wasm_bindgen(js_name = "runtime.getBackgroundPage")]
    pub fn get_background_page_callback(callback: &::js_sys::Function);
    #[doc = "<p>Open your Extension's options page, if possible.</p><p>The precise behavior may depend on your manifest's <code><a href=\"/docs/extensions/develop/ui/options-page#embedded_options\">options_ui</a></code> or <code><a href=\"/docs/extensions/develop/ui/options-page#full_page\">options_page</a></code> key, or what Chrome happens to support at the time. For example, the page may be opened in a new tab, within chrome://extensions, within an App, or it may just focus an open options page. It will never cause the caller page to reload.</p><p>If your Extension does not declare an options page, or Chrome failed to create one for some other reason, the callback will set $(ref:lastError).</p>"]
    #[wasm_bindgen(js_name = "runtime.openOptionsPage", catch)]
    pub async fn open_options_page() -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "<p>Open your Extension's options page, if possible.</p><p>The precise behavior may depend on your manifest's <code><a href=\"/docs/extensions/develop/ui/options-page#embedded_options\">options_ui</a></code> or <code><a href=\"/docs/extensions/develop/ui/options-page#full_page\">options_page</a></code> key, or what Chrome happens to support at the time. For example, the page may be opened in a new tab, within chrome://extensions, within an App, or it may just focus an open options page. It will never cause the caller page to reload.</p><p>If your Extension does not declare an options page, or Chrome failed to create one for some other reason, the callback will set $(ref:lastError).</p>"]
    #[wasm_bindgen(js_name = "runtime.openOptionsPage")]
    pub fn open_options_page_callback(callback: &::js_sys::Function);
    #[doc = "Returns details about the app or extension from the manifest. The object returned is a serialization of the full <a href=\"reference/manifest\">manifest file</a>."]
    #[wasm_bindgen(js_name = "runtime.getManifest")]
    pub fn get_manifest() -> ::js_sys::Object;
    #[doc = "Converts a relative path within an app/extension install directory to a fully-qualified URL."]
    #[wasm_bindgen(js_name = "runtime.getURL")]
    pub fn get_url(path: ::js_sys::JsString) -> ::js_sys::JsString;
    #[doc = "Sets the URL to be visited upon uninstallation. This may be used to clean up server-side data, do analytics, and implement surveys. Maximum 1023 characters."]
    #[wasm_bindgen(js_name = "runtime.setUninstallURL", catch)]
    pub async fn set_uninstall_url(url: ::js_sys::JsString) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Sets the URL to be visited upon uninstallation. This may be used to clean up server-side data, do analytics, and implement surveys. Maximum 1023 characters."]
    #[wasm_bindgen(js_name = "runtime.setUninstallURL")]
    pub fn set_uninstall_url_callback(url: ::js_sys::JsString, callback: &::js_sys::Function);
    #[doc = "<p>Requests an immediate update check be done for this app/extension.</p> <p><b>Important</b>: Most extensions/apps should <b>not</b> use this method, since Chrome already does automatic checks every few hours, and you can listen for the $(ref:runtime.onUpdateAvailable) event without needing to call requestUpdateCheck.</p><p>This method is only appropriate to call in very limited circumstances, such as if your extension/app talks to a backend service, and the backend service has determined that the client extension/app version is very far out of date and you'd like to prompt a user to update. Most other uses of requestUpdateCheck, such as calling it unconditionally based on a repeating timer, probably only serve to waste client, network, and server resources.</p><p>Note: When called with a callback, instead of returning an object this function will return the two properties as separate arguments passed to the callback.</p>"]
    #[wasm_bindgen(js_name = "runtime.requestUpdateCheck", catch)]
    pub async fn request_update_check() -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "<p>Requests an immediate update check be done for this app/extension.</p> <p><b>Important</b>: Most extensions/apps should <b>not</b> use this method, since Chrome already does automatic checks every few hours, and you can listen for the $(ref:runtime.onUpdateAvailable) event without needing to call requestUpdateCheck.</p><p>This method is only appropriate to call in very limited circumstances, such as if your extension/app talks to a backend service, and the backend service has determined that the client extension/app version is very far out of date and you'd like to prompt a user to update. Most other uses of requestUpdateCheck, such as calling it unconditionally based on a repeating timer, probably only serve to waste client, network, and server resources.</p><p>Note: When called with a callback, instead of returning an object this function will return the two properties as separate arguments passed to the callback.</p>"]
    #[wasm_bindgen(js_name = "runtime.requestUpdateCheck")]
    pub fn request_update_check_callback(callback: &::js_sys::Function);
    #[doc = "Restart the ChromeOS device when the app runs in kiosk mode after the given seconds. If called again before the time ends, the reboot will be delayed. If called with a value of -1, the reboot will be cancelled. It's a no-op in non-kiosk mode. It's only allowed to be called repeatedly by the first extension to invoke this API."]
    #[wasm_bindgen(js_name = "runtime.restartAfterDelay", catch)]
    pub async fn restart_after_delay(
        seconds: ::js_sys::Number,
    ) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Restart the ChromeOS device when the app runs in kiosk mode after the given seconds. If called again before the time ends, the reboot will be delayed. If called with a value of -1, the reboot will be cancelled. It's a no-op in non-kiosk mode. It's only allowed to be called repeatedly by the first extension to invoke this API."]
    #[wasm_bindgen(js_name = "runtime.restartAfterDelay")]
    pub fn restart_after_delay_callback(seconds: ::js_sys::Number, callback: &::js_sys::Function);
    #[doc = "Attempts to connect listeners within an extension/app (such as the background page), or other extensions/apps. This is useful for content scripts connecting to their extension processes, inter-app/extension communication, and <a href=\"/docs/extensions/manifest/externally_connectable\">web messaging</a>. Note that this does not connect to any listeners in a content script. Extensions may connect to content scripts embedded in tabs via $(ref:tabs.connect)."]
    #[wasm_bindgen(js_name = "runtime.connect")]
    pub fn connect(
        extensionId: Option<::js_sys::JsString>,
        connectInfo: Option<::js_sys::Object>,
    ) -> Port;
    #[doc = "Connects to a native application in the host machine. This method requires the <code>\"nativeMessaging\"</code> permission. See <a href=\"develop/concepts/native-messaging\">Native Messaging</a> for more information."]
    #[wasm_bindgen(js_name = "runtime.connectNative")]
    pub fn connect_native(application: ::js_sys::JsString) -> Port;
    #[doc = "Sends a single message to event listeners within your extension/app or a different extension/app. Similar to $(ref:runtime.connect) but only sends a single message, with an optional response. If sending to your extension, the $(ref:runtime.onMessage) event will be fired in every frame of your extension (except for the sender's frame), or $(ref:runtime.onMessageExternal), if a different extension. Note that extensions cannot send messages to content scripts using this method. To send messages to content scripts, use $(ref:tabs.sendMessage)."]
    #[wasm_bindgen(js_name = "runtime.sendMessage", catch)]
    pub async fn send_message(
        extensionId: Option<::js_sys::JsString>,
        message: ::wasm_bindgen::JsValue,
        options: Option<::js_sys::Object>,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Sends a single message to event listeners within your extension/app or a different extension/app. Similar to $(ref:runtime.connect) but only sends a single message, with an optional response. If sending to your extension, the $(ref:runtime.onMessage) event will be fired in every frame of your extension (except for the sender's frame), or $(ref:runtime.onMessageExternal), if a different extension. Note that extensions cannot send messages to content scripts using this method. To send messages to content scripts, use $(ref:tabs.sendMessage)."]
    #[wasm_bindgen(js_name = "runtime.sendMessage")]
    pub fn send_message_callback(
        extensionId: Option<::js_sys::JsString>,
        message: ::wasm_bindgen::JsValue,
        options: Option<::js_sys::Object>,
        callback: &::js_sys::Function,
    );
    #[doc = "Send a single message to a native application. This method requires the <code>\"nativeMessaging\"</code> permission."]
    #[wasm_bindgen(js_name = "runtime.sendNativeMessage", catch)]
    pub async fn send_native_message(
        application: ::js_sys::JsString,
        message: ::js_sys::Object,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Send a single message to a native application. This method requires the <code>\"nativeMessaging\"</code> permission."]
    #[wasm_bindgen(js_name = "runtime.sendNativeMessage")]
    pub fn send_native_message_callback(
        application: ::js_sys::JsString,
        message: ::js_sys::Object,
        callback: &::js_sys::Function,
    );
    #[doc = "Returns information about the current platform."]
    #[wasm_bindgen(js_name = "runtime.getPlatformInfo", catch)]
    pub async fn get_platform_info() -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Returns information about the current platform."]
    #[wasm_bindgen(js_name = "runtime.getPlatformInfo")]
    pub fn get_platform_info_callback(callback: &::js_sys::Function);
    #[doc = "Returns a DirectoryEntry for the package directory."]
    #[wasm_bindgen(js_name = "runtime.getPackageDirectoryEntry", catch)]
    pub async fn get_package_directory_entry(
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Returns a DirectoryEntry for the package directory."]
    #[wasm_bindgen(js_name = "runtime.getPackageDirectoryEntry")]
    pub fn get_package_directory_entry_callback(callback: &::js_sys::Function);
    #[doc = "Fetches information about active contexts associated with this extension"]
    #[wasm_bindgen(js_name = "runtime.getContexts", catch)]
    pub async fn get_contexts(
        filter: ContextFilter,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Fetches information about active contexts associated with this extension"]
    #[wasm_bindgen(js_name = "runtime.getContexts")]
    pub fn get_contexts_callback(filter: ContextFilter, callback: &::js_sys::Function);
}
#[wasm_bindgen]
pub async fn runtime_get_background_page(
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    get_background_page().await
}
#[wasm_bindgen]
pub fn runtime_get_background_page_callback(callback: &::js_sys::Function) {
    get_background_page_callback(callback);
}
#[wasm_bindgen]
pub async fn runtime_open_options_page() -> Result<(), ::wasm_bindgen::JsValue> {
    open_options_page().await
}
#[wasm_bindgen]
pub fn runtime_open_options_page_callback(callback: &::js_sys::Function) {
    open_options_page_callback(callback);
}
#[wasm_bindgen]
pub fn runtime_get_manifest() -> ::js_sys::Object {
    get_manifest()
}
#[wasm_bindgen]
pub fn runtime_get_url(path: ::js_sys::JsString) -> ::js_sys::JsString {
    get_url(path)
}
#[wasm_bindgen]
pub async fn runtime_set_uninstall_url(
    url: ::js_sys::JsString,
) -> Result<(), ::wasm_bindgen::JsValue> {
    set_uninstall_url(url).await
}
#[wasm_bindgen]
pub fn runtime_set_uninstall_url_callback(url: ::js_sys::JsString, callback: &::js_sys::Function) {
    set_uninstall_url_callback(url, callback);
}
#[wasm_bindgen]
pub async fn runtime_request_update_check(
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    request_update_check().await
}
#[wasm_bindgen]
pub fn runtime_request_update_check_callback(callback: &::js_sys::Function) {
    request_update_check_callback(callback);
}
#[wasm_bindgen]
pub async fn runtime_restart_after_delay(
    seconds: ::js_sys::Number,
) -> Result<(), ::wasm_bindgen::JsValue> {
    restart_after_delay(seconds).await
}
#[wasm_bindgen]
pub fn runtime_restart_after_delay_callback(
    seconds: ::js_sys::Number,
    callback: &::js_sys::Function,
) {
    restart_after_delay_callback(seconds, callback);
}
#[wasm_bindgen]
pub fn runtime_connect(
    extensionId: Option<::js_sys::JsString>,
    connectInfo: Option<::js_sys::Object>,
) -> Port {
    connect(extensionId, connectInfo)
}
#[wasm_bindgen]
pub fn runtime_connect_native(application: ::js_sys::JsString) -> Port {
    connect_native(application)
}
#[wasm_bindgen]
pub async fn runtime_send_message(
    extensionId: Option<::js_sys::JsString>,
    message: ::wasm_bindgen::JsValue,
    options: Option<::js_sys::Object>,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    send_message(extensionId, message, options).await
}
#[wasm_bindgen]
pub fn runtime_send_message_callback(
    extensionId: Option<::js_sys::JsString>,
    message: ::wasm_bindgen::JsValue,
    options: Option<::js_sys::Object>,
    callback: &::js_sys::Function,
) {
    send_message_callback(extensionId, message, options, callback);
}
#[wasm_bindgen]
pub async fn runtime_send_native_message(
    application: ::js_sys::JsString,
    message: ::js_sys::Object,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    send_native_message(application, message).await
}
#[wasm_bindgen]
pub fn runtime_send_native_message_callback(
    application: ::js_sys::JsString,
    message: ::js_sys::Object,
    callback: &::js_sys::Function,
) {
    send_native_message_callback(application, message, callback);
}
#[wasm_bindgen]
pub async fn runtime_get_platform_info() -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>
{
    get_platform_info().await
}
#[wasm_bindgen]
pub fn runtime_get_platform_info_callback(callback: &::js_sys::Function) {
    get_platform_info_callback(callback);
}
#[wasm_bindgen]
pub async fn runtime_get_package_directory_entry(
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    get_package_directory_entry().await
}
#[wasm_bindgen]
pub fn runtime_get_package_directory_entry_callback(callback: &::js_sys::Function) {
    get_package_directory_entry_callback(callback);
}
#[wasm_bindgen]
pub async fn runtime_get_contexts(
    filter: ContextFilter,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    get_contexts(filter).await
}
#[wasm_bindgen]
pub fn runtime_get_contexts_callback(filter: ContextFilter, callback: &::js_sys::Function) {
    get_contexts_callback(filter, callback);
}
