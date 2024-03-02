#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>chrome.permissions</code> API to request <a href='/docs/extensions/develop/concepts/declare-permissions'>declared optional permissions</a> at run time rather than install time, so users understand why the permissions are needed and grant only those that are necessary."]
# [wasm_bindgen (js_namespace = chrome)]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "permissions.Permissions" , typescript_type = "permissions.Permissions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type Permissions;
    # [wasm_bindgen (method , getter , js_class = Permissions)]
    #[doc = "The list of host permissions, including those specified in the <code>optional_permissions</code> or <code>permissions</code> keys in the manifest, and those associated with <a href='/docs/extensions/develop/concepts/content-scripts'>Content Scripts</a>."]
    pub fn origins(this: &Permissions) -> Option<::js_sys::Array>;
    # [wasm_bindgen (method , getter , js_class = Permissions)]
    #[doc = "List of named permissions (does not include hosts or origins)."]
    pub fn permissions(this: &Permissions) -> Option<::js_sys::Array>;
    #[doc = "Gets the extension's current set of permissions."]
    #[wasm_bindgen(js_name = "permissions.getAll", catch)]
    pub async fn getAll() -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Checks if the extension has the specified permissions."]
    #[wasm_bindgen(js_name = "permissions.contains", catch)]
    pub async fn contains(
        permissions: Permissions,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Requests access to the specified permissions, displaying a prompt to the user if necessary. These permissions must either be defined in the <code>optional_permissions</code> field of the manifest or be required permissions that were withheld by the user. Paths on origin patterns will be ignored. You can request subsets of optional origin permissions; for example, if you specify <code>*://*/*</code> in the <code>optional_permissions</code> section of the manifest, you can request <code>http://example.com/</code>. If there are any problems requesting the permissions, $(ref:runtime.lastError) will be set."]
    #[wasm_bindgen(js_name = "permissions.request", catch)]
    pub async fn request(
        permissions: Permissions,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Removes access to the specified permissions. If there are any problems removing the permissions, $(ref:runtime.lastError) will be set."]
    #[wasm_bindgen(js_name = "permissions.remove", catch)]
    pub async fn remove(
        permissions: Permissions,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
}
#[wasm_bindgen]
pub async fn permissions_get_all() -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    getAll().await
}
#[wasm_bindgen]
pub async fn permissions_contains(
    permissions: Permissions,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    contains(permissions).await
}
#[wasm_bindgen]
pub async fn permissions_request(
    permissions: Permissions,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    request(permissions).await
}
#[wasm_bindgen]
pub async fn permissions_remove(
    permissions: Permissions,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    remove(permissions).await
}
