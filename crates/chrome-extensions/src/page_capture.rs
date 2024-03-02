#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>chrome.pageCapture</code> API to save a tab as MHTML."]
# [wasm_bindgen (js_namespace = chrome)]
extern "C" {
    #[doc = "Saves the content of the tab with given id as MHTML."]
    #[wasm_bindgen(js_name = "pageCapture.saveAsMHTML", catch)]
    pub async fn saveAsMHTML(
        details: ::js_sys::Object,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
}
#[wasm_bindgen]
pub async fn page_capture_save_as_mhtml(
    details: ::js_sys::Object,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    saveAsMHTML(details).await
}
