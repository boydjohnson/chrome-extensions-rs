#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use <code>chrome.instanceID</code> to access the Instance ID service."]
# [wasm_bindgen (js_namespace = chrome)]
extern "C" {
    #[doc = "Retrieves an identifier for the app instance. The instance ID will be returned by the <code>callback</code>. The same ID will be returned as long as the application identity has not been revoked or expired."]
    #[wasm_bindgen(js_name = "instanceID.getID", catch)]
    pub async fn get_id() -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Retrieves an identifier for the app instance. The instance ID will be returned by the <code>callback</code>. The same ID will be returned as long as the application identity has not been revoked or expired."]
    #[wasm_bindgen(js_name = "instanceID.getID")]
    pub fn get_id_callback(callback: &::js_sys::Function);
    #[doc = "Retrieves the time when the InstanceID has been generated. The creation time will be returned by the <code>callback</code>."]
    #[wasm_bindgen(js_name = "instanceID.getCreationTime", catch)]
    pub async fn get_creation_time() -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Retrieves the time when the InstanceID has been generated. The creation time will be returned by the <code>callback</code>."]
    #[wasm_bindgen(js_name = "instanceID.getCreationTime")]
    pub fn get_creation_time_callback(callback: &::js_sys::Function);
    #[doc = "Return a token that allows the authorized entity to access the service defined by scope."]
    #[wasm_bindgen(js_name = "instanceID.getToken", catch)]
    pub async fn get_token(
        get_token_params: ::js_sys::Object,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Return a token that allows the authorized entity to access the service defined by scope."]
    #[wasm_bindgen(js_name = "instanceID.getToken")]
    pub fn get_token_callback(get_token_params: ::js_sys::Object, callback: &::js_sys::Function);
    #[doc = "Revokes a granted token."]
    #[wasm_bindgen(js_name = "instanceID.deleteToken", catch)]
    pub async fn delete_token(
        delete_token_params: ::js_sys::Object,
    ) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Revokes a granted token."]
    #[wasm_bindgen(js_name = "instanceID.deleteToken")]
    pub fn delete_token_callback(
        delete_token_params: ::js_sys::Object,
        callback: &::js_sys::Function,
    );
    #[doc = "Resets the app instance identifier and revokes all tokens associated with it."]
    #[wasm_bindgen(js_name = "instanceID.deleteID", catch)]
    pub async fn delete_id() -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Resets the app instance identifier and revokes all tokens associated with it."]
    #[wasm_bindgen(js_name = "instanceID.deleteID")]
    pub fn delete_id_callback(callback: &::js_sys::Function);
}
#[wasm_bindgen]
pub async fn instance_id_get_id() -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    get_id().await
}
#[wasm_bindgen]
pub fn instance_id_get_id_callback(callback: &::js_sys::Function) {
    get_id_callback(callback);
}
#[wasm_bindgen]
pub async fn instance_id_get_creation_time(
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    get_creation_time().await
}
#[wasm_bindgen]
pub fn instance_id_get_creation_time_callback(callback: &::js_sys::Function) {
    get_creation_time_callback(callback);
}
#[wasm_bindgen]
pub async fn instance_id_get_token(
    get_token_params: ::js_sys::Object,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    get_token(get_token_params).await
}
#[wasm_bindgen]
pub fn instance_id_get_token_callback(
    get_token_params: ::js_sys::Object,
    callback: &::js_sys::Function,
) {
    get_token_callback(get_token_params, callback);
}
#[wasm_bindgen]
pub async fn instance_id_delete_token(
    delete_token_params: ::js_sys::Object,
) -> Result<(), ::wasm_bindgen::JsValue> {
    delete_token(delete_token_params).await
}
#[wasm_bindgen]
pub fn instance_id_delete_token_callback(
    delete_token_params: ::js_sys::Object,
    callback: &::js_sys::Function,
) {
    delete_token_callback(delete_token_params, callback);
}
#[wasm_bindgen]
pub async fn instance_id_delete_id() -> Result<(), ::wasm_bindgen::JsValue> {
    delete_id().await
}
#[wasm_bindgen]
pub fn instance_id_delete_id_callback(callback: &::js_sys::Function) {
    delete_id_callback(callback);
}
