#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>chrome.privacy</code> API to control usage of the features in Chrome that can affect a user's privacy. This API relies on the <a href='types#ChromeSetting'>ChromeSetting prototype of the type API</a> for getting and setting Chrome's configuration."]
# [wasm_bindgen (js_namespace = chrome)]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "privacy.IPHandlingPolicy" , typescript_type = "privacy.IPHandlingPolicy")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The IP handling policy of WebRTC."]
    pub type IPHandlingPolicy;
}
