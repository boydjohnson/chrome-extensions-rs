#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>chrome.browsingData</code> API to remove browsing data from a user's local profile."]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "RemovalOptions" , typescript_type = "RemovalOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Options that determine exactly what data will be removed."]
    pub type RemovalOptions;
    # [wasm_bindgen (method , getter , js_class = RemovalOptions)]
    #[doc = "When present, data for origins in this list is excluded from deletion. Can't be used together with <code>origins</code>. Only supported for cookies, storage and cache.  Cookies are excluded for the whole registrable domain."]
    pub fn excludeOrigins(this: &RemovalOptions) -> Option<::js_sys::Array>;
    # [wasm_bindgen (method , getter , js_class = RemovalOptions)]
    #[doc = "An object whose properties specify which origin types ought to be cleared. If this object isn't specified, it defaults to clearing only \"unprotected\" origins. Please ensure that you <em>really</em> want to remove application data before adding 'protectedWeb' or 'extensions'."]
    pub fn originTypes(this: &RemovalOptions) -> Option<::js_sys::Object>;
    # [wasm_bindgen (method , getter , js_class = RemovalOptions)]
    #[doc = "When present, only data for origins in this list is deleted. Only supported for cookies, storage and cache. Cookies are cleared for the whole registrable domain."]
    pub fn origins(this: &RemovalOptions) -> Option<::js_sys::Array>;
    # [wasm_bindgen (method , getter , js_class = RemovalOptions)]
    #[doc = "Remove data accumulated on or after this date, represented in milliseconds since the epoch (accessible via the <code>getTime</code> method of the JavaScript <code>Date</code> object). If absent, defaults to 0 (which would remove all browsing data)."]
    pub fn since(this: &RemovalOptions) -> Option<::js_sys::Number>;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "DataTypeSet" , typescript_type = "DataTypeSet")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "A set of data types. Missing data types are interpreted as <code>false</code>."]
    pub type DataTypeSet;
    # [wasm_bindgen (method , getter , js_class = DataTypeSet)]
    #[doc = "Websites' appcaches."]
    pub fn appcache(this: &DataTypeSet) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = DataTypeSet)]
    #[doc = "The browser's cache."]
    pub fn cache(this: &DataTypeSet) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = DataTypeSet)]
    #[doc = "Cache storage"]
    pub fn cacheStorage(this: &DataTypeSet) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = DataTypeSet)]
    #[doc = "The browser's cookies."]
    pub fn cookies(this: &DataTypeSet) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = DataTypeSet)]
    #[doc = "The browser's download list."]
    pub fn downloads(this: &DataTypeSet) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = DataTypeSet)]
    #[doc = "Websites' file systems."]
    pub fn fileSystems(this: &DataTypeSet) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = DataTypeSet)]
    #[doc = "The browser's stored form data."]
    pub fn formData(this: &DataTypeSet) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = DataTypeSet)]
    #[doc = "The browser's history."]
    pub fn history(this: &DataTypeSet) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = DataTypeSet)]
    #[doc = "Websites' IndexedDB data."]
    pub fn indexedDB(this: &DataTypeSet) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = DataTypeSet)]
    #[doc = "Websites' local storage data."]
    pub fn localStorage(this: &DataTypeSet) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = DataTypeSet)]
    #[doc = "Stored passwords."]
    pub fn passwords(this: &DataTypeSet) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = DataTypeSet)]
    #[doc = "Plugins' data."]
    pub fn pluginData(this: &DataTypeSet) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = DataTypeSet)]
    #[doc = "Server-bound certificates."]
    pub fn serverBoundCertificates(this: &DataTypeSet) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = DataTypeSet)]
    #[doc = "Service Workers."]
    pub fn serviceWorkers(this: &DataTypeSet) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = DataTypeSet)]
    #[doc = "Websites' WebSQL data."]
    pub fn webSQL(this: &DataTypeSet) -> Option<::js_sys::Boolean>;
}
