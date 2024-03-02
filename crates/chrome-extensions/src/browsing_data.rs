#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>chrome.browsingData</code> API to remove browsing data from a user's local profile."]
# [wasm_bindgen (js_namespace = chrome)]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "browsingData.RemovalOptions" , typescript_type = "browsingData.RemovalOptions")]
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
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "browsingData.DataTypeSet" , typescript_type = "browsingData.DataTypeSet")]
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
    #[doc = "Reports which types of data are currently selected in the 'Clear browsing data' settings UI.  Note: some of the data types included in this API are not available in the settings UI, and some UI settings control more than one data type listed here."]
    #[wasm_bindgen(js_name = "browsingData.settings", catch)]
    pub async fn settings() -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Clears various types of browsing data stored in a user's profile."]
    #[wasm_bindgen(js_name = "browsingData.remove", catch)]
    pub async fn remove(
        options: RemovalOptions,
        dataToRemove: DataTypeSet,
    ) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Clears websites' appcache data."]
    #[wasm_bindgen(js_name = "browsingData.removeAppcache", catch)]
    pub async fn removeAppcache(options: RemovalOptions) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Clears the browser's cache."]
    #[wasm_bindgen(js_name = "browsingData.removeCache", catch)]
    pub async fn removeCache(options: RemovalOptions) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Clears websites' cache storage data."]
    #[wasm_bindgen(js_name = "browsingData.removeCacheStorage", catch)]
    pub async fn removeCacheStorage(options: RemovalOptions)
        -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Clears the browser's cookies and server-bound certificates modified within a particular timeframe."]
    #[wasm_bindgen(js_name = "browsingData.removeCookies", catch)]
    pub async fn removeCookies(options: RemovalOptions) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Clears the browser's list of downloaded files (<em>not</em> the downloaded files themselves)."]
    #[wasm_bindgen(js_name = "browsingData.removeDownloads", catch)]
    pub async fn removeDownloads(options: RemovalOptions) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Clears websites' file system data."]
    #[wasm_bindgen(js_name = "browsingData.removeFileSystems", catch)]
    pub async fn removeFileSystems(options: RemovalOptions) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Clears the browser's stored form data (autofill)."]
    #[wasm_bindgen(js_name = "browsingData.removeFormData", catch)]
    pub async fn removeFormData(options: RemovalOptions) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Clears the browser's history."]
    #[wasm_bindgen(js_name = "browsingData.removeHistory", catch)]
    pub async fn removeHistory(options: RemovalOptions) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Clears websites' IndexedDB data."]
    #[wasm_bindgen(js_name = "browsingData.removeIndexedDB", catch)]
    pub async fn removeIndexedDB(options: RemovalOptions) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Clears websites' local storage data."]
    #[wasm_bindgen(js_name = "browsingData.removeLocalStorage", catch)]
    pub async fn removeLocalStorage(options: RemovalOptions)
        -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Clears plugins' data."]
    #[wasm_bindgen(js_name = "browsingData.removePluginData", catch)]
    pub async fn removePluginData(options: RemovalOptions) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Clears the browser's stored passwords."]
    #[wasm_bindgen(js_name = "browsingData.removePasswords", catch)]
    pub async fn removePasswords(options: RemovalOptions) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Clears websites' service workers."]
    #[wasm_bindgen(js_name = "browsingData.removeServiceWorkers", catch)]
    pub async fn removeServiceWorkers(
        options: RemovalOptions,
    ) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Clears websites' WebSQL data."]
    #[wasm_bindgen(js_name = "browsingData.removeWebSQL", catch)]
    pub async fn removeWebSQL(options: RemovalOptions) -> Result<(), ::wasm_bindgen::JsValue>;
}
#[wasm_bindgen]
pub async fn browsing_data_settings() -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    settings().await
}
#[wasm_bindgen]
pub async fn browsing_data_remove(
    options: RemovalOptions,
    dataToRemove: DataTypeSet,
) -> Result<(), ::wasm_bindgen::JsValue> {
    remove(options, dataToRemove).await
}
#[wasm_bindgen]
pub async fn browsing_data_remove_appcache(
    options: RemovalOptions,
) -> Result<(), ::wasm_bindgen::JsValue> {
    removeAppcache(options).await
}
#[wasm_bindgen]
pub async fn browsing_data_remove_cache(
    options: RemovalOptions,
) -> Result<(), ::wasm_bindgen::JsValue> {
    removeCache(options).await
}
#[wasm_bindgen]
pub async fn browsing_data_remove_cache_storage(
    options: RemovalOptions,
) -> Result<(), ::wasm_bindgen::JsValue> {
    removeCacheStorage(options).await
}
#[wasm_bindgen]
pub async fn browsing_data_remove_cookies(
    options: RemovalOptions,
) -> Result<(), ::wasm_bindgen::JsValue> {
    removeCookies(options).await
}
#[wasm_bindgen]
pub async fn browsing_data_remove_downloads(
    options: RemovalOptions,
) -> Result<(), ::wasm_bindgen::JsValue> {
    removeDownloads(options).await
}
#[wasm_bindgen]
pub async fn browsing_data_remove_file_systems(
    options: RemovalOptions,
) -> Result<(), ::wasm_bindgen::JsValue> {
    removeFileSystems(options).await
}
#[wasm_bindgen]
pub async fn browsing_data_remove_form_data(
    options: RemovalOptions,
) -> Result<(), ::wasm_bindgen::JsValue> {
    removeFormData(options).await
}
#[wasm_bindgen]
pub async fn browsing_data_remove_history(
    options: RemovalOptions,
) -> Result<(), ::wasm_bindgen::JsValue> {
    removeHistory(options).await
}
#[wasm_bindgen]
pub async fn browsing_data_remove_indexed_db(
    options: RemovalOptions,
) -> Result<(), ::wasm_bindgen::JsValue> {
    removeIndexedDB(options).await
}
#[wasm_bindgen]
pub async fn browsing_data_remove_local_storage(
    options: RemovalOptions,
) -> Result<(), ::wasm_bindgen::JsValue> {
    removeLocalStorage(options).await
}
#[wasm_bindgen]
pub async fn browsing_data_remove_plugin_data(
    options: RemovalOptions,
) -> Result<(), ::wasm_bindgen::JsValue> {
    removePluginData(options).await
}
#[wasm_bindgen]
pub async fn browsing_data_remove_passwords(
    options: RemovalOptions,
) -> Result<(), ::wasm_bindgen::JsValue> {
    removePasswords(options).await
}
#[wasm_bindgen]
pub async fn browsing_data_remove_service_workers(
    options: RemovalOptions,
) -> Result<(), ::wasm_bindgen::JsValue> {
    removeServiceWorkers(options).await
}
#[wasm_bindgen]
pub async fn browsing_data_remove_web_sql(
    options: RemovalOptions,
) -> Result<(), ::wasm_bindgen::JsValue> {
    removeWebSQL(options).await
}
