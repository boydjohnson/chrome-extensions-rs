#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>chrome.idle</code> API to detect when the machine's idle state changes."]
# [wasm_bindgen (js_namespace = chrome)]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "idle.IdleState" , typescript_type = "idle.IdleState")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type IdleState;
    #[doc = "Returns \"locked\" if the system is locked, \"idle\" if the user has not generated any input for a specified number of seconds, or \"active\" otherwise."]
    #[wasm_bindgen(js_name = "idle.queryState", catch)]
    pub async fn queryState(
        detectionIntervalInSeconds: ::js_sys::Number,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Gets the time, in seconds, it takes until the screen is locked automatically while idle. Returns a zero duration if the screen is never locked automatically. Currently supported on Chrome OS only."]
    #[wasm_bindgen(js_name = "idle.getAutoLockDelay", catch)]
    pub async fn getAutoLockDelay() -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
}
#[wasm_bindgen]
pub async fn idle_query_state(
    detectionIntervalInSeconds: ::js_sys::Number,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    queryState(detectionIntervalInSeconds).await
}
#[wasm_bindgen]
pub async fn idle_get_auto_lock_delay() -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>
{
    getAutoLockDelay().await
}
