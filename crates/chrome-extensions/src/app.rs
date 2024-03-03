#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "none"]
# [wasm_bindgen (js_namespace = chrome)]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "app.Details" , typescript_type = "app.Details")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "TODO (it's a manifest)"]
    pub type Details;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "app.DOMWindow" , typescript_type = "app.DOMWindow")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type DOMWindow;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "app.InstallState" , typescript_type = "app.InstallState")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type InstallState;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "app.RunningState" , typescript_type = "app.RunningState")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type RunningState;
    #[doc = "TODO"]
    #[wasm_bindgen(js_name = "app.getIsInstalled")]
    pub fn get_is_installed() -> ::js_sys::Boolean;
    #[doc = "TODO"]
    #[wasm_bindgen(js_name = "app.installState", catch)]
    pub async fn install_state() -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "TODO"]
    #[wasm_bindgen(js_name = "app.runningState")]
    pub fn running_state() -> RunningState;
    #[doc = "TODO"]
    #[wasm_bindgen(js_name = "app.getDetails")]
    pub fn get_details() -> Details;
}
#[wasm_bindgen]
pub fn app_get_is_installed() -> ::js_sys::Boolean {
    get_is_installed()
}
#[wasm_bindgen]
pub async fn app_install_state() -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    install_state().await
}
#[wasm_bindgen]
pub fn app_running_state() -> RunningState {
    running_state()
}
#[wasm_bindgen]
pub fn app_get_details() -> Details {
    get_details()
}
