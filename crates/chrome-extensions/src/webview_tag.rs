#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>webview</code> tag to actively load live content from the web over the network and embed it in your Chrome App. Your app can control the appearance of the <code>webview</code> and interact with the web content, initiate navigations in an embedded web page, react to error events that happen within it, and more (see <a href=\"#usage\">Usage</a>)."]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "ClearDataOptions" , typescript_type = "ClearDataOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Options that determine what data should be cleared by <code>clearData</code>."]
    pub type ClearDataOptions;
    # [wasm_bindgen (method , getter , js_class = ClearDataOptions)]
    #[doc = "Clear data accumulated on or after this date, represented in milliseconds since the epoch (accessible via the getTime method of the JavaScript <code>Date</code> object). If absent, defaults to <code>0</code> (which would remove all browsing data)."]
    pub fn since(this: &ClearDataOptions) -> Option<::js_sys::Number>;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "ClearDataTypeSet" , typescript_type = "ClearDataTypeSet")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "A set of data types. Missing properties are interpreted as <code>false</code>."]
    pub type ClearDataTypeSet;
    # [wasm_bindgen (method , getter , js_class = ClearDataTypeSet)]
    #[doc = "Websites' appcaches."]
    pub fn appcache(this: &ClearDataTypeSet) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = ClearDataTypeSet)]
    #[doc = "Since Chrome 43.<br>The browser's cache. Note: when removing data, this clears the entire cache; it is not limited to the range you specify."]
    pub fn cache(this: &ClearDataTypeSet) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = ClearDataTypeSet)]
    #[doc = "The partition's cookies."]
    pub fn cookies(this: &ClearDataTypeSet) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = ClearDataTypeSet)]
    #[doc = "Websites' filesystems."]
    pub fn fileSystems(this: &ClearDataTypeSet) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = ClearDataTypeSet)]
    #[doc = "Websites' IndexedDB data."]
    pub fn indexedDB(this: &ClearDataTypeSet) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = ClearDataTypeSet)]
    #[doc = "Websites' local storage data."]
    pub fn localStorage(this: &ClearDataTypeSet) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = ClearDataTypeSet)]
    #[doc = "The partition's persistent cookies."]
    pub fn persistentCookies(this: &ClearDataTypeSet) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = ClearDataTypeSet)]
    #[doc = "The partition's session cookies."]
    pub fn sessionCookies(this: &ClearDataTypeSet) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = ClearDataTypeSet)]
    #[doc = "Websites' WebSQL data."]
    pub fn webSQL(this: &ClearDataTypeSet) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "ContextType" , typescript_type = "ContextType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The different contexts a menu can appear in. Specifying 'all' is equivalent to the combination of all other contexts."]
    pub type ContextType;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "InjectDetails" , typescript_type = "InjectDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Details of the script or CSS to inject. Either the code or the file property must be set, but both may not be set at the same time."]
    pub type InjectDetails;
    # [wasm_bindgen (method , getter , js_class = InjectDetails)]
    #[doc = "JavaScript or CSS code to inject.<br><br><b>Warning:</b><br>Be careful using the <code>code</code> parameter. Incorrect use of it may open your app to <a href=\"https://en.wikipedia.org/wiki/Cross-site_scripting\">cross site scripting</a> attacks."]
    pub fn code(this: &InjectDetails) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = InjectDetails)]
    #[doc = "JavaScript or CSS file to inject."]
    pub fn file(this: &InjectDetails) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "InjectionItems" , typescript_type = "InjectionItems")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The type of injection item: code or a set of files."]
    pub type InjectionItems;
    # [wasm_bindgen (method , getter , js_class = InjectionItems)]
    #[doc = "JavaScript code or CSS to be injected into matching pages."]
    pub fn code(this: &InjectionItems) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = InjectionItems)]
    #[doc = "The list of JavaScript or CSS files to be injected into matching pages. These are injected in the order they appear in this array."]
    pub fn files(this: &InjectionItems) -> Option<::js_sys::Array>;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "ContentScriptDetails" , typescript_type = "ContentScriptDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Details of the content script to inject. Refer to the <a href='/extensions/content_scripts'>content scripts</a> documentation for more details."]
    pub type ContentScriptDetails;
    # [wasm_bindgen (method , getter , js_class = ContentScriptDetails)]
    #[doc = "If <code>all_frames</code> is <code>true</code>, this implies that the JavaScript or CSS should be injected into all frames of current page. By default, <code>all_frames</code> is <code>false</code> and the JavaScript or CSS is only injected into the top frame."]
    pub fn all_frames(this: &ContentScriptDetails) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = ContentScriptDetails)]
    #[doc = "The CSS code or a list of CSS files to be injected into matching pages. These are injected in the order they appear, before any DOM is constructed or displayed for the page."]
    pub fn css(this: &ContentScriptDetails) -> Option<InjectionItems>;
    # [wasm_bindgen (method , getter , js_class = ContentScriptDetails)]
    #[doc = "Applied after matches to exclude URLs that match this glob. Intended to emulate the @exclude Greasemonkey keyword."]
    pub fn exclude_globs(this: &ContentScriptDetails) -> Option<::js_sys::Array>;
    # [wasm_bindgen (method , getter , js_class = ContentScriptDetails)]
    #[doc = "Excludes pages that this content script would otherwise be injected into."]
    pub fn exclude_matches(this: &ContentScriptDetails) -> Option<::js_sys::Array>;
    # [wasm_bindgen (method , getter , js_class = ContentScriptDetails)]
    #[doc = "Applied after matches to include only those URLs that also match this glob. Intended to emulate the @include Greasemonkey keyword."]
    pub fn include_globs(this: &ContentScriptDetails) -> Option<::js_sys::Array>;
    # [wasm_bindgen (method , getter , js_class = ContentScriptDetails)]
    #[doc = "The JavaScript code or a list of JavaScript files to be injected into matching pages. These are injected in the order they appear."]
    pub fn js(this: &ContentScriptDetails) -> Option<InjectionItems>;
    # [wasm_bindgen (method , getter , js_class = ContentScriptDetails)]
    #[doc = "Whether to insert the content script on about:blank and about:srcdoc. Content scripts will only be injected on pages when their inherit URL is matched by one of the declared patterns in the matches field. The inherit URL is the URL of the document that created the frame or window. Content scripts cannot be inserted in sandboxed frames."]
    pub fn match_about_blank(this: &ContentScriptDetails) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = ContentScriptDetails)]
    #[doc = "Specifies which pages this content script will be injected into."]
    pub fn matches(this: &ContentScriptDetails) -> ::js_sys::Array;
    # [wasm_bindgen (method , getter , js_class = ContentScriptDetails)]
    #[doc = "The name of the content script to inject."]
    pub fn name(this: &ContentScriptDetails) -> ::js_sys::JsString;
    # [wasm_bindgen (method , getter , js_class = ContentScriptDetails)]
    #[doc = "The soonest that the JavaScript or CSS will be injected into the tab. Defaults to \"document_idle\"."]
    pub fn run_at(this: &ContentScriptDetails) -> Option<crate::extension_types::RunAt>;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "ContextMenuCreateProperties" , typescript_type = "ContextMenuCreateProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type ContextMenuCreateProperties;
    # [wasm_bindgen (method , getter , js_class = ContextMenuCreateProperties)]
    #[doc = "The initial state of a checkbox or radio item: true for selected and false for unselected. Only one radio item can be selected at a time in a given group of radio items."]
    pub fn checked(this: &ContextMenuCreateProperties) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = ContextMenuCreateProperties)]
    #[doc = "List of contexts this menu item will appear in. Defaults to ['page'] if not specified."]
    pub fn contexts(this: &ContextMenuCreateProperties) -> Option<::js_sys::Array>;
    # [wasm_bindgen (method , getter , js_class = ContextMenuCreateProperties)]
    #[doc = "Lets you restrict the item to apply only to documents whose URL matches one of the given patterns. (This applies to frames as well.) For details on the format of a pattern, see <a href='match_patterns'>Match Patterns</a>."]
    pub fn documentUrlPatterns(this: &ContextMenuCreateProperties) -> Option<::js_sys::Array>;
    # [wasm_bindgen (method , getter , js_class = ContextMenuCreateProperties)]
    #[doc = "Whether this context menu item is enabled or disabled. Defaults to <code>true</code>."]
    pub fn enabled(this: &ContextMenuCreateProperties) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = ContextMenuCreateProperties)]
    #[doc = "The unique ID to assign to this item. Mandatory for event pages. Cannot be the same as another ID for this extension."]
    pub fn id(this: &ContextMenuCreateProperties) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = ContextMenuCreateProperties)]
    #[doc = "The ID of a parent menu item; this makes the item a child of a previously added item."]
    pub fn parentId(this: &ContextMenuCreateProperties) -> wasm_bindgen::JsValue;
    # [wasm_bindgen (method , getter , js_class = ContextMenuCreateProperties)]
    #[doc = "Similar to documentUrlPatterns, but lets you filter based on the <code>src</code> attribute of img/audio/video tags and the <code>href</code> of anchor tags."]
    pub fn targetUrlPatterns(this: &ContextMenuCreateProperties) -> Option<::js_sys::Array>;
    # [wasm_bindgen (method , getter , js_class = ContextMenuCreateProperties)]
    #[doc = "The text to be displayed in the item; this is <em>required</em> unless <code>type</code> is 'separator'. When the context is 'selection', you can use <code>%s</code> within the string to show the selected text. For example, if this parameter's value is \"Translate '%s' to Pig Latin\" and the user selects the word \"cool\", the context menu item for the selection is \"Translate 'cool' to Pig Latin\"."]
    pub fn title(this: &ContextMenuCreateProperties) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = ContextMenuCreateProperties)]
    #[doc = "The type of menu item. Defaults to 'normal' if not specified."]
    pub fn type_(this: &ContextMenuCreateProperties) -> Option<crate::context_menus::ItemType>;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "ContextMenuUpdateProperties" , typescript_type = "ContextMenuUpdateProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type ContextMenuUpdateProperties;
    # [wasm_bindgen (method , getter , js_class = ContextMenuUpdateProperties)]
    #[doc = "The state of a checkbox or radio item: true for selected and false for unselected. Only one radio item can be selected at a time in a given group of radio items."]
    pub fn checked(this: &ContextMenuUpdateProperties) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = ContextMenuUpdateProperties)]
    #[doc = "List of contexts this menu item will appear in."]
    pub fn contexts(this: &ContextMenuUpdateProperties) -> Option<::js_sys::Array>;
    # [wasm_bindgen (method , getter , js_class = ContextMenuUpdateProperties)]
    #[doc = "Lets you restrict the item to apply only to documents whose URL matches one of the given patterns. (This applies to frames as well.) For details on the format of a pattern, see <a href='match_patterns'>Match Patterns</a>."]
    pub fn documentUrlPatterns(this: &ContextMenuUpdateProperties) -> Option<::js_sys::Array>;
    # [wasm_bindgen (method , getter , js_class = ContextMenuUpdateProperties)]
    #[doc = "Whether this context menu item is enabled or disabled."]
    pub fn enabled(this: &ContextMenuUpdateProperties) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = ContextMenuUpdateProperties)]
    #[doc = "The ID of a parent menu item; this makes the item a child of a previously added item. <em>Note:</em> You cannot change an item to be a child of one of its own descendants."]
    pub fn parentId(this: &ContextMenuUpdateProperties) -> wasm_bindgen::JsValue;
    # [wasm_bindgen (method , getter , js_class = ContextMenuUpdateProperties)]
    #[doc = "Similar to documentUrlPatterns, but lets you filter based on the <code>src</code> attribute of img/audio/video tags and the <code>href</code> of anchor tags."]
    pub fn targetUrlPatterns(this: &ContextMenuUpdateProperties) -> Option<::js_sys::Array>;
    # [wasm_bindgen (method , getter , js_class = ContextMenuUpdateProperties)]
    #[doc = "The text to be displayed in the item"]
    pub fn title(this: &ContextMenuUpdateProperties) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = ContextMenuUpdateProperties)]
    #[doc = "The type of menu item."]
    pub fn type_(this: &ContextMenuUpdateProperties) -> Option<crate::context_menus::ItemType>;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "ContextMenus" , typescript_type = "ContextMenus")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type ContextMenus;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "ContentWindow" , typescript_type = "ContentWindow")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Messaging handle to a guest window."]
    pub type ContentWindow;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "DialogController" , typescript_type = "DialogController")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Interface attached to <code>dialog</code> DOM events."]
    pub type DialogController;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "FindCallbackResults" , typescript_type = "FindCallbackResults")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Contains all of the results of the find request."]
    pub type FindCallbackResults;
    # [wasm_bindgen (method , getter , js_class = FindCallbackResults)]
    #[doc = "The ordinal number of the current match."]
    pub fn activeMatchOrdinal(this: &FindCallbackResults) -> ::js_sys::Number;
    # [wasm_bindgen (method , getter , js_class = FindCallbackResults)]
    #[doc = "Indicates whether this find request was canceled."]
    pub fn canceled(this: &FindCallbackResults) -> ::js_sys::Boolean;
    # [wasm_bindgen (method , getter , js_class = FindCallbackResults)]
    #[doc = "The number of times <code>searchText</code> was matched on the page."]
    pub fn numberOfMatches(this: &FindCallbackResults) -> ::js_sys::Number;
    # [wasm_bindgen (method , getter , js_class = FindCallbackResults)]
    #[doc = "Describes a rectangle around the active match in screen coordinates."]
    pub fn selectionRect(this: &FindCallbackResults) -> SelectionRect;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "FindOptions" , typescript_type = "FindOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Options for the find request."]
    pub type FindOptions;
    # [wasm_bindgen (method , getter , js_class = FindOptions)]
    #[doc = "Flag to find matches in reverse order. The default value is <code>false</code>."]
    pub fn backward(this: &FindOptions) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = FindOptions)]
    #[doc = "Flag to match with case-sensitivity. The default value is <code>false</code>."]
    pub fn matchCase(this: &FindOptions) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "NewWindow" , typescript_type = "NewWindow")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Interface attached to <code>newwindow</code> DOM events."]
    pub type NewWindow;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "MediaPermissionRequest" , typescript_type = "MediaPermissionRequest")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The type of <code>request</code> object which accompanies a <code>media</code> <a href=\"#event-permissionrequest\">permissionrequest</a></code> DOM event."]
    pub type MediaPermissionRequest;
    # [wasm_bindgen (method , getter , js_class = MediaPermissionRequest)]
    #[doc = "The URL of the frame requesting access to user media."]
    pub fn url(this: &MediaPermissionRequest) -> ::js_sys::JsString;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "GeolocationPermissionRequest" , typescript_type = "GeolocationPermissionRequest")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The type of <code>request</code> object which accompanies a <code>geolocation</code> <a href=\"#event-permissionrequest\">permissionrequest</a></code> DOM event."]
    pub type GeolocationPermissionRequest;
    # [wasm_bindgen (method , getter , js_class = GeolocationPermissionRequest)]
    #[doc = "The URL of the frame requesting access to geolocation data."]
    pub fn url(this: &GeolocationPermissionRequest) -> ::js_sys::JsString;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "PointerLockPermissionRequest" , typescript_type = "PointerLockPermissionRequest")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The type of <code>request</code> object which accompanies a <code>pointerLock</code> <a href=\"#event-permissionrequest\">permissionrequest</a></code> DOM event."]
    pub type PointerLockPermissionRequest;
    # [wasm_bindgen (method , getter , js_class = PointerLockPermissionRequest)]
    #[doc = "Whether or not the requesting frame was the most recent client to hold pointer lock."]
    pub fn lastUnlockedBySelf(this: &PointerLockPermissionRequest) -> ::js_sys::Boolean;
    # [wasm_bindgen (method , getter , js_class = PointerLockPermissionRequest)]
    #[doc = "The URL of the frame requesting pointer lock."]
    pub fn url(this: &PointerLockPermissionRequest) -> ::js_sys::JsString;
    # [wasm_bindgen (method , getter , js_class = PointerLockPermissionRequest)]
    #[doc = "Whether or not pointer lock was requested as a result of a user input gesture."]
    pub fn userGesture(this: &PointerLockPermissionRequest) -> ::js_sys::Boolean;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "DownloadPermissionRequest" , typescript_type = "DownloadPermissionRequest")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The type of <code>request</code> object which accompanies a <code>download</code> <a href=\"#event-permissionrequest\">permissionrequest</a></code> DOM event."]
    pub type DownloadPermissionRequest;
    # [wasm_bindgen (method , getter , js_class = DownloadPermissionRequest)]
    #[doc = "The HTTP request type (e.g. <code>GET</code>) associated with the download request."]
    pub fn requestMethod(this: &DownloadPermissionRequest) -> ::js_sys::JsString;
    # [wasm_bindgen (method , getter , js_class = DownloadPermissionRequest)]
    #[doc = "The requested download URL."]
    pub fn url(this: &DownloadPermissionRequest) -> ::js_sys::JsString;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "FileSystemPermissionRequest" , typescript_type = "FileSystemPermissionRequest")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The type of <code>request</code> object which accompanies a <code>filesystem</code> <a href=\"#event-permissionrequest\">permissionrequest</a></code> DOM event."]
    pub type FileSystemPermissionRequest;
    # [wasm_bindgen (method , getter , js_class = FileSystemPermissionRequest)]
    #[doc = "The URL of the frame requesting access to local file system."]
    pub fn url(this: &FileSystemPermissionRequest) -> ::js_sys::JsString;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "FullscreenPermissionRequest" , typescript_type = "FullscreenPermissionRequest")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The type of <code>request</code> object which accompanies a <code>fullscreen</code> <a href=\"#event-permissionrequest\">permissionrequest</a> DOM event.<p>"]
    pub type FullscreenPermissionRequest;
    # [wasm_bindgen (method , getter , js_class = FullscreenPermissionRequest)]
    #[doc = "The origin of the frame inside the <code>webview</code> that initiated the fullscreen request."]
    pub fn origin(this: &FullscreenPermissionRequest) -> ::js_sys::JsString;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "LoadPluginPermissionRequest" , typescript_type = "LoadPluginPermissionRequest")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The type of <code>request</code> object which accompanies a <code>loadplugin</code> <a href=\"#event-permissionrequest\">permissionrequest</a> DOM event.<p>"]
    pub type LoadPluginPermissionRequest;
    # [wasm_bindgen (method , getter , js_class = LoadPluginPermissionRequest)]
    #[doc = "The plugin's identifier string."]
    pub fn identifier(this: &LoadPluginPermissionRequest) -> ::js_sys::JsString;
    # [wasm_bindgen (method , getter , js_class = LoadPluginPermissionRequest)]
    #[doc = "The plugin's display name."]
    pub fn name(this: &LoadPluginPermissionRequest) -> ::js_sys::JsString;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "SelectionRect" , typescript_type = "SelectionRect")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "<p>Describes a rectangle in screen coordinates.</p><p>The containment semantics are array-like; that is, the coordinate <code>(left, top)</code> is considered to be contained by the rectangle, but the coordinate <code>(left + width, top)</code> is not.</p>"]
    pub type SelectionRect;
    # [wasm_bindgen (method , getter , js_class = SelectionRect)]
    #[doc = "Height of the rectangle."]
    pub fn height(this: &SelectionRect) -> ::js_sys::Number;
    # [wasm_bindgen (method , getter , js_class = SelectionRect)]
    #[doc = "Distance from the left edge of the screen to the left edge of the rectangle."]
    pub fn left(this: &SelectionRect) -> ::js_sys::Number;
    # [wasm_bindgen (method , getter , js_class = SelectionRect)]
    #[doc = "Distance from the top edge of the screen to the top edge of the rectangle."]
    pub fn top(this: &SelectionRect) -> ::js_sys::Number;
    # [wasm_bindgen (method , getter , js_class = SelectionRect)]
    #[doc = "Width of the rectangle."]
    pub fn width(this: &SelectionRect) -> ::js_sys::Number;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "WebRequestEventInterface" , typescript_type = "WebRequestEventInterface")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Interface which provides access to webRequest events on the guest page. See the <a href=\"http://developer.chrome.com/extensions/webRequest\">chrome.webRequest</a> extensions API for details on webRequest life cycle and related concepts. Note: The <a href=\"/extensions/webRequest#event-onActionIgnored\">chrome.webRequest.onActionIgnored</a> event is not supported for webviews. <p>To illustrate how usage differs from the extensions webRequest API, consider the following example code which blocks any guest requests for URLs which match <code>*://www.evil.com/*</code>:</p><pre>webview.request.onBeforeRequest.addListener(\r  function(details) { return {cancel: true}; },\r  {urls: [\"*://www.evil.com/*\"]},\r  [\"blocking\"]);</pre><p>Additionally, this interface supports declarative webRequest rules through <code>onRequest</code> and <code>onMessage</code> events. See <a href=\"http://developer.chrome.com/extensions/declarativeWebRequest.html\">declarativeWebRequest</a> for API details.</p>Note that conditions and actions for declarative webview webRequests should be instantiated from their <code>chrome.webViewRequest.*</code> counterparts. The following example code declaratively blocks all requests to <code>\"example.com\"</code> on the webview <code>myWebview</code>:</p><pre>var rule = {\r  conditions: [\r    new chrome.webViewRequest.RequestMatcher({ url: { hostSuffix: 'example.com' } })\r  ],\r  actions: [ new chrome.webViewRequest.CancelRequest() ]\r};\rmyWebview.request.onRequest.addRules([rule]);</pre>"]
    pub type WebRequestEventInterface;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "ZoomMode" , typescript_type = "ZoomMode")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Defines the how zooming is handled in the <code>webview</code>."]
    pub type ZoomMode;
}
