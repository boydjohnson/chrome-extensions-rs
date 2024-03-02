#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use <code>chrome.gcm</code> to enable apps and extensions to send and receive messages through <a href='https://firebase.google.com/docs/cloud-messaging/'>Firebase Cloud Messaging</a> (FCM)."]
# [wasm_bindgen (js_namespace = chrome)]
extern "C" {
    #[doc = "Registers the application with FCM. The registration ID will be returned by the <code>callback</code>. If <code>register</code> is called again with the same list of <code>senderIds</code>, the same registration ID will be returned."]
    #[wasm_bindgen(js_name = "gcm.register", catch)]
    pub async fn register(
        senderIds: ::js_sys::Array,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Unregisters the application from FCM."]
    #[wasm_bindgen(js_name = "gcm.unregister", catch)]
    pub async fn unregister() -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Sends a message according to its contents."]
    #[wasm_bindgen(js_name = "gcm.send", catch)]
    pub async fn send(
        message: ::js_sys::Object,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
}
#[wasm_bindgen]
pub async fn gcm_register(
    senderIds: ::js_sys::Array,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    register(senderIds).await
}
#[wasm_bindgen]
pub async fn gcm_unregister() -> Result<(), ::wasm_bindgen::JsValue> {
    unregister().await
}
#[wasm_bindgen]
pub async fn gcm_send(
    message: ::js_sys::Object,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    send(message).await
}
