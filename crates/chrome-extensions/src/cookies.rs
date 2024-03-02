#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>chrome.cookies</code> API to query and modify cookies, and to be notified when they change."]
# [wasm_bindgen (js_namespace = chrome)]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "cookies.SameSiteStatus" , typescript_type = "cookies.SameSiteStatus")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "A cookie's 'SameSite' state (https://tools.ietf.org/html/draft-west-first-party-cookies). 'no_restriction' corresponds to a cookie set with 'SameSite=None', 'lax' to 'SameSite=Lax', and 'strict' to 'SameSite=Strict'. 'unspecified' corresponds to a cookie set without the SameSite attribute."]
    pub type SameSiteStatus;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "cookies.CookiePartitionKey" , typescript_type = "cookies.CookiePartitionKey")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Represents a partitioned cookie's partition key."]
    pub type CookiePartitionKey;
    # [wasm_bindgen (method , getter , js_class = CookiePartitionKey)]
    #[doc = "The top-level site the partitioned cookie is available in."]
    pub fn topLevelSite(this: &CookiePartitionKey) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "cookies.Cookie" , typescript_type = "cookies.Cookie")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Represents information about an HTTP cookie."]
    pub type Cookie;
    # [wasm_bindgen (method , getter , js_class = Cookie)]
    #[doc = "The domain of the cookie (e.g. \"www.google.com\", \"example.com\")."]
    pub fn domain(this: &Cookie) -> ::js_sys::JsString;
    # [wasm_bindgen (method , getter , js_class = Cookie)]
    #[doc = "The expiration date of the cookie as the number of seconds since the UNIX epoch. Not provided for session cookies."]
    pub fn expirationDate(this: &Cookie) -> Option<::js_sys::Number>;
    # [wasm_bindgen (method , getter , js_class = Cookie)]
    #[doc = "True if the cookie is a host-only cookie (i.e. a request's host must exactly match the domain of the cookie)."]
    pub fn hostOnly(this: &Cookie) -> ::js_sys::Boolean;
    # [wasm_bindgen (method , getter , js_class = Cookie)]
    #[doc = "True if the cookie is marked as HttpOnly (i.e. the cookie is inaccessible to client-side scripts)."]
    pub fn httpOnly(this: &Cookie) -> ::js_sys::Boolean;
    # [wasm_bindgen (method , getter , js_class = Cookie)]
    #[doc = "The name of the cookie."]
    pub fn name(this: &Cookie) -> ::js_sys::JsString;
    # [wasm_bindgen (method , getter , js_class = Cookie)]
    #[doc = "The partition key for reading or modifying cookies with the Partitioned attribute."]
    pub fn partitionKey(this: &Cookie) -> Option<CookiePartitionKey>;
    # [wasm_bindgen (method , getter , js_class = Cookie)]
    #[doc = "The path of the cookie."]
    pub fn path(this: &Cookie) -> ::js_sys::JsString;
    # [wasm_bindgen (method , getter , js_class = Cookie)]
    #[doc = "The cookie's same-site status (i.e. whether the cookie is sent with cross-site requests)."]
    pub fn sameSite(this: &Cookie) -> SameSiteStatus;
    # [wasm_bindgen (method , getter , js_class = Cookie)]
    #[doc = "True if the cookie is marked as Secure (i.e. its scope is limited to secure channels, typically HTTPS)."]
    pub fn secure(this: &Cookie) -> ::js_sys::Boolean;
    # [wasm_bindgen (method , getter , js_class = Cookie)]
    #[doc = "True if the cookie is a session cookie, as opposed to a persistent cookie with an expiration date."]
    pub fn session(this: &Cookie) -> ::js_sys::Boolean;
    # [wasm_bindgen (method , getter , js_class = Cookie)]
    #[doc = "The ID of the cookie store containing this cookie, as provided in getAllCookieStores()."]
    pub fn storeId(this: &Cookie) -> ::js_sys::JsString;
    # [wasm_bindgen (method , getter , js_class = Cookie)]
    #[doc = "The value of the cookie."]
    pub fn value(this: &Cookie) -> ::js_sys::JsString;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "cookies.CookieStore" , typescript_type = "cookies.CookieStore")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Represents a cookie store in the browser. An incognito mode window, for instance, uses a separate cookie store from a non-incognito window."]
    pub type CookieStore;
    # [wasm_bindgen (method , getter , js_class = CookieStore)]
    #[doc = "The unique identifier for the cookie store."]
    pub fn id(this: &CookieStore) -> ::js_sys::JsString;
    # [wasm_bindgen (method , getter , js_class = CookieStore)]
    #[doc = "Identifiers of all the browser tabs that share this cookie store."]
    pub fn tabIds(this: &CookieStore) -> ::js_sys::Array;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "cookies.OnChangedCause" , typescript_type = "cookies.OnChangedCause")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The underlying reason behind the cookie's change. If a cookie was inserted, or removed via an explicit call to \"chrome.cookies.remove\", \"cause\" will be \"explicit\". If a cookie was automatically removed due to expiry, \"cause\" will be \"expired\". If a cookie was removed due to being overwritten with an already-expired expiration date, \"cause\" will be set to \"expired_overwrite\".  If a cookie was automatically removed due to garbage collection, \"cause\" will be \"evicted\".  If a cookie was automatically removed due to a \"set\" call that overwrote it, \"cause\" will be \"overwrite\". Plan your response accordingly."]
    pub type OnChangedCause;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "cookies.CookieDetails" , typescript_type = "cookies.CookieDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Details to identify the cookie."]
    pub type CookieDetails;
    # [wasm_bindgen (method , getter , js_class = CookieDetails)]
    #[doc = "The name of the cookie to access."]
    pub fn name(this: &CookieDetails) -> ::js_sys::JsString;
    # [wasm_bindgen (method , getter , js_class = CookieDetails)]
    #[doc = "The partition key for reading or modifying cookies with the Partitioned attribute."]
    pub fn partitionKey(this: &CookieDetails) -> Option<CookiePartitionKey>;
    # [wasm_bindgen (method , getter , js_class = CookieDetails)]
    #[doc = "The ID of the cookie store in which to look for the cookie. By default, the current execution context's cookie store will be used."]
    pub fn storeId(this: &CookieDetails) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = CookieDetails)]
    #[doc = "The URL with which the cookie to access is associated. This argument may be a full URL, in which case any data following the URL path (e.g. the query string) is simply ignored. If host permissions for this URL are not specified in the manifest file, the API call will fail."]
    pub fn url(this: &CookieDetails) -> ::js_sys::JsString;
    #[doc = "Retrieves information about a single cookie. If more than one cookie of the same name exists for the given URL, the one with the longest path will be returned. For cookies with the same path length, the cookie with the earliest creation time will be returned."]
    #[wasm_bindgen(js_name = "cookies.get", catch)]
    pub async fn get(
        details: CookieDetails,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Retrieves all cookies from a single cookie store that match the given information. The cookies returned will be sorted, with those with the longest path first. If multiple cookies have the same path length, those with the earliest creation time will be first. This method only retrieves cookies for domains that the extension has host permissions to."]
    #[wasm_bindgen(js_name = "cookies.getAll", catch)]
    pub async fn getAll(
        details: ::js_sys::Object,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Sets a cookie with the given cookie data; may overwrite equivalent cookies if they exist."]
    #[wasm_bindgen(js_name = "cookies.set", catch)]
    pub async fn set(
        details: ::js_sys::Object,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Deletes a cookie by name."]
    #[wasm_bindgen(js_name = "cookies.remove", catch)]
    pub async fn remove(
        details: CookieDetails,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Lists all existing cookie stores."]
    #[wasm_bindgen(js_name = "cookies.getAllCookieStores", catch)]
    pub async fn getAllCookieStores() -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
}
#[wasm_bindgen]
pub async fn cookies_get(
    details: CookieDetails,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    get(details).await
}
#[wasm_bindgen]
pub async fn cookies_get_all(
    details: ::js_sys::Object,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    getAll(details).await
}
#[wasm_bindgen]
pub async fn cookies_set(
    details: ::js_sys::Object,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    set(details).await
}
#[wasm_bindgen]
pub async fn cookies_remove(
    details: CookieDetails,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    remove(details).await
}
#[wasm_bindgen]
pub async fn cookies_get_all_cookie_stores(
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    getAllCookieStores().await
}
