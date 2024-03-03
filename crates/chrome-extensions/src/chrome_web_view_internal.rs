#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "none"]
# [wasm_bindgen (js_namespace = chrome)]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "chromeWebViewInternal.ContextMenuItem" , typescript_type = "chromeWebViewInternal.ContextMenuItem")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "An item in the context menu."]
    pub type ContextMenuItem;
    # [wasm_bindgen (method , getter , js_class = ContextMenuItem)]
    #[doc = "id of the input item"]
    pub fn commandId(this: &ContextMenuItem) -> ::js_sys::Number;
    # [wasm_bindgen (method , getter , js_class = ContextMenuItem)]
    #[doc = "label of the item"]
    pub fn label(this: &ContextMenuItem) -> Option<::js_sys::JsString>;
    #[doc = ""]
    #[wasm_bindgen(js_name = "chromeWebViewInternal.contextMenusCreate")]
    pub fn context_menus_create(instance_id: ::js_sys::Number, create_properties: ::js_sys::Object);
    #[doc = "Updates a previously created context menu item."]
    #[wasm_bindgen(js_name = "chromeWebViewInternal.contextMenusUpdate", catch)]
    pub async fn context_menus_update(
        instance_id: ::js_sys::Number,
        id: ::wasm_bindgen::JsValue,
        update_properties: ::js_sys::Object,
    ) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Updates a previously created context menu item."]
    #[wasm_bindgen(js_name = "chromeWebViewInternal.contextMenusUpdate")]
    pub fn context_menus_update_callback(
        instance_id: ::js_sys::Number,
        id: ::wasm_bindgen::JsValue,
        update_properties: ::js_sys::Object,
        callback: &::js_sys::Function,
    );
    #[doc = "Removes a context menu item."]
    #[wasm_bindgen(js_name = "chromeWebViewInternal.contextMenusRemove", catch)]
    pub async fn context_menus_remove(
        instance_id: ::js_sys::Number,
        menu_item_id: ::wasm_bindgen::JsValue,
    ) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Removes a context menu item."]
    #[wasm_bindgen(js_name = "chromeWebViewInternal.contextMenusRemove")]
    pub fn context_menus_remove_callback(
        instance_id: ::js_sys::Number,
        menu_item_id: ::wasm_bindgen::JsValue,
        callback: &::js_sys::Function,
    );
    #[doc = "Removes all context menu items added by this webview."]
    #[wasm_bindgen(js_name = "chromeWebViewInternal.contextMenusRemoveAll", catch)]
    pub async fn context_menus_remove_all(
        instance_id: ::js_sys::Number,
    ) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Removes all context menu items added by this webview."]
    #[wasm_bindgen(js_name = "chromeWebViewInternal.contextMenusRemoveAll")]
    pub fn context_menus_remove_all_callback(
        instance_id: ::js_sys::Number,
        callback: &::js_sys::Function,
    );
}
#[wasm_bindgen]
pub fn chrome_web_view_internal_context_menus_create(
    instance_id: ::js_sys::Number,
    create_properties: ::js_sys::Object,
) {
    context_menus_create(instance_id, create_properties)
}
#[wasm_bindgen]
pub async fn chrome_web_view_internal_context_menus_update(
    instance_id: ::js_sys::Number,
    id: ::wasm_bindgen::JsValue,
    update_properties: ::js_sys::Object,
) -> Result<(), ::wasm_bindgen::JsValue> {
    context_menus_update(instance_id, id, update_properties).await
}
#[wasm_bindgen]
pub fn chrome_web_view_internal_context_menus_update_callback(
    instance_id: ::js_sys::Number,
    id: ::wasm_bindgen::JsValue,
    update_properties: ::js_sys::Object,
    callback: &::js_sys::Function,
) {
    context_menus_update_callback(instance_id, id, update_properties, callback);
}
#[wasm_bindgen]
pub async fn chrome_web_view_internal_context_menus_remove(
    instance_id: ::js_sys::Number,
    menu_item_id: ::wasm_bindgen::JsValue,
) -> Result<(), ::wasm_bindgen::JsValue> {
    context_menus_remove(instance_id, menu_item_id).await
}
#[wasm_bindgen]
pub fn chrome_web_view_internal_context_menus_remove_callback(
    instance_id: ::js_sys::Number,
    menu_item_id: ::wasm_bindgen::JsValue,
    callback: &::js_sys::Function,
) {
    context_menus_remove_callback(instance_id, menu_item_id, callback);
}
#[wasm_bindgen]
pub async fn chrome_web_view_internal_context_menus_remove_all(
    instance_id: ::js_sys::Number,
) -> Result<(), ::wasm_bindgen::JsValue> {
    context_menus_remove_all(instance_id).await
}
#[wasm_bindgen]
pub fn chrome_web_view_internal_context_menus_remove_all_callback(
    instance_id: ::js_sys::Number,
    callback: &::js_sys::Function,
) {
    context_menus_remove_all_callback(instance_id, callback);
}
