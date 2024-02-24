#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "none"]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "Details" , typescript_type = "Details")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "TODO (it's a manifest)"]
    pub type Details;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "DOMWindow" , typescript_type = "DOMWindow")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type DOMWindow;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "InstallState" , typescript_type = "InstallState")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type InstallState;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "RunningState" , typescript_type = "RunningState")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type RunningState;
}
