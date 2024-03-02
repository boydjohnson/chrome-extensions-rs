#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use <code>chrome.instanceID</code> to access the Instance ID service."]
# [wasm_bindgen (js_namespace = chrome)]
extern "C" {
    #[doc = "Retrieves an identifier for the app instance. The instance ID will be returned by the <code>callback</code>. The same ID will be returned as long as the application identity has not been revoked or expired."]
    #[wasm_bindgen(js_name = "instanceID.getID", catch)]
    pub async fn getID() -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Retrieves the time when the InstanceID has been generated. The creation time will be returned by the <code>callback</code>."]
    #[wasm_bindgen(js_name = "instanceID.getCreationTime", catch)]
    pub async fn getCreationTime() -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Return a token that allows the authorized entity to access the service defined by scope."]
    #[wasm_bindgen(js_name = "instanceID.getToken", catch)]
    pub async fn getToken(
        getTokenParams: ::js_sys::Object,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Revokes a granted token."]
    #[wasm_bindgen(js_name = "instanceID.deleteToken", catch)]
    pub async fn deleteToken(
        deleteTokenParams: ::js_sys::Object,
    ) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Resets the app instance identifier and revokes all tokens associated with it."]
    #[wasm_bindgen(js_name = "instanceID.deleteID", catch)]
    pub async fn deleteID() -> Result<(), ::wasm_bindgen::JsValue>;
}
#[wasm_bindgen]
pub async fn instance_id_get_id() -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    getID().await
}
#[wasm_bindgen]
pub async fn instance_id_get_creation_time(
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    getCreationTime().await
}
#[wasm_bindgen]
pub async fn instance_id_get_token(
    getTokenParams: ::js_sys::Object,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    getToken(getTokenParams).await
}
#[wasm_bindgen]
pub async fn instance_id_delete_token(
    deleteTokenParams: ::js_sys::Object,
) -> Result<(), ::wasm_bindgen::JsValue> {
    deleteToken(deleteTokenParams).await
}
#[wasm_bindgen]
pub async fn instance_id_delete_id() -> Result<(), ::wasm_bindgen::JsValue> {
    deleteID().await
}
