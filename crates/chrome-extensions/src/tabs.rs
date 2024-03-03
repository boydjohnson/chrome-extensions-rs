#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>chrome.tabs</code> API to interact with the browser's tab system. You can use this API to create, modify, and rearrange tabs in the browser."]
# [wasm_bindgen (js_namespace = chrome)]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "tabs.TabStatus" , typescript_type = "tabs.TabStatus")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The tab's loading status."]
    pub type TabStatus;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "tabs.MutedInfoReason" , typescript_type = "tabs.MutedInfoReason")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "An event that caused a muted state change."]
    pub type MutedInfoReason;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "tabs.MutedInfo" , typescript_type = "tabs.MutedInfo")]
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
    pub fn reason(this: &MutedInfo) -> Option<MutedInfoReason>;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "tabs.Tab" , typescript_type = "tabs.Tab")]
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
    pub fn mutedInfo(this: &Tab) -> Option<MutedInfo>;
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
    pub fn status(this: &Tab) -> Option<TabStatus>;
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
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "tabs.ZoomSettingsMode" , typescript_type = "tabs.ZoomSettingsMode")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Defines how zoom changes are handled, i.e., which entity is responsible for the actual scaling of the page; defaults to <code>automatic</code>."]
    pub type ZoomSettingsMode;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "tabs.ZoomSettingsScope" , typescript_type = "tabs.ZoomSettingsScope")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Defines whether zoom changes persist for the page's origin, or only take effect in this tab; defaults to <code>per-origin</code> when in <code>automatic</code> mode, and <code>per-tab</code> otherwise."]
    pub type ZoomSettingsScope;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "tabs.ZoomSettings" , typescript_type = "tabs.ZoomSettings")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Defines how zoom changes in a tab are handled and at what scope."]
    pub type ZoomSettings;
    # [wasm_bindgen (method , getter , js_class = ZoomSettings)]
    #[doc = "Used to return the default zoom level for the current tab in calls to tabs.getZoomSettings."]
    pub fn defaultZoomFactor(this: &ZoomSettings) -> Option<::js_sys::Number>;
    # [wasm_bindgen (method , getter , js_class = ZoomSettings)]
    #[doc = "Defines how zoom changes are handled, i.e., which entity is responsible for the actual scaling of the page; defaults to <code>automatic</code>."]
    pub fn mode(this: &ZoomSettings) -> Option<ZoomSettingsMode>;
    # [wasm_bindgen (method , getter , js_class = ZoomSettings)]
    #[doc = "Defines whether zoom changes persist for the page's origin, or only take effect in this tab; defaults to <code>per-origin</code> when in <code>automatic</code> mode, and <code>per-tab</code> otherwise."]
    pub fn scope(this: &ZoomSettings) -> Option<ZoomSettingsScope>;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "tabs.WindowType" , typescript_type = "tabs.WindowType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The type of window."]
    pub type WindowType;
    #[doc = "Retrieves details about the specified tab."]
    #[wasm_bindgen(js_name = "tabs.get", catch)]
    pub async fn get(
        tabId: ::js_sys::Number,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Retrieves details about the specified tab."]
    #[wasm_bindgen(js_name = "tabs.get")]
    pub fn get_callback(tabId: ::js_sys::Number, callback: &::js_sys::Function);
    #[doc = "Gets the tab that this script call is being made from. Returns <code>undefined</code> if called from a non-tab context (for example, a background page or popup view)."]
    #[wasm_bindgen(js_name = "tabs.getCurrent", catch)]
    pub async fn get_current() -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Gets the tab that this script call is being made from. Returns <code>undefined</code> if called from a non-tab context (for example, a background page or popup view)."]
    #[wasm_bindgen(js_name = "tabs.getCurrent")]
    pub fn get_current_callback(callback: &::js_sys::Function);
    #[doc = "Connects to the content script(s) in the specified tab. The $(ref:runtime.onConnect) event is fired in each content script running in the specified tab for the current extension. For more details, see <a href='messaging'>Content Script Messaging</a>."]
    #[wasm_bindgen(js_name = "tabs.connect")]
    pub fn connect(
        tabId: ::js_sys::Number,
        connectInfo: Option<::js_sys::Object>,
    ) -> crate::runtime::Port;
    #[doc = "Sends a single request to the content script(s) in the specified tab, with an optional callback to run when a response is sent back.  The $(ref:extension.onRequest) event is fired in each content script running in the specified tab for the current extension."]
    #[wasm_bindgen(js_name = "tabs.sendRequest", catch)]
    pub async fn send_request(
        tabId: ::js_sys::Number,
        request: ::wasm_bindgen::JsValue,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Sends a single request to the content script(s) in the specified tab, with an optional callback to run when a response is sent back.  The $(ref:extension.onRequest) event is fired in each content script running in the specified tab for the current extension."]
    #[wasm_bindgen(js_name = "tabs.sendRequest")]
    pub fn send_request_callback(
        tabId: ::js_sys::Number,
        request: ::wasm_bindgen::JsValue,
        callback: &::js_sys::Function,
    );
    #[doc = "Sends a single message to the content script(s) in the specified tab, with an optional callback to run when a response is sent back.  The $(ref:runtime.onMessage) event is fired in each content script running in the specified tab for the current extension."]
    #[wasm_bindgen(js_name = "tabs.sendMessage", catch)]
    pub async fn send_message(
        tabId: ::js_sys::Number,
        message: ::wasm_bindgen::JsValue,
        options: Option<::js_sys::Object>,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Sends a single message to the content script(s) in the specified tab, with an optional callback to run when a response is sent back.  The $(ref:runtime.onMessage) event is fired in each content script running in the specified tab for the current extension."]
    #[wasm_bindgen(js_name = "tabs.sendMessage")]
    pub fn send_message_callback(
        tabId: ::js_sys::Number,
        message: ::wasm_bindgen::JsValue,
        options: Option<::js_sys::Object>,
        callback: &::js_sys::Function,
    );
    #[doc = "Gets the tab that is selected in the specified window."]
    #[wasm_bindgen(js_name = "tabs.getSelected", catch)]
    pub async fn get_selected(
        windowId: Option<::js_sys::Number>,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Gets the tab that is selected in the specified window."]
    #[wasm_bindgen(js_name = "tabs.getSelected")]
    pub fn get_selected_callback(windowId: Option<::js_sys::Number>, callback: &::js_sys::Function);
    #[doc = "Gets details about all tabs in the specified window."]
    #[wasm_bindgen(js_name = "tabs.getAllInWindow", catch)]
    pub async fn get_all_in_window(
        windowId: Option<::js_sys::Number>,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Gets details about all tabs in the specified window."]
    #[wasm_bindgen(js_name = "tabs.getAllInWindow")]
    pub fn get_all_in_window_callback(
        windowId: Option<::js_sys::Number>,
        callback: &::js_sys::Function,
    );
    #[doc = "Creates a new tab."]
    #[wasm_bindgen(js_name = "tabs.create", catch)]
    pub async fn create(
        createProperties: ::js_sys::Object,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Creates a new tab."]
    #[wasm_bindgen(js_name = "tabs.create")]
    pub fn create_callback(createProperties: ::js_sys::Object, callback: &::js_sys::Function);
    #[doc = "Duplicates a tab."]
    #[wasm_bindgen(js_name = "tabs.duplicate", catch)]
    pub async fn duplicate(
        tabId: ::js_sys::Number,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Duplicates a tab."]
    #[wasm_bindgen(js_name = "tabs.duplicate")]
    pub fn duplicate_callback(tabId: ::js_sys::Number, callback: &::js_sys::Function);
    #[doc = "Gets all tabs that have the specified properties, or all tabs if no properties are specified."]
    #[wasm_bindgen(js_name = "tabs.query", catch)]
    pub async fn query(
        queryInfo: ::js_sys::Object,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Gets all tabs that have the specified properties, or all tabs if no properties are specified."]
    #[wasm_bindgen(js_name = "tabs.query")]
    pub fn query_callback(queryInfo: ::js_sys::Object, callback: &::js_sys::Function);
    #[doc = "Highlights the given tabs and focuses on the first of group. Will appear to do nothing if the specified tab is currently active."]
    #[wasm_bindgen(js_name = "tabs.highlight", catch)]
    pub async fn highlight(
        highlightInfo: ::js_sys::Object,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Highlights the given tabs and focuses on the first of group. Will appear to do nothing if the specified tab is currently active."]
    #[wasm_bindgen(js_name = "tabs.highlight")]
    pub fn highlight_callback(highlightInfo: ::js_sys::Object, callback: &::js_sys::Function);
    #[doc = "Modifies the properties of a tab. Properties that are not specified in <var>updateProperties</var> are not modified."]
    #[wasm_bindgen(js_name = "tabs.update", catch)]
    pub async fn update(
        tabId: Option<::js_sys::Number>,
        updateProperties: ::js_sys::Object,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Modifies the properties of a tab. Properties that are not specified in <var>updateProperties</var> are not modified."]
    #[wasm_bindgen(js_name = "tabs.update")]
    pub fn update_callback(
        tabId: Option<::js_sys::Number>,
        updateProperties: ::js_sys::Object,
        callback: &::js_sys::Function,
    );
    #[doc = "Moves one or more tabs to a new position within its window, or to a new window. Note that tabs can only be moved to and from normal (window.type === \"normal\") windows."]
    #[wasm_bindgen(js_name = "tabs.move", catch)]
    pub async fn move_(
        tabIds: ::wasm_bindgen::JsValue,
        moveProperties: ::js_sys::Object,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Moves one or more tabs to a new position within its window, or to a new window. Note that tabs can only be moved to and from normal (window.type === \"normal\") windows."]
    #[wasm_bindgen(js_name = "tabs.move")]
    pub fn move_callback(
        tabIds: ::wasm_bindgen::JsValue,
        moveProperties: ::js_sys::Object,
        callback: &::js_sys::Function,
    );
    #[doc = "Reload a tab."]
    #[wasm_bindgen(js_name = "tabs.reload", catch)]
    pub async fn reload(
        tabId: Option<::js_sys::Number>,
        reloadProperties: Option<::js_sys::Object>,
    ) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Reload a tab."]
    #[wasm_bindgen(js_name = "tabs.reload")]
    pub fn reload_callback(
        tabId: Option<::js_sys::Number>,
        reloadProperties: Option<::js_sys::Object>,
        callback: &::js_sys::Function,
    );
    #[doc = "Closes one or more tabs."]
    #[wasm_bindgen(js_name = "tabs.remove", catch)]
    pub async fn remove(tabIds: ::wasm_bindgen::JsValue) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Closes one or more tabs."]
    #[wasm_bindgen(js_name = "tabs.remove")]
    pub fn remove_callback(tabIds: ::wasm_bindgen::JsValue, callback: &::js_sys::Function);
    #[doc = "Adds one or more tabs to a specified group, or if no group is specified, adds the given tabs to a newly created group."]
    #[wasm_bindgen(js_name = "tabs.group", catch)]
    pub async fn group(
        options: ::js_sys::Object,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Adds one or more tabs to a specified group, or if no group is specified, adds the given tabs to a newly created group."]
    #[wasm_bindgen(js_name = "tabs.group")]
    pub fn group_callback(options: ::js_sys::Object, callback: &::js_sys::Function);
    #[doc = "Removes one or more tabs from their respective groups. If any groups become empty, they are deleted."]
    #[wasm_bindgen(js_name = "tabs.ungroup", catch)]
    pub async fn ungroup(tabIds: ::wasm_bindgen::JsValue) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Removes one or more tabs from their respective groups. If any groups become empty, they are deleted."]
    #[wasm_bindgen(js_name = "tabs.ungroup")]
    pub fn ungroup_callback(tabIds: ::wasm_bindgen::JsValue, callback: &::js_sys::Function);
    #[doc = "Detects the primary language of the content in a tab."]
    #[wasm_bindgen(js_name = "tabs.detectLanguage", catch)]
    pub async fn detect_language(
        tabId: Option<::js_sys::Number>,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Detects the primary language of the content in a tab."]
    #[wasm_bindgen(js_name = "tabs.detectLanguage")]
    pub fn detect_language_callback(tabId: Option<::js_sys::Number>, callback: &::js_sys::Function);
    #[doc = "Captures the visible area of the currently active tab in the specified window. In order to call this method, the extension must have either the <a href='develop/concepts/declare-permissions'>&lt;all_urls&gt;</a> permission or the <a href='develop/concepts/activeTab'>activeTab</a> permission. In addition to sites that extensions can normally access, this method allows extensions to capture sensitive sites that are otherwise restricted, including chrome:-scheme pages, other extensions' pages, and data: URLs. These sensitive sites can only be captured with the activeTab permission. File URLs may be captured only if the extension has been granted file access."]
    #[wasm_bindgen(js_name = "tabs.captureVisibleTab", catch)]
    pub async fn capture_visible_tab(
        windowId: Option<::js_sys::Number>,
        options: Option<crate::extension_types::ImageDetails>,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Captures the visible area of the currently active tab in the specified window. In order to call this method, the extension must have either the <a href='develop/concepts/declare-permissions'>&lt;all_urls&gt;</a> permission or the <a href='develop/concepts/activeTab'>activeTab</a> permission. In addition to sites that extensions can normally access, this method allows extensions to capture sensitive sites that are otherwise restricted, including chrome:-scheme pages, other extensions' pages, and data: URLs. These sensitive sites can only be captured with the activeTab permission. File URLs may be captured only if the extension has been granted file access."]
    #[wasm_bindgen(js_name = "tabs.captureVisibleTab")]
    pub fn capture_visible_tab_callback(
        windowId: Option<::js_sys::Number>,
        options: Option<crate::extension_types::ImageDetails>,
        callback: &::js_sys::Function,
    );
    #[doc = "Injects JavaScript code into a page. For details, see the <a href='/docs/extensions/develop/concepts/content-scripts#programmatic'>programmatic injection</a> section of the content scripts doc."]
    #[wasm_bindgen(js_name = "tabs.executeScript", catch)]
    pub async fn execute_script(
        tabId: Option<::js_sys::Number>,
        details: crate::extension_types::InjectDetails,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Injects JavaScript code into a page. For details, see the <a href='/docs/extensions/develop/concepts/content-scripts#programmatic'>programmatic injection</a> section of the content scripts doc."]
    #[wasm_bindgen(js_name = "tabs.executeScript")]
    pub fn execute_script_callback(
        tabId: Option<::js_sys::Number>,
        details: crate::extension_types::InjectDetails,
        callback: &::js_sys::Function,
    );
    #[doc = "Injects CSS into a page. Styles inserted with this method can be removed with $(ref:scripting.removeCSS). For details, see the <a href='/docs/extensions/develop/concepts/content-scripts#programmatic'>programmatic injection</a> section of the content scripts doc."]
    #[wasm_bindgen(js_name = "tabs.insertCSS", catch)]
    pub async fn insert_css(
        tabId: Option<::js_sys::Number>,
        details: crate::extension_types::InjectDetails,
    ) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Injects CSS into a page. Styles inserted with this method can be removed with $(ref:scripting.removeCSS). For details, see the <a href='/docs/extensions/develop/concepts/content-scripts#programmatic'>programmatic injection</a> section of the content scripts doc."]
    #[wasm_bindgen(js_name = "tabs.insertCSS")]
    pub fn insert_css_callback(
        tabId: Option<::js_sys::Number>,
        details: crate::extension_types::InjectDetails,
        callback: &::js_sys::Function,
    );
    #[doc = "Removes from a page CSS that was previously injected by a call to $(ref:scripting.insertCSS)."]
    #[wasm_bindgen(js_name = "tabs.removeCSS", catch)]
    pub async fn remove_css(
        tabId: Option<::js_sys::Number>,
        details: crate::extension_types::DeleteInjectionDetails,
    ) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Removes from a page CSS that was previously injected by a call to $(ref:scripting.insertCSS)."]
    #[wasm_bindgen(js_name = "tabs.removeCSS")]
    pub fn remove_css_callback(
        tabId: Option<::js_sys::Number>,
        details: crate::extension_types::DeleteInjectionDetails,
        callback: &::js_sys::Function,
    );
    #[doc = "Zooms a specified tab."]
    #[wasm_bindgen(js_name = "tabs.setZoom", catch)]
    pub async fn set_zoom(
        tabId: Option<::js_sys::Number>,
        zoomFactor: ::js_sys::Number,
    ) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Zooms a specified tab."]
    #[wasm_bindgen(js_name = "tabs.setZoom")]
    pub fn set_zoom_callback(
        tabId: Option<::js_sys::Number>,
        zoomFactor: ::js_sys::Number,
        callback: &::js_sys::Function,
    );
    #[doc = "Gets the current zoom factor of a specified tab."]
    #[wasm_bindgen(js_name = "tabs.getZoom", catch)]
    pub async fn get_zoom(
        tabId: Option<::js_sys::Number>,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Gets the current zoom factor of a specified tab."]
    #[wasm_bindgen(js_name = "tabs.getZoom")]
    pub fn get_zoom_callback(tabId: Option<::js_sys::Number>, callback: &::js_sys::Function);
    #[doc = "Sets the zoom settings for a specified tab, which define how zoom changes are handled. These settings are reset to defaults upon navigating the tab."]
    #[wasm_bindgen(js_name = "tabs.setZoomSettings", catch)]
    pub async fn set_zoom_settings(
        tabId: Option<::js_sys::Number>,
        zoomSettings: ZoomSettings,
    ) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Sets the zoom settings for a specified tab, which define how zoom changes are handled. These settings are reset to defaults upon navigating the tab."]
    #[wasm_bindgen(js_name = "tabs.setZoomSettings")]
    pub fn set_zoom_settings_callback(
        tabId: Option<::js_sys::Number>,
        zoomSettings: ZoomSettings,
        callback: &::js_sys::Function,
    );
    #[doc = "Gets the current zoom settings of a specified tab."]
    #[wasm_bindgen(js_name = "tabs.getZoomSettings", catch)]
    pub async fn get_zoom_settings(
        tabId: Option<::js_sys::Number>,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Gets the current zoom settings of a specified tab."]
    #[wasm_bindgen(js_name = "tabs.getZoomSettings")]
    pub fn get_zoom_settings_callback(
        tabId: Option<::js_sys::Number>,
        callback: &::js_sys::Function,
    );
    #[doc = "Discards a tab from memory. Discarded tabs are still visible on the tab strip and are reloaded when activated."]
    #[wasm_bindgen(js_name = "tabs.discard", catch)]
    pub async fn discard(
        tabId: Option<::js_sys::Number>,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Discards a tab from memory. Discarded tabs are still visible on the tab strip and are reloaded when activated."]
    #[wasm_bindgen(js_name = "tabs.discard")]
    pub fn discard_callback(tabId: Option<::js_sys::Number>, callback: &::js_sys::Function);
    #[doc = "Go foward to the next page, if one is available."]
    #[wasm_bindgen(js_name = "tabs.goForward", catch)]
    pub async fn go_forward(tabId: Option<::js_sys::Number>)
        -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Go foward to the next page, if one is available."]
    #[wasm_bindgen(js_name = "tabs.goForward")]
    pub fn go_forward_callback(tabId: Option<::js_sys::Number>, callback: &::js_sys::Function);
    #[doc = "Go back to the previous page, if one is available."]
    #[wasm_bindgen(js_name = "tabs.goBack", catch)]
    pub async fn go_back(tabId: Option<::js_sys::Number>) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Go back to the previous page, if one is available."]
    #[wasm_bindgen(js_name = "tabs.goBack")]
    pub fn go_back_callback(tabId: Option<::js_sys::Number>, callback: &::js_sys::Function);
}
#[wasm_bindgen]
pub async fn tabs_get(
    tabId: ::js_sys::Number,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    get(tabId).await
}
#[wasm_bindgen]
pub fn tabs_get_callback(tabId: ::js_sys::Number, callback: &::js_sys::Function) {
    get_callback(tabId, callback);
}
#[wasm_bindgen]
pub async fn tabs_get_current() -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    get_current().await
}
#[wasm_bindgen]
pub fn tabs_get_current_callback(callback: &::js_sys::Function) {
    get_current_callback(callback);
}
#[wasm_bindgen]
pub fn tabs_connect(
    tabId: ::js_sys::Number,
    connectInfo: Option<::js_sys::Object>,
) -> crate::runtime::Port {
    connect(tabId, connectInfo)
}
#[wasm_bindgen]
pub async fn tabs_send_request(
    tabId: ::js_sys::Number,
    request: ::wasm_bindgen::JsValue,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    send_request(tabId, request).await
}
#[wasm_bindgen]
pub fn tabs_send_request_callback(
    tabId: ::js_sys::Number,
    request: ::wasm_bindgen::JsValue,
    callback: &::js_sys::Function,
) {
    send_request_callback(tabId, request, callback);
}
#[wasm_bindgen]
pub async fn tabs_send_message(
    tabId: ::js_sys::Number,
    message: ::wasm_bindgen::JsValue,
    options: Option<::js_sys::Object>,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    send_message(tabId, message, options).await
}
#[wasm_bindgen]
pub fn tabs_send_message_callback(
    tabId: ::js_sys::Number,
    message: ::wasm_bindgen::JsValue,
    options: Option<::js_sys::Object>,
    callback: &::js_sys::Function,
) {
    send_message_callback(tabId, message, options, callback);
}
#[wasm_bindgen]
pub async fn tabs_get_selected(
    windowId: Option<::js_sys::Number>,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    get_selected(windowId).await
}
#[wasm_bindgen]
pub fn tabs_get_selected_callback(
    windowId: Option<::js_sys::Number>,
    callback: &::js_sys::Function,
) {
    get_selected_callback(windowId, callback);
}
#[wasm_bindgen]
pub async fn tabs_get_all_in_window(
    windowId: Option<::js_sys::Number>,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    get_all_in_window(windowId).await
}
#[wasm_bindgen]
pub fn tabs_get_all_in_window_callback(
    windowId: Option<::js_sys::Number>,
    callback: &::js_sys::Function,
) {
    get_all_in_window_callback(windowId, callback);
}
#[wasm_bindgen]
pub async fn tabs_create(
    createProperties: ::js_sys::Object,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    create(createProperties).await
}
#[wasm_bindgen]
pub fn tabs_create_callback(createProperties: ::js_sys::Object, callback: &::js_sys::Function) {
    create_callback(createProperties, callback);
}
#[wasm_bindgen]
pub async fn tabs_duplicate(
    tabId: ::js_sys::Number,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    duplicate(tabId).await
}
#[wasm_bindgen]
pub fn tabs_duplicate_callback(tabId: ::js_sys::Number, callback: &::js_sys::Function) {
    duplicate_callback(tabId, callback);
}
#[wasm_bindgen]
pub async fn tabs_query(
    queryInfo: ::js_sys::Object,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    query(queryInfo).await
}
#[wasm_bindgen]
pub fn tabs_query_callback(queryInfo: ::js_sys::Object, callback: &::js_sys::Function) {
    query_callback(queryInfo, callback);
}
#[wasm_bindgen]
pub async fn tabs_highlight(
    highlightInfo: ::js_sys::Object,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    highlight(highlightInfo).await
}
#[wasm_bindgen]
pub fn tabs_highlight_callback(highlightInfo: ::js_sys::Object, callback: &::js_sys::Function) {
    highlight_callback(highlightInfo, callback);
}
#[wasm_bindgen]
pub async fn tabs_update(
    tabId: Option<::js_sys::Number>,
    updateProperties: ::js_sys::Object,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    update(tabId, updateProperties).await
}
#[wasm_bindgen]
pub fn tabs_update_callback(
    tabId: Option<::js_sys::Number>,
    updateProperties: ::js_sys::Object,
    callback: &::js_sys::Function,
) {
    update_callback(tabId, updateProperties, callback);
}
#[wasm_bindgen]
pub async fn tabs_move(
    tabIds: ::wasm_bindgen::JsValue,
    moveProperties: ::js_sys::Object,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    move_(tabIds, moveProperties).await
}
#[wasm_bindgen]
pub fn tabs_move_callback(
    tabIds: ::wasm_bindgen::JsValue,
    moveProperties: ::js_sys::Object,
    callback: &::js_sys::Function,
) {
    move_callback(tabIds, moveProperties, callback);
}
#[wasm_bindgen]
pub async fn tabs_reload(
    tabId: Option<::js_sys::Number>,
    reloadProperties: Option<::js_sys::Object>,
) -> Result<(), ::wasm_bindgen::JsValue> {
    reload(tabId, reloadProperties).await
}
#[wasm_bindgen]
pub fn tabs_reload_callback(
    tabId: Option<::js_sys::Number>,
    reloadProperties: Option<::js_sys::Object>,
    callback: &::js_sys::Function,
) {
    reload_callback(tabId, reloadProperties, callback);
}
#[wasm_bindgen]
pub async fn tabs_remove(tabIds: ::wasm_bindgen::JsValue) -> Result<(), ::wasm_bindgen::JsValue> {
    remove(tabIds).await
}
#[wasm_bindgen]
pub fn tabs_remove_callback(tabIds: ::wasm_bindgen::JsValue, callback: &::js_sys::Function) {
    remove_callback(tabIds, callback);
}
#[wasm_bindgen]
pub async fn tabs_group(
    options: ::js_sys::Object,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    group(options).await
}
#[wasm_bindgen]
pub fn tabs_group_callback(options: ::js_sys::Object, callback: &::js_sys::Function) {
    group_callback(options, callback);
}
#[wasm_bindgen]
pub async fn tabs_ungroup(tabIds: ::wasm_bindgen::JsValue) -> Result<(), ::wasm_bindgen::JsValue> {
    ungroup(tabIds).await
}
#[wasm_bindgen]
pub fn tabs_ungroup_callback(tabIds: ::wasm_bindgen::JsValue, callback: &::js_sys::Function) {
    ungroup_callback(tabIds, callback);
}
#[wasm_bindgen]
pub async fn tabs_detect_language(
    tabId: Option<::js_sys::Number>,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    detect_language(tabId).await
}
#[wasm_bindgen]
pub fn tabs_detect_language_callback(
    tabId: Option<::js_sys::Number>,
    callback: &::js_sys::Function,
) {
    detect_language_callback(tabId, callback);
}
#[wasm_bindgen]
pub async fn tabs_capture_visible_tab(
    windowId: Option<::js_sys::Number>,
    options: Option<crate::extension_types::ImageDetails>,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    capture_visible_tab(windowId, options).await
}
#[wasm_bindgen]
pub fn tabs_capture_visible_tab_callback(
    windowId: Option<::js_sys::Number>,
    options: Option<crate::extension_types::ImageDetails>,
    callback: &::js_sys::Function,
) {
    capture_visible_tab_callback(windowId, options, callback);
}
#[wasm_bindgen]
pub async fn tabs_execute_script(
    tabId: Option<::js_sys::Number>,
    details: crate::extension_types::InjectDetails,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    execute_script(tabId, details).await
}
#[wasm_bindgen]
pub fn tabs_execute_script_callback(
    tabId: Option<::js_sys::Number>,
    details: crate::extension_types::InjectDetails,
    callback: &::js_sys::Function,
) {
    execute_script_callback(tabId, details, callback);
}
#[wasm_bindgen]
pub async fn tabs_insert_css(
    tabId: Option<::js_sys::Number>,
    details: crate::extension_types::InjectDetails,
) -> Result<(), ::wasm_bindgen::JsValue> {
    insert_css(tabId, details).await
}
#[wasm_bindgen]
pub fn tabs_insert_css_callback(
    tabId: Option<::js_sys::Number>,
    details: crate::extension_types::InjectDetails,
    callback: &::js_sys::Function,
) {
    insert_css_callback(tabId, details, callback);
}
#[wasm_bindgen]
pub async fn tabs_remove_css(
    tabId: Option<::js_sys::Number>,
    details: crate::extension_types::DeleteInjectionDetails,
) -> Result<(), ::wasm_bindgen::JsValue> {
    remove_css(tabId, details).await
}
#[wasm_bindgen]
pub fn tabs_remove_css_callback(
    tabId: Option<::js_sys::Number>,
    details: crate::extension_types::DeleteInjectionDetails,
    callback: &::js_sys::Function,
) {
    remove_css_callback(tabId, details, callback);
}
#[wasm_bindgen]
pub async fn tabs_set_zoom(
    tabId: Option<::js_sys::Number>,
    zoomFactor: ::js_sys::Number,
) -> Result<(), ::wasm_bindgen::JsValue> {
    set_zoom(tabId, zoomFactor).await
}
#[wasm_bindgen]
pub fn tabs_set_zoom_callback(
    tabId: Option<::js_sys::Number>,
    zoomFactor: ::js_sys::Number,
    callback: &::js_sys::Function,
) {
    set_zoom_callback(tabId, zoomFactor, callback);
}
#[wasm_bindgen]
pub async fn tabs_get_zoom(
    tabId: Option<::js_sys::Number>,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    get_zoom(tabId).await
}
#[wasm_bindgen]
pub fn tabs_get_zoom_callback(tabId: Option<::js_sys::Number>, callback: &::js_sys::Function) {
    get_zoom_callback(tabId, callback);
}
#[wasm_bindgen]
pub async fn tabs_set_zoom_settings(
    tabId: Option<::js_sys::Number>,
    zoomSettings: ZoomSettings,
) -> Result<(), ::wasm_bindgen::JsValue> {
    set_zoom_settings(tabId, zoomSettings).await
}
#[wasm_bindgen]
pub fn tabs_set_zoom_settings_callback(
    tabId: Option<::js_sys::Number>,
    zoomSettings: ZoomSettings,
    callback: &::js_sys::Function,
) {
    set_zoom_settings_callback(tabId, zoomSettings, callback);
}
#[wasm_bindgen]
pub async fn tabs_get_zoom_settings(
    tabId: Option<::js_sys::Number>,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    get_zoom_settings(tabId).await
}
#[wasm_bindgen]
pub fn tabs_get_zoom_settings_callback(
    tabId: Option<::js_sys::Number>,
    callback: &::js_sys::Function,
) {
    get_zoom_settings_callback(tabId, callback);
}
#[wasm_bindgen]
pub async fn tabs_discard(
    tabId: Option<::js_sys::Number>,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    discard(tabId).await
}
#[wasm_bindgen]
pub fn tabs_discard_callback(tabId: Option<::js_sys::Number>, callback: &::js_sys::Function) {
    discard_callback(tabId, callback);
}
#[wasm_bindgen]
pub async fn tabs_go_forward(
    tabId: Option<::js_sys::Number>,
) -> Result<(), ::wasm_bindgen::JsValue> {
    go_forward(tabId).await
}
#[wasm_bindgen]
pub fn tabs_go_forward_callback(tabId: Option<::js_sys::Number>, callback: &::js_sys::Function) {
    go_forward_callback(tabId, callback);
}
#[wasm_bindgen]
pub async fn tabs_go_back(tabId: Option<::js_sys::Number>) -> Result<(), ::wasm_bindgen::JsValue> {
    go_back(tabId).await
}
#[wasm_bindgen]
pub fn tabs_go_back_callback(tabId: Option<::js_sys::Number>, callback: &::js_sys::Function) {
    go_back_callback(tabId, callback);
}
