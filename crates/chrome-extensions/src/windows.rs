#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>chrome.windows</code> API to interact with browser windows. You can use this API to create, modify, and rearrange windows in the browser."]
# [wasm_bindgen (js_namespace = chrome)]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "windows.WindowType" , typescript_type = "windows.WindowType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The type of browser window this is. In some circumstances a window may not be assigned a <code>type</code> property; for example, when querying closed windows from the $(ref:sessions) API."]
    pub type WindowType;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "windows.WindowState" , typescript_type = "windows.WindowState")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The state of this browser window. In some circumstances a window may not be assigned a <code>state</code> property; for example, when querying closed windows from the $(ref:sessions) API."]
    pub type WindowState;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "windows.Window" , typescript_type = "windows.Window")]
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
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "windows.CreateType" , typescript_type = "windows.CreateType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Specifies what type of browser window to create. 'panel' is deprecated and is available only to existing allowlisted extensions on Chrome OS."]
    pub type CreateType;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "windows.QueryOptions" , typescript_type = "windows.QueryOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type QueryOptions;
    # [wasm_bindgen (method , getter , js_class = QueryOptions)]
    #[doc = "If true, the $(ref:windows.Window) object has a <var>tabs</var> property that contains a list of the $(ref:tabs.Tab) objects. The <code>Tab</code> objects only contain the <code>url</code>, <code>pendingUrl</code>, <code>title</code>, and <code>favIconUrl</code> properties if the extension's manifest file includes the <code>\"tabs\"</code> permission."]
    pub fn populate(this: &QueryOptions) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = QueryOptions)]
    #[doc = "If set, the $(ref:windows.Window) returned is filtered based on its type. If unset, the default filter is set to <code>['normal', 'popup']</code>."]
    pub fn windowTypes(this: &QueryOptions) -> Option<::js_sys::Array>;
    #[doc = "Gets details about a window."]
    #[wasm_bindgen(js_name = "windows.get", catch)]
    pub async fn get(
        windowId: ::js_sys::Number,
        queryOptions: Option<QueryOptions>,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Gets details about a window."]
    #[wasm_bindgen(js_name = "windows.get")]
    pub fn get_callback(
        windowId: ::js_sys::Number,
        queryOptions: Option<QueryOptions>,
        callback: &::js_sys::Function,
    );
    #[doc = "Gets the <a href='#current-window'>current window</a>."]
    #[wasm_bindgen(js_name = "windows.getCurrent", catch)]
    pub async fn get_current(
        queryOptions: Option<QueryOptions>,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Gets the <a href='#current-window'>current window</a>."]
    #[wasm_bindgen(js_name = "windows.getCurrent")]
    pub fn get_current_callback(queryOptions: Option<QueryOptions>, callback: &::js_sys::Function);
    #[doc = "Gets the window that was most recently focused &mdash; typically the window 'on top'."]
    #[wasm_bindgen(js_name = "windows.getLastFocused", catch)]
    pub async fn get_last_focused(
        queryOptions: Option<QueryOptions>,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Gets the window that was most recently focused &mdash; typically the window 'on top'."]
    #[wasm_bindgen(js_name = "windows.getLastFocused")]
    pub fn get_last_focused_callback(
        queryOptions: Option<QueryOptions>,
        callback: &::js_sys::Function,
    );
    #[doc = "Gets all windows."]
    #[wasm_bindgen(js_name = "windows.getAll", catch)]
    pub async fn get_all(
        queryOptions: Option<QueryOptions>,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Gets all windows."]
    #[wasm_bindgen(js_name = "windows.getAll")]
    pub fn get_all_callback(queryOptions: Option<QueryOptions>, callback: &::js_sys::Function);
    #[doc = "Creates (opens) a new browser window with any optional sizing, position, or default URL provided."]
    #[wasm_bindgen(js_name = "windows.create", catch)]
    pub async fn create(
        createData: Option<::js_sys::Object>,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Creates (opens) a new browser window with any optional sizing, position, or default URL provided."]
    #[wasm_bindgen(js_name = "windows.create")]
    pub fn create_callback(createData: Option<::js_sys::Object>, callback: &::js_sys::Function);
    #[doc = "Updates the properties of a window. Specify only the properties that to be changed; unspecified properties are unchanged."]
    #[wasm_bindgen(js_name = "windows.update", catch)]
    pub async fn update(
        windowId: ::js_sys::Number,
        updateInfo: ::js_sys::Object,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Updates the properties of a window. Specify only the properties that to be changed; unspecified properties are unchanged."]
    #[wasm_bindgen(js_name = "windows.update")]
    pub fn update_callback(
        windowId: ::js_sys::Number,
        updateInfo: ::js_sys::Object,
        callback: &::js_sys::Function,
    );
    #[doc = "Removes (closes) a window and all the tabs inside it."]
    #[wasm_bindgen(js_name = "windows.remove", catch)]
    pub async fn remove(windowId: ::js_sys::Number) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Removes (closes) a window and all the tabs inside it."]
    #[wasm_bindgen(js_name = "windows.remove")]
    pub fn remove_callback(windowId: ::js_sys::Number, callback: &::js_sys::Function);
}
#[wasm_bindgen]
pub async fn windows_get(
    windowId: ::js_sys::Number,
    queryOptions: Option<QueryOptions>,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    get(windowId, queryOptions).await
}
#[wasm_bindgen]
pub fn windows_get_callback(
    windowId: ::js_sys::Number,
    queryOptions: Option<QueryOptions>,
    callback: &::js_sys::Function,
) {
    get_callback(windowId, queryOptions, callback);
}
#[wasm_bindgen]
pub async fn windows_get_current(
    queryOptions: Option<QueryOptions>,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    get_current(queryOptions).await
}
#[wasm_bindgen]
pub fn windows_get_current_callback(
    queryOptions: Option<QueryOptions>,
    callback: &::js_sys::Function,
) {
    get_current_callback(queryOptions, callback);
}
#[wasm_bindgen]
pub async fn windows_get_last_focused(
    queryOptions: Option<QueryOptions>,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    get_last_focused(queryOptions).await
}
#[wasm_bindgen]
pub fn windows_get_last_focused_callback(
    queryOptions: Option<QueryOptions>,
    callback: &::js_sys::Function,
) {
    get_last_focused_callback(queryOptions, callback);
}
#[wasm_bindgen]
pub async fn windows_get_all(
    queryOptions: Option<QueryOptions>,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    get_all(queryOptions).await
}
#[wasm_bindgen]
pub fn windows_get_all_callback(queryOptions: Option<QueryOptions>, callback: &::js_sys::Function) {
    get_all_callback(queryOptions, callback);
}
#[wasm_bindgen]
pub async fn windows_create(
    createData: Option<::js_sys::Object>,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    create(createData).await
}
#[wasm_bindgen]
pub fn windows_create_callback(
    createData: Option<::js_sys::Object>,
    callback: &::js_sys::Function,
) {
    create_callback(createData, callback);
}
#[wasm_bindgen]
pub async fn windows_update(
    windowId: ::js_sys::Number,
    updateInfo: ::js_sys::Object,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    update(windowId, updateInfo).await
}
#[wasm_bindgen]
pub fn windows_update_callback(
    windowId: ::js_sys::Number,
    updateInfo: ::js_sys::Object,
    callback: &::js_sys::Function,
) {
    update_callback(windowId, updateInfo, callback);
}
#[wasm_bindgen]
pub async fn windows_remove(windowId: ::js_sys::Number) -> Result<(), ::wasm_bindgen::JsValue> {
    remove(windowId).await
}
#[wasm_bindgen]
pub fn windows_remove_callback(windowId: ::js_sys::Number, callback: &::js_sys::Function) {
    remove_callback(windowId, callback);
}
