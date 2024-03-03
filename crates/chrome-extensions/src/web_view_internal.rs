#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "none"]
# [wasm_bindgen (js_namespace = chrome)]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "webViewInternal.DataTypeSet" , typescript_type = "webViewInternal.DataTypeSet")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "A set of data types. Missing data types are interpreted as <code>false</code>."]
    pub type DataTypeSet;
    # [wasm_bindgen (method , getter , js_class = DataTypeSet)]
    #[doc = "Websites' appcaches."]
    pub fn appcache(this: &DataTypeSet) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = DataTypeSet)]
    #[doc = "The Websites' cache data. Note: when removing data, this clears the <em>entire</em> cache: it is not limited to the range you specify."]
    pub fn cache(this: &DataTypeSet) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = DataTypeSet)]
    #[doc = "The Websites' cookies. This will remove both session and persistent cookies"]
    pub fn cookies(this: &DataTypeSet) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = DataTypeSet)]
    #[doc = "Websites' file systems."]
    pub fn fileSystems(this: &DataTypeSet) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = DataTypeSet)]
    #[doc = "Websites' IndexedDB data."]
    pub fn indexedDB(this: &DataTypeSet) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = DataTypeSet)]
    #[doc = "Websites' local storage data."]
    pub fn localStorage(this: &DataTypeSet) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = DataTypeSet)]
    #[doc = "The Websites' persistent cookies."]
    pub fn persistentCookies(this: &DataTypeSet) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = DataTypeSet)]
    #[doc = "The Websites' session cookies."]
    pub fn sessionCookies(this: &DataTypeSet) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = DataTypeSet)]
    #[doc = "Websites' WebSQL data."]
    pub fn webSQL(this: &DataTypeSet) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "webViewInternal.RemovalOptions" , typescript_type = "webViewInternal.RemovalOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Options that determine exactly what data will be removed."]
    pub type RemovalOptions;
    # [wasm_bindgen (method , getter , js_class = RemovalOptions)]
    #[doc = "Remove data accumulated on or after this date, represented in milliseconds since the epoch (accessible via the <code>getTime</code> method of the JavaScript <code>Date</code> object). If absent, defaults to 0 (which would remove all browsing data)."]
    pub fn since(this: &RemovalOptions) -> Option<::js_sys::Number>;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "webViewInternal.ZoomMode" , typescript_type = "webViewInternal.ZoomMode")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Defines the how zooming is handled in the webview."]
    pub type ZoomMode;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "webViewInternal.StopFindingAction" , typescript_type = "webViewInternal.StopFindingAction")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Determines what to do with the active match after the find session has ended. 'clear' will clear the highlighting over the active match; 'keep' will keep the active match highlighted; 'activate' will keep the active match highlighted and simulate a user click on that match."]
    pub type StopFindingAction;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "webViewInternal.SetPermissionAction" , typescript_type = "webViewInternal.SetPermissionAction")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type SetPermissionAction;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "webViewInternal.InjectionItems" , typescript_type = "webViewInternal.InjectionItems")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The type of injection item: code or a set of files."]
    pub type InjectionItems;
    # [wasm_bindgen (method , getter , js_class = InjectionItems)]
    #[doc = "JavaScript code or CSS to be injected into matching pages."]
    pub fn code(this: &InjectionItems) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = InjectionItems)]
    #[doc = "The list of JavaScript or CSS files to be injected into matching pages. These are injected in the order they appear in this array."]
    pub fn files(this: &InjectionItems) -> Option<::js_sys::Array>;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "webViewInternal.ContentScriptDetails" , typescript_type = "webViewInternal.ContentScriptDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Details of the content script to inject."]
    pub type ContentScriptDetails;
    # [wasm_bindgen (method , getter , js_class = ContentScriptDetails)]
    #[doc = "If allFrames is <code>true</code>, implies that the JavaScript or CSS should be injected into all frames of current page. By default, it's <code>false</code> and is only injected into the top frame."]
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
    #[doc = "Callback that returns audio state."]
    #[wasm_bindgen(js_name = "webViewInternal.getAudioState", catch)]
    pub async fn get_audio_state(
        instanceId: ::js_sys::Number,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Callback that returns audio state."]
    #[wasm_bindgen(js_name = "webViewInternal.getAudioState")]
    pub fn get_audio_state_callback(instanceId: ::js_sys::Number, callback: &::js_sys::Function);
    #[doc = "Callback that returns whether audio is muted."]
    #[wasm_bindgen(js_name = "webViewInternal.isAudioMuted", catch)]
    pub async fn is_audio_muted(
        instanceId: ::js_sys::Number,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Callback that returns whether audio is muted."]
    #[wasm_bindgen(js_name = "webViewInternal.isAudioMuted")]
    pub fn is_audio_muted_callback(instanceId: ::js_sys::Number, callback: &::js_sys::Function);
    #[doc = "Injects JavaScript code into a <webview> page."]
    #[wasm_bindgen(js_name = "webViewInternal.executeScript", catch)]
    pub async fn execute_script(
        instanceId: ::js_sys::Number,
        src: ::js_sys::JsString,
        details: crate::extension_types::InjectDetails,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Injects JavaScript code into a <webview> page."]
    #[wasm_bindgen(js_name = "webViewInternal.executeScript")]
    pub fn execute_script_callback(
        instanceId: ::js_sys::Number,
        src: ::js_sys::JsString,
        details: crate::extension_types::InjectDetails,
        callback: &::js_sys::Function,
    );
    #[doc = "Injects CSS into a <webview> page. For details, see the <a href='/extensions/content_scripts#pi'>programmatic injection</a> section of the content scripts doc."]
    #[wasm_bindgen(js_name = "webViewInternal.insertCSS", catch)]
    pub async fn insert_css(
        instanceId: ::js_sys::Number,
        src: ::js_sys::JsString,
        details: crate::extension_types::InjectDetails,
    ) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Injects CSS into a <webview> page. For details, see the <a href='/extensions/content_scripts#pi'>programmatic injection</a> section of the content scripts doc."]
    #[wasm_bindgen(js_name = "webViewInternal.insertCSS")]
    pub fn insert_css_callback(
        instanceId: ::js_sys::Number,
        src: ::js_sys::JsString,
        details: crate::extension_types::InjectDetails,
        callback: &::js_sys::Function,
    );
    #[doc = ""]
    #[wasm_bindgen(js_name = "webViewInternal.setZoom", catch)]
    pub async fn set_zoom(
        instanceId: ::js_sys::Number,
        zoomFactor: ::js_sys::Number,
    ) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = ""]
    #[wasm_bindgen(js_name = "webViewInternal.setZoom")]
    pub fn set_zoom_callback(
        instanceId: ::js_sys::Number,
        zoomFactor: ::js_sys::Number,
        callback: &::js_sys::Function,
    );
    #[doc = ""]
    #[wasm_bindgen(js_name = "webViewInternal.getZoom", catch)]
    pub async fn get_zoom(
        instanceId: ::js_sys::Number,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = ""]
    #[wasm_bindgen(js_name = "webViewInternal.getZoom")]
    pub fn get_zoom_callback(instanceId: ::js_sys::Number, callback: &::js_sys::Function);
    #[doc = "Sets the zoom mode of the webview."]
    #[wasm_bindgen(js_name = "webViewInternal.setZoomMode", catch)]
    pub async fn set_zoom_mode(
        instanceId: ::js_sys::Number,
        ZoomMode: ZoomMode,
    ) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Sets the zoom mode of the webview."]
    #[wasm_bindgen(js_name = "webViewInternal.setZoomMode")]
    pub fn set_zoom_mode_callback(
        instanceId: ::js_sys::Number,
        ZoomMode: ZoomMode,
        callback: &::js_sys::Function,
    );
    #[doc = "Gets the current zoom mode."]
    #[wasm_bindgen(js_name = "webViewInternal.getZoomMode", catch)]
    pub async fn get_zoom_mode(
        instanceId: ::js_sys::Number,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Gets the current zoom mode."]
    #[wasm_bindgen(js_name = "webViewInternal.getZoomMode")]
    pub fn get_zoom_mode_callback(instanceId: ::js_sys::Number, callback: &::js_sys::Function);
    #[doc = "Initiates a find-in-page request."]
    #[wasm_bindgen(js_name = "webViewInternal.find", catch)]
    pub async fn find(
        instanceId: ::js_sys::Number,
        searchText: ::js_sys::JsString,
        options: Option<::js_sys::Object>,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Initiates a find-in-page request."]
    #[wasm_bindgen(js_name = "webViewInternal.find")]
    pub fn find_callback(
        instanceId: ::js_sys::Number,
        searchText: ::js_sys::JsString,
        options: Option<::js_sys::Object>,
        callback: &::js_sys::Function,
    );
    #[doc = "Loads a data URL with a specified base URL used for relative links. Optionally, a virtual URL can be provided to be shown to the user instead of the data URL."]
    #[wasm_bindgen(js_name = "webViewInternal.loadDataWithBaseUrl", catch)]
    pub async fn load_data_with_base_url(
        instanceId: ::js_sys::Number,
        dataUrl: ::js_sys::JsString,
        baseUrl: ::js_sys::JsString,
        virtualUrl: Option<::js_sys::JsString>,
    ) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Loads a data URL with a specified base URL used for relative links. Optionally, a virtual URL can be provided to be shown to the user instead of the data URL."]
    #[wasm_bindgen(js_name = "webViewInternal.loadDataWithBaseUrl")]
    pub fn load_data_with_base_url_callback(
        instanceId: ::js_sys::Number,
        dataUrl: ::js_sys::JsString,
        baseUrl: ::js_sys::JsString,
        virtualUrl: Option<::js_sys::JsString>,
        callback: &::js_sys::Function,
    );
    #[doc = ""]
    #[wasm_bindgen(js_name = "webViewInternal.go", catch)]
    pub async fn go(
        instanceId: ::js_sys::Number,
        relativeIndex: ::js_sys::Number,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = ""]
    #[wasm_bindgen(js_name = "webViewInternal.go")]
    pub fn go_callback(
        instanceId: ::js_sys::Number,
        relativeIndex: ::js_sys::Number,
        callback: &::js_sys::Function,
    );
    #[doc = ""]
    #[wasm_bindgen(js_name = "webViewInternal.setPermission", catch)]
    pub async fn set_permission(
        instanceId: ::js_sys::Number,
        requestId: ::js_sys::Number,
        action: SetPermissionAction,
        userInput: Option<::js_sys::JsString>,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = ""]
    #[wasm_bindgen(js_name = "webViewInternal.setPermission")]
    pub fn set_permission_callback(
        instanceId: ::js_sys::Number,
        requestId: ::js_sys::Number,
        action: SetPermissionAction,
        userInput: Option<::js_sys::JsString>,
        callback: &::js_sys::Function,
    );
    #[doc = "foo"]
    #[wasm_bindgen(js_name = "webViewInternal.captureVisibleRegion", catch)]
    pub async fn capture_visible_region(
        instanceId: ::js_sys::Number,
        options: Option<crate::extension_types::ImageDetails>,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "foo"]
    #[wasm_bindgen(js_name = "webViewInternal.captureVisibleRegion")]
    pub fn capture_visible_region_callback(
        instanceId: ::js_sys::Number,
        options: Option<crate::extension_types::ImageDetails>,
        callback: &::js_sys::Function,
    );
    #[doc = "Callback that returns whether whether spatial navigation is enabled for the webview."]
    #[wasm_bindgen(js_name = "webViewInternal.isSpatialNavigationEnabled", catch)]
    pub async fn is_spatial_navigation_enabled(
        instanceId: ::js_sys::Number,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Callback that returns whether whether spatial navigation is enabled for the webview."]
    #[wasm_bindgen(js_name = "webViewInternal.isSpatialNavigationEnabled")]
    pub fn is_spatial_navigation_enabled_callback(
        instanceId: ::js_sys::Number,
        callback: &::js_sys::Function,
    );
    #[doc = "Clears various types of browsing data stored in a storage partition of a <webview>."]
    #[wasm_bindgen(js_name = "webViewInternal.clearData", catch)]
    pub async fn clear_data(
        instanceId: ::js_sys::Number,
        options: RemovalOptions,
        dataToRemove: DataTypeSet,
    ) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Clears various types of browsing data stored in a storage partition of a <webview>."]
    #[wasm_bindgen(js_name = "webViewInternal.clearData")]
    pub fn clear_data_callback(
        instanceId: ::js_sys::Number,
        options: RemovalOptions,
        dataToRemove: DataTypeSet,
        callback: &::js_sys::Function,
    );
}
#[wasm_bindgen]
pub async fn web_view_internal_get_audio_state(
    instanceId: ::js_sys::Number,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    get_audio_state(instanceId).await
}
#[wasm_bindgen]
pub fn web_view_internal_get_audio_state_callback(
    instanceId: ::js_sys::Number,
    callback: &::js_sys::Function,
) {
    get_audio_state_callback(instanceId, callback);
}
#[wasm_bindgen]
pub async fn web_view_internal_is_audio_muted(
    instanceId: ::js_sys::Number,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    is_audio_muted(instanceId).await
}
#[wasm_bindgen]
pub fn web_view_internal_is_audio_muted_callback(
    instanceId: ::js_sys::Number,
    callback: &::js_sys::Function,
) {
    is_audio_muted_callback(instanceId, callback);
}
#[wasm_bindgen]
pub async fn web_view_internal_execute_script(
    instanceId: ::js_sys::Number,
    src: ::js_sys::JsString,
    details: crate::extension_types::InjectDetails,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    execute_script(instanceId, src, details).await
}
#[wasm_bindgen]
pub fn web_view_internal_execute_script_callback(
    instanceId: ::js_sys::Number,
    src: ::js_sys::JsString,
    details: crate::extension_types::InjectDetails,
    callback: &::js_sys::Function,
) {
    execute_script_callback(instanceId, src, details, callback);
}
#[wasm_bindgen]
pub async fn web_view_internal_insert_css(
    instanceId: ::js_sys::Number,
    src: ::js_sys::JsString,
    details: crate::extension_types::InjectDetails,
) -> Result<(), ::wasm_bindgen::JsValue> {
    insert_css(instanceId, src, details).await
}
#[wasm_bindgen]
pub fn web_view_internal_insert_css_callback(
    instanceId: ::js_sys::Number,
    src: ::js_sys::JsString,
    details: crate::extension_types::InjectDetails,
    callback: &::js_sys::Function,
) {
    insert_css_callback(instanceId, src, details, callback);
}
#[wasm_bindgen]
pub async fn web_view_internal_set_zoom(
    instanceId: ::js_sys::Number,
    zoomFactor: ::js_sys::Number,
) -> Result<(), ::wasm_bindgen::JsValue> {
    set_zoom(instanceId, zoomFactor).await
}
#[wasm_bindgen]
pub fn web_view_internal_set_zoom_callback(
    instanceId: ::js_sys::Number,
    zoomFactor: ::js_sys::Number,
    callback: &::js_sys::Function,
) {
    set_zoom_callback(instanceId, zoomFactor, callback);
}
#[wasm_bindgen]
pub async fn web_view_internal_get_zoom(
    instanceId: ::js_sys::Number,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    get_zoom(instanceId).await
}
#[wasm_bindgen]
pub fn web_view_internal_get_zoom_callback(
    instanceId: ::js_sys::Number,
    callback: &::js_sys::Function,
) {
    get_zoom_callback(instanceId, callback);
}
#[wasm_bindgen]
pub async fn web_view_internal_set_zoom_mode(
    instanceId: ::js_sys::Number,
    ZoomMode: ZoomMode,
) -> Result<(), ::wasm_bindgen::JsValue> {
    set_zoom_mode(instanceId, ZoomMode).await
}
#[wasm_bindgen]
pub fn web_view_internal_set_zoom_mode_callback(
    instanceId: ::js_sys::Number,
    ZoomMode: ZoomMode,
    callback: &::js_sys::Function,
) {
    set_zoom_mode_callback(instanceId, ZoomMode, callback);
}
#[wasm_bindgen]
pub async fn web_view_internal_get_zoom_mode(
    instanceId: ::js_sys::Number,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    get_zoom_mode(instanceId).await
}
#[wasm_bindgen]
pub fn web_view_internal_get_zoom_mode_callback(
    instanceId: ::js_sys::Number,
    callback: &::js_sys::Function,
) {
    get_zoom_mode_callback(instanceId, callback);
}
#[wasm_bindgen]
pub async fn web_view_internal_find(
    instanceId: ::js_sys::Number,
    searchText: ::js_sys::JsString,
    options: Option<::js_sys::Object>,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    find(instanceId, searchText, options).await
}
#[wasm_bindgen]
pub fn web_view_internal_find_callback(
    instanceId: ::js_sys::Number,
    searchText: ::js_sys::JsString,
    options: Option<::js_sys::Object>,
    callback: &::js_sys::Function,
) {
    find_callback(instanceId, searchText, options, callback);
}
#[wasm_bindgen]
pub async fn web_view_internal_load_data_with_base_url(
    instanceId: ::js_sys::Number,
    dataUrl: ::js_sys::JsString,
    baseUrl: ::js_sys::JsString,
    virtualUrl: Option<::js_sys::JsString>,
) -> Result<(), ::wasm_bindgen::JsValue> {
    load_data_with_base_url(instanceId, dataUrl, baseUrl, virtualUrl).await
}
#[wasm_bindgen]
pub fn web_view_internal_load_data_with_base_url_callback(
    instanceId: ::js_sys::Number,
    dataUrl: ::js_sys::JsString,
    baseUrl: ::js_sys::JsString,
    virtualUrl: Option<::js_sys::JsString>,
    callback: &::js_sys::Function,
) {
    load_data_with_base_url_callback(instanceId, dataUrl, baseUrl, virtualUrl, callback);
}
#[wasm_bindgen]
pub async fn web_view_internal_go(
    instanceId: ::js_sys::Number,
    relativeIndex: ::js_sys::Number,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    go(instanceId, relativeIndex).await
}
#[wasm_bindgen]
pub fn web_view_internal_go_callback(
    instanceId: ::js_sys::Number,
    relativeIndex: ::js_sys::Number,
    callback: &::js_sys::Function,
) {
    go_callback(instanceId, relativeIndex, callback);
}
#[wasm_bindgen]
pub async fn web_view_internal_set_permission(
    instanceId: ::js_sys::Number,
    requestId: ::js_sys::Number,
    action: SetPermissionAction,
    userInput: Option<::js_sys::JsString>,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    set_permission(instanceId, requestId, action, userInput).await
}
#[wasm_bindgen]
pub fn web_view_internal_set_permission_callback(
    instanceId: ::js_sys::Number,
    requestId: ::js_sys::Number,
    action: SetPermissionAction,
    userInput: Option<::js_sys::JsString>,
    callback: &::js_sys::Function,
) {
    set_permission_callback(instanceId, requestId, action, userInput, callback);
}
#[wasm_bindgen]
pub async fn web_view_internal_capture_visible_region(
    instanceId: ::js_sys::Number,
    options: Option<crate::extension_types::ImageDetails>,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    capture_visible_region(instanceId, options).await
}
#[wasm_bindgen]
pub fn web_view_internal_capture_visible_region_callback(
    instanceId: ::js_sys::Number,
    options: Option<crate::extension_types::ImageDetails>,
    callback: &::js_sys::Function,
) {
    capture_visible_region_callback(instanceId, options, callback);
}
#[wasm_bindgen]
pub async fn web_view_internal_is_spatial_navigation_enabled(
    instanceId: ::js_sys::Number,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    is_spatial_navigation_enabled(instanceId).await
}
#[wasm_bindgen]
pub fn web_view_internal_is_spatial_navigation_enabled_callback(
    instanceId: ::js_sys::Number,
    callback: &::js_sys::Function,
) {
    is_spatial_navigation_enabled_callback(instanceId, callback);
}
#[wasm_bindgen]
pub async fn web_view_internal_clear_data(
    instanceId: ::js_sys::Number,
    options: RemovalOptions,
    dataToRemove: DataTypeSet,
) -> Result<(), ::wasm_bindgen::JsValue> {
    clear_data(instanceId, options, dataToRemove).await
}
#[wasm_bindgen]
pub fn web_view_internal_clear_data_callback(
    instanceId: ::js_sys::Number,
    options: RemovalOptions,
    dataToRemove: DataTypeSet,
    callback: &::js_sys::Function,
) {
    clear_data_callback(instanceId, options, dataToRemove, callback);
}
