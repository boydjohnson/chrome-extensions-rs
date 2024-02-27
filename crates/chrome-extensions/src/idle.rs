#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>chrome.idle</code> API to detect when the machine's idle state changes."]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "IdleState" , typescript_type = "IdleState")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type IdleState;
}
