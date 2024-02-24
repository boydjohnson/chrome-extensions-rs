#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>chrome.permissions</code> API to request <a href='/docs/extensions/develop/concepts/declare-permissions'>declared optional permissions</a> at run time rather than install time, so users understand why the permissions are needed and grant only those that are necessary."]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "Permissions" , typescript_type = "Permissions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type Permissions;
    # [wasm_bindgen (method , getter , js_class = Permissions)]
    #[doc = "The list of host permissions, including those specified in the <code>optional_permissions</code> or <code>permissions</code> keys in the manifest, and those associated with <a href='/docs/extensions/develop/concepts/content-scripts'>Content Scripts</a>."]
    pub fn origins(this: &Permissions) -> Option<::js_sys::Array>;
    # [wasm_bindgen (method , getter , js_class = Permissions)]
    #[doc = "List of named permissions (does not include hosts or origins)."]
    pub fn permissions(this: &Permissions) -> Option<::js_sys::Array>;
}
