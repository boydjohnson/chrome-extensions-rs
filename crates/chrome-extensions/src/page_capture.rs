#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>chrome.pageCapture</code> API to save a tab as MHTML."]
# [wasm_bindgen (js_namespace = chrome)]
extern "C" {
    #[doc = "Saves the content of the tab with given id as MHTML."]
    #[wasm_bindgen(js_name = "pageCapture.saveAsMHTML", catch)]
    pub async fn save_as_mhtml(
        details: ::js_sys::Object,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Saves the content of the tab with given id as MHTML."]
    #[wasm_bindgen(js_name = "pageCapture.saveAsMHTML")]
    pub fn save_as_mhtml_callback(details: ::js_sys::Object, callback: &::js_sys::Function);
}
#[wasm_bindgen]
pub async fn page_capture_save_as_mhtml(
    details: ::js_sys::Object,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    save_as_mhtml(details).await
}
#[wasm_bindgen]
pub fn page_capture_save_as_mhtml_callback(
    details: ::js_sys::Object,
    callback: &::js_sys::Function,
) {
    save_as_mhtml_callback(details, callback);
}
