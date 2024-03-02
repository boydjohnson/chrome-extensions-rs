#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "none"]
# [wasm_bindgen (js_namespace = chrome)]
extern "C" {
    #[doc = "Attaches the specified url to the AppView with the provided instance ID."]
    #[wasm_bindgen(js_name = "appViewGuestInternal.attachFrame", catch)]
    pub async fn attachFrame(
        url: ::js_sys::JsString,
        guestInstanceId: ::js_sys::Number,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
}
#[wasm_bindgen]
pub async fn app_view_guest_internal_attach_frame(
    url: ::js_sys::JsString,
    guestInstanceId: ::js_sys::Number,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    attachFrame(url, guestInstanceId).await
}
