#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "none"]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "DataTypeSet" , typescript_type = "DataTypeSet")]
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
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "RemovalOptions" , typescript_type = "RemovalOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Options that determine exactly what data will be removed."]
    pub type RemovalOptions;
    # [wasm_bindgen (method , getter , js_class = RemovalOptions)]
    #[doc = "Remove data accumulated on or after this date, represented in milliseconds since the epoch (accessible via the <code>getTime</code> method of the JavaScript <code>Date</code> object). If absent, defaults to 0 (which would remove all browsing data)."]
    pub fn since(this: &RemovalOptions) -> Option<::js_sys::Number>;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "ZoomMode" , typescript_type = "ZoomMode")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Defines the how zooming is handled in the webview."]
    pub type ZoomMode;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "StopFindingAction" , typescript_type = "StopFindingAction")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Determines what to do with the active match after the find session has ended. 'clear' will clear the highlighting over the active match; 'keep' will keep the active match highlighted; 'activate' will keep the active match highlighted and simulate a user click on that match."]
    pub type StopFindingAction;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "SetPermissionAction" , typescript_type = "SetPermissionAction")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type SetPermissionAction;
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
    #[doc = "Details of the content script to inject."]
    pub type ContentScriptDetails;
    # [wasm_bindgen (method , getter , js_class = ContentScriptDetails)]
    #[doc = "If allFrames is <code>true</code>, implies that the JavaScript or CSS should be injected into all frames of current page. By default, it's <code>false</code> and is only injected into the top frame."]
    pub fn all_frames(this: &ContentScriptDetails) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = ContentScriptDetails)]
    #[doc = "The CSS code or a list of CSS files to be injected into matching pages. These are injected in the order they appear, before any DOM is constructed or displayed for the page."]
    pub fn css(this: &ContentScriptDetails) -> Option<i32>;
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
    pub fn js(this: &ContentScriptDetails) -> Option<i32>;
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
    pub fn run_at(this: &ContentScriptDetails) -> Option<i32>;
}
