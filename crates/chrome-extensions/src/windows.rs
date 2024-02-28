#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>chrome.windows</code> API to interact with browser windows. You can use this API to create, modify, and rearrange windows in the browser."]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "WindowType" , typescript_type = "WindowType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The type of browser window this is. In some circumstances a window may not be assigned a <code>type</code> property; for example, when querying closed windows from the $(ref:sessions) API."]
    pub type WindowType;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "WindowState" , typescript_type = "WindowState")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The state of this browser window. In some circumstances a window may not be assigned a <code>state</code> property; for example, when querying closed windows from the $(ref:sessions) API."]
    pub type WindowState;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "Window" , typescript_type = "Window")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type Window;
    # [wasm_bindgen (method , getter , js_class = Window)]
    #[doc = "Whether the window is set to be always on top."]
    pub fn alwaysOnTop(this: &Window) -> ::js_sys::Boolean;
    # [wasm_bindgen (method , getter , js_class = Window)]
    #[doc = "Whether the window is currently the focused window."]
    pub fn focused(this: &Window) -> ::js_sys::Boolean;
    # [wasm_bindgen (method , getter , js_class = Window)]
    #[doc = "The height of the window, including the frame, in pixels. In some circumstances a window may not be assigned a <code>height</code> property; for example, when querying closed windows from the $(ref:sessions) API."]
    pub fn height(this: &Window) -> Option<::js_sys::Number>;
    # [wasm_bindgen (method , getter , js_class = Window)]
    #[doc = "The ID of the window. Window IDs are unique within a browser session. In some circumstances a window may not be assigned an <code>ID</code> property; for example, when querying windows using the $(ref:sessions) API, in which case a session ID may be present."]
    pub fn id(this: &Window) -> Option<::js_sys::Number>;
    # [wasm_bindgen (method , getter , js_class = Window)]
    #[doc = "Whether the window is incognito."]
    pub fn incognito(this: &Window) -> ::js_sys::Boolean;
    # [wasm_bindgen (method , getter , js_class = Window)]
    #[doc = "The offset of the window from the left edge of the screen in pixels. In some circumstances a window may not be assigned a <code>left</code> property; for example, when querying closed windows from the $(ref:sessions) API."]
    pub fn left(this: &Window) -> Option<::js_sys::Number>;
    # [wasm_bindgen (method , getter , js_class = Window)]
    #[doc = "The session ID used to uniquely identify a window, obtained from the $(ref:sessions) API."]
    pub fn sessionId(this: &Window) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = Window)]
    #[doc = "The state of this browser window."]
    pub fn state(this: &Window) -> Option<WindowState>;
    # [wasm_bindgen (method , getter , js_class = Window)]
    #[doc = "Array of $(ref:tabs.Tab) objects representing the current tabs in the window."]
    pub fn tabs(this: &Window) -> Option<::js_sys::Array>;
    # [wasm_bindgen (method , getter , js_class = Window)]
    #[doc = "The offset of the window from the top edge of the screen in pixels. In some circumstances a window may not be assigned a <code>top</code> property; for example, when querying closed windows from the $(ref:sessions) API."]
    pub fn top(this: &Window) -> Option<::js_sys::Number>;
    # [wasm_bindgen (method , getter , js_class = Window)]
    #[doc = "The type of browser window this is."]
    pub fn type_(this: &Window) -> Option<WindowType>;
    # [wasm_bindgen (method , getter , js_class = Window)]
    #[doc = "The width of the window, including the frame, in pixels. In some circumstances a window may not be assigned a <code>width</code> property; for example, when querying closed windows from the $(ref:sessions) API."]
    pub fn width(this: &Window) -> Option<::js_sys::Number>;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "CreateType" , typescript_type = "CreateType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Specifies what type of browser window to create. 'panel' is deprecated and is available only to existing allowlisted extensions on Chrome OS."]
    pub type CreateType;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "QueryOptions" , typescript_type = "QueryOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type QueryOptions;
    # [wasm_bindgen (method , getter , js_class = QueryOptions)]
    #[doc = "If true, the $(ref:windows.Window) object has a <var>tabs</var> property that contains a list of the $(ref:tabs.Tab) objects. The <code>Tab</code> objects only contain the <code>url</code>, <code>pendingUrl</code>, <code>title</code>, and <code>favIconUrl</code> properties if the extension's manifest file includes the <code>\"tabs\"</code> permission."]
    pub fn populate(this: &QueryOptions) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = QueryOptions)]
    #[doc = "If set, the $(ref:windows.Window) returned is filtered based on its type. If unset, the default filter is set to <code>['normal', 'popup']</code>."]
    pub fn windowTypes(this: &QueryOptions) -> Option<::js_sys::Array>;
}
