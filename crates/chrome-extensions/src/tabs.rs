#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>chrome.tabs</code> API to interact with the browser's tab system. You can use this API to create, modify, and rearrange tabs in the browser."]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "TabStatus" , typescript_type = "TabStatus")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The tab's loading status."]
    pub type TabStatus;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "MutedInfoReason" , typescript_type = "MutedInfoReason")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "An event that caused a muted state change."]
    pub type MutedInfoReason;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "MutedInfo" , typescript_type = "MutedInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The tab's muted state and the reason for the last state change."]
    pub type MutedInfo;
    # [wasm_bindgen (method , getter , js_class = MutedInfo)]
    #[doc = "The ID of the extension that changed the muted state. Not set if an extension was not the reason the muted state last changed."]
    pub fn extensionId(this: &MutedInfo) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = MutedInfo)]
    #[doc = "Whether the tab is muted (prevented from playing sound). The tab may be muted even if it has not played or is not currently playing sound. Equivalent to whether the 'muted' audio indicator is showing."]
    pub fn muted(this: &MutedInfo) -> ::js_sys::Boolean;
    # [wasm_bindgen (method , getter , js_class = MutedInfo)]
    #[doc = "The reason the tab was muted or unmuted. Not set if the tab's mute state has never been changed."]
    pub fn reason(this: &MutedInfo) -> Option<i32>;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "Tab" , typescript_type = "Tab")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type Tab;
    # [wasm_bindgen (method , getter , js_class = Tab)]
    #[doc = "Whether the tab is active in its window. Does not necessarily mean the window is focused."]
    pub fn active(this: &Tab) -> ::js_sys::Boolean;
    # [wasm_bindgen (method , getter , js_class = Tab)]
    #[doc = "Whether the tab has produced sound over the past couple of seconds (but it might not be heard if also muted). Equivalent to whether the 'speaker audio' indicator is showing."]
    pub fn audible(this: &Tab) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = Tab)]
    #[doc = "Whether the tab can be discarded automatically by the browser when resources are low."]
    pub fn autoDiscardable(this: &Tab) -> ::js_sys::Boolean;
    # [wasm_bindgen (method , getter , js_class = Tab)]
    #[doc = "Whether the tab is discarded. A discarded tab is one whose content has been unloaded from memory, but is still visible in the tab strip. Its content is reloaded the next time it is activated."]
    pub fn discarded(this: &Tab) -> ::js_sys::Boolean;
    # [wasm_bindgen (method , getter , js_class = Tab)]
    #[doc = "The URL of the tab's favicon. This property is only present if the extension's manifest includes the <code>\"tabs\"</code> permission. It may also be an empty string if the tab is loading."]
    pub fn favIconUrl(this: &Tab) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = Tab)]
    #[doc = "The ID of the group that the tab belongs to."]
    pub fn groupId(this: &Tab) -> ::js_sys::Number;
    # [wasm_bindgen (method , getter , js_class = Tab)]
    #[doc = "The height of the tab in pixels."]
    pub fn height(this: &Tab) -> Option<::js_sys::Number>;
    # [wasm_bindgen (method , getter , js_class = Tab)]
    #[doc = "Whether the tab is highlighted."]
    pub fn highlighted(this: &Tab) -> ::js_sys::Boolean;
    # [wasm_bindgen (method , getter , js_class = Tab)]
    #[doc = "The ID of the tab. Tab IDs are unique within a browser session. Under some circumstances a tab may not be assigned an ID; for example, when querying foreign tabs using the $(ref:sessions) API, in which case a session ID may be present. Tab ID can also be set to <code>chrome.tabs.TAB_ID_NONE</code> for apps and devtools windows."]
    pub fn id(this: &Tab) -> Option<::js_sys::Number>;
    # [wasm_bindgen (method , getter , js_class = Tab)]
    #[doc = "Whether the tab is in an incognito window."]
    pub fn incognito(this: &Tab) -> ::js_sys::Boolean;
    # [wasm_bindgen (method , getter , js_class = Tab)]
    #[doc = "The zero-based index of the tab within its window."]
    pub fn index(this: &Tab) -> ::js_sys::Number;
    # [wasm_bindgen (method , getter , js_class = Tab)]
    #[doc = "The last time the tab was accessed as the number of milliseconds since epoch."]
    pub fn lastAccessed(this: &Tab) -> Option<::js_sys::Number>;
    # [wasm_bindgen (method , getter , js_class = Tab)]
    #[doc = "The tab's muted state and the reason for the last state change."]
    pub fn mutedInfo(this: &Tab) -> Option<i32>;
    # [wasm_bindgen (method , getter , js_class = Tab)]
    #[doc = "The ID of the tab that opened this tab, if any. This property is only present if the opener tab still exists."]
    pub fn openerTabId(this: &Tab) -> Option<::js_sys::Number>;
    # [wasm_bindgen (method , getter , js_class = Tab)]
    #[doc = "The URL the tab is navigating to, before it has committed. This property is only present if the extension's manifest includes the <code>\"tabs\"</code> permission and there is a pending navigation."]
    pub fn pendingUrl(this: &Tab) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = Tab)]
    #[doc = "Whether the tab is pinned."]
    pub fn pinned(this: &Tab) -> ::js_sys::Boolean;
    # [wasm_bindgen (method , getter , js_class = Tab)]
    #[doc = "Whether the tab is selected."]
    pub fn selected(this: &Tab) -> ::js_sys::Boolean;
    # [wasm_bindgen (method , getter , js_class = Tab)]
    #[doc = "The session ID used to uniquely identify a tab obtained from the $(ref:sessions) API."]
    pub fn sessionId(this: &Tab) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = Tab)]
    #[doc = "The tab's loading status."]
    pub fn status(this: &Tab) -> Option<i32>;
    # [wasm_bindgen (method , getter , js_class = Tab)]
    #[doc = "The title of the tab. This property is only present if the extension's manifest includes the <code>\"tabs\"</code> permission."]
    pub fn title(this: &Tab) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = Tab)]
    #[doc = "The last committed URL of the main frame of the tab. This property is only present if the extension's manifest includes the <code>\"tabs\"</code> permission and may be an empty string if the tab has not yet committed. See also $(ref:Tab.pendingUrl)."]
    pub fn url(this: &Tab) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = Tab)]
    #[doc = "The width of the tab in pixels."]
    pub fn width(this: &Tab) -> Option<::js_sys::Number>;
    # [wasm_bindgen (method , getter , js_class = Tab)]
    #[doc = "The ID of the window that contains the tab."]
    pub fn windowId(this: &Tab) -> ::js_sys::Number;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "ZoomSettingsMode" , typescript_type = "ZoomSettingsMode")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Defines how zoom changes are handled, i.e., which entity is responsible for the actual scaling of the page; defaults to <code>automatic</code>."]
    pub type ZoomSettingsMode;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "ZoomSettingsScope" , typescript_type = "ZoomSettingsScope")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Defines whether zoom changes persist for the page's origin, or only take effect in this tab; defaults to <code>per-origin</code> when in <code>automatic</code> mode, and <code>per-tab</code> otherwise."]
    pub type ZoomSettingsScope;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "ZoomSettings" , typescript_type = "ZoomSettings")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Defines how zoom changes in a tab are handled and at what scope."]
    pub type ZoomSettings;
    # [wasm_bindgen (method , getter , js_class = ZoomSettings)]
    #[doc = "Used to return the default zoom level for the current tab in calls to tabs.getZoomSettings."]
    pub fn defaultZoomFactor(this: &ZoomSettings) -> Option<::js_sys::Number>;
    # [wasm_bindgen (method , getter , js_class = ZoomSettings)]
    #[doc = "Defines how zoom changes are handled, i.e., which entity is responsible for the actual scaling of the page; defaults to <code>automatic</code>."]
    pub fn mode(this: &ZoomSettings) -> Option<i32>;
    # [wasm_bindgen (method , getter , js_class = ZoomSettings)]
    #[doc = "Defines whether zoom changes persist for the page's origin, or only take effect in this tab; defaults to <code>per-origin</code> when in <code>automatic</code> mode, and <code>per-tab</code> otherwise."]
    pub fn scope(this: &ZoomSettings) -> Option<i32>;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "WindowType" , typescript_type = "WindowType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The type of window."]
    pub type WindowType;
}
