#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "none"]
# [wasm_bindgen (js_namespace = chrome)]
extern "C" {
    #[doc = "Attaches the specified url to the AppView with the provided instance ID."]
    #[wasm_bindgen(js_name = "appViewGuestInternal.attachFrame", catch)]
    pub async fn attach_frame(
        url: ::js_sys::JsString,
        guest_instance_id: ::js_sys::Number,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Attaches the specified url to the AppView with the provided instance ID."]
    #[wasm_bindgen(js_name = "appViewGuestInternal.attachFrame")]
    pub fn attach_frame_callback(
        url: ::js_sys::JsString,
        guest_instance_id: ::js_sys::Number,
        callback: &::js_sys::Function,
    );
}
#[wasm_bindgen]
pub async fn app_view_guest_internal_attach_frame(
    url: ::js_sys::JsString,
    guest_instance_id: ::js_sys::Number,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    attach_frame(url, guest_instance_id).await
}
#[wasm_bindgen]
pub fn app_view_guest_internal_attach_frame_callback(
    url: ::js_sys::JsString,
    guest_instance_id: ::js_sys::Number,
    callback: &::js_sys::Function,
) {
    attach_frame_callback(url, guest_instance_id, callback);
}
