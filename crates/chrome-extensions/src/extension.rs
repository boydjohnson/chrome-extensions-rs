#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "The <code>chrome.extension</code> API has utilities that can be used by any extension page. It includes support for exchanging messages between an extension and its content scripts or between extensions, as described in detail in <a href='messaging'>Message Passing</a>."]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "ViewType" , typescript_type = "ViewType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The type of extension view."]
    pub type ViewType;
}
