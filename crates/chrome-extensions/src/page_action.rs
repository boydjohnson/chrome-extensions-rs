#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>chrome.pageAction</code> API to put icons in the main Google Chrome toolbar, to the right of the address bar. Page actions represent actions that can be taken on the current page, but that aren't applicable to all pages. Page actions appear grayed out when inactive."]
# [wasm_bindgen (js_namespace = chrome)]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "pageAction.ImageDataType" , typescript_type = "pageAction.ImageDataType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Pixel data for an image. Must be an ImageData object (for example, from a <code>canvas</code> element)."]
    pub type ImageDataType;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "pageAction.TabDetails" , typescript_type = "pageAction.TabDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type TabDetails;
    # [wasm_bindgen (method , getter , js_class = TabDetails)]
    #[doc = "The ID of the tab to query state for. If no tab is specified, the non-tab-specific state is returned."]
    pub fn tabId(this: &TabDetails) -> Option<::js_sys::Number>;
    #[doc = "Shows the page action. The page action is shown whenever the tab is selected."]
    #[wasm_bindgen(js_name = "pageAction.show", catch)]
    pub async fn show(tab_id: ::js_sys::Number) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Shows the page action. The page action is shown whenever the tab is selected."]
    #[wasm_bindgen(js_name = "pageAction.show")]
    pub fn show_callback(tab_id: ::js_sys::Number, callback: &::js_sys::Function);
    #[doc = "Hides the page action. Hidden page actions still appear in the Chrome toolbar, but are grayed out."]
    #[wasm_bindgen(js_name = "pageAction.hide", catch)]
    pub async fn hide(tab_id: ::js_sys::Number) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Hides the page action. Hidden page actions still appear in the Chrome toolbar, but are grayed out."]
    #[wasm_bindgen(js_name = "pageAction.hide")]
    pub fn hide_callback(tab_id: ::js_sys::Number, callback: &::js_sys::Function);
    #[doc = "Sets the title of the page action. This is displayed in a tooltip over the page action."]
    #[wasm_bindgen(js_name = "pageAction.setTitle", catch)]
    pub async fn set_title(details: ::js_sys::Object) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Sets the title of the page action. This is displayed in a tooltip over the page action."]
    #[wasm_bindgen(js_name = "pageAction.setTitle")]
    pub fn set_title_callback(details: ::js_sys::Object, callback: &::js_sys::Function);
    #[doc = "Gets the title of the page action."]
    #[wasm_bindgen(js_name = "pageAction.getTitle", catch)]
    pub async fn get_title(
        details: TabDetails,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Gets the title of the page action."]
    #[wasm_bindgen(js_name = "pageAction.getTitle")]
    pub fn get_title_callback(details: TabDetails, callback: &::js_sys::Function);
    #[doc = "Sets the icon for the page action. The icon can be specified either as the path to an image file or as the pixel data from a canvas element, or as dictionary of either one of those. Either the <b>path</b> or the <b>imageData</b> property must be specified."]
    #[wasm_bindgen(js_name = "pageAction.setIcon", catch)]
    pub async fn set_icon(details: ::js_sys::Object) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Sets the icon for the page action. The icon can be specified either as the path to an image file or as the pixel data from a canvas element, or as dictionary of either one of those. Either the <b>path</b> or the <b>imageData</b> property must be specified."]
    #[wasm_bindgen(js_name = "pageAction.setIcon")]
    pub fn set_icon_callback(details: ::js_sys::Object, callback: &::js_sys::Function);
    #[doc = "Sets the HTML document to be opened as a popup when the user clicks on the page action's icon."]
    #[wasm_bindgen(js_name = "pageAction.setPopup", catch)]
    pub async fn set_popup(details: ::js_sys::Object) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Sets the HTML document to be opened as a popup when the user clicks on the page action's icon."]
    #[wasm_bindgen(js_name = "pageAction.setPopup")]
    pub fn set_popup_callback(details: ::js_sys::Object, callback: &::js_sys::Function);
    #[doc = "Gets the html document set as the popup for this page action."]
    #[wasm_bindgen(js_name = "pageAction.getPopup", catch)]
    pub async fn get_popup(
        details: TabDetails,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Gets the html document set as the popup for this page action."]
    #[wasm_bindgen(js_name = "pageAction.getPopup")]
    pub fn get_popup_callback(details: TabDetails, callback: &::js_sys::Function);
}
#[wasm_bindgen]
pub async fn page_action_show(tab_id: ::js_sys::Number) -> Result<(), ::wasm_bindgen::JsValue> {
    show(tab_id).await
}
#[wasm_bindgen]
pub fn page_action_show_callback(tab_id: ::js_sys::Number, callback: &::js_sys::Function) {
    show_callback(tab_id, callback);
}
#[wasm_bindgen]
pub async fn page_action_hide(tab_id: ::js_sys::Number) -> Result<(), ::wasm_bindgen::JsValue> {
    hide(tab_id).await
}
#[wasm_bindgen]
pub fn page_action_hide_callback(tab_id: ::js_sys::Number, callback: &::js_sys::Function) {
    hide_callback(tab_id, callback);
}
#[wasm_bindgen]
pub async fn page_action_set_title(
    details: ::js_sys::Object,
) -> Result<(), ::wasm_bindgen::JsValue> {
    set_title(details).await
}
#[wasm_bindgen]
pub fn page_action_set_title_callback(details: ::js_sys::Object, callback: &::js_sys::Function) {
    set_title_callback(details, callback);
}
#[wasm_bindgen]
pub async fn page_action_get_title(
    details: TabDetails,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    get_title(details).await
}
#[wasm_bindgen]
pub fn page_action_get_title_callback(details: TabDetails, callback: &::js_sys::Function) {
    get_title_callback(details, callback);
}
#[wasm_bindgen]
pub async fn page_action_set_icon(
    details: ::js_sys::Object,
) -> Result<(), ::wasm_bindgen::JsValue> {
    set_icon(details).await
}
#[wasm_bindgen]
pub fn page_action_set_icon_callback(details: ::js_sys::Object, callback: &::js_sys::Function) {
    set_icon_callback(details, callback);
}
#[wasm_bindgen]
pub async fn page_action_set_popup(
    details: ::js_sys::Object,
) -> Result<(), ::wasm_bindgen::JsValue> {
    set_popup(details).await
}
#[wasm_bindgen]
pub fn page_action_set_popup_callback(details: ::js_sys::Object, callback: &::js_sys::Function) {
    set_popup_callback(details, callback);
}
#[wasm_bindgen]
pub async fn page_action_get_popup(
    details: TabDetails,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    get_popup(details).await
}
#[wasm_bindgen]
pub fn page_action_get_popup_callback(details: TabDetails, callback: &::js_sys::Function) {
    get_popup_callback(details, callback);
}
