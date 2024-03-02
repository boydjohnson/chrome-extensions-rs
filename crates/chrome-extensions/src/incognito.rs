#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Dummy namespace for the incognito manifest key."]
# [wasm_bindgen (js_namespace = chrome)]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "incognito.IncognitoMode" , typescript_type = "incognito.IncognitoMode")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type IncognitoMode;
}
