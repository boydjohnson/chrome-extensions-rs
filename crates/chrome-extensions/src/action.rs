#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>chrome.action</code> API to control the extension's icon in the Google Chrome toolbar."]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "TabDetails" , typescript_type = "TabDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type TabDetails;
    # [wasm_bindgen (method , getter , js_class = TabDetails)]
    #[doc = "The ID of the tab to query state for. If no tab is specified, the non-tab-specific state is returned."]
    pub fn tabId(this: &TabDetails) -> Option<::js_sys::Number>;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "UserSettings" , typescript_type = "UserSettings")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The collection of user-specified settings relating to an extension's action."]
    pub type UserSettings;
    # [wasm_bindgen (method , getter , js_class = UserSettings)]
    #[doc = "Whether the extension's action icon is visible on browser windows' top-level toolbar (i.e., whether the extension has been 'pinned' by the user)."]
    pub fn isOnToolbar(this: &UserSettings) -> ::js_sys::Boolean;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "OpenPopupOptions" , typescript_type = "OpenPopupOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type OpenPopupOptions;
    # [wasm_bindgen (method , getter , js_class = OpenPopupOptions)]
    #[doc = "The ID of the window to open the action popup in. Defaults to the currently-active window if unspecified."]
    pub fn windowId(this: &OpenPopupOptions) -> Option<::js_sys::Number>;
}
