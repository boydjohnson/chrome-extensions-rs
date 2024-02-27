#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Dummy namespace for the incognito manifest key."]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "IncognitoMode" , typescript_type = "IncognitoMode")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type IncognitoMode;
}
