#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>chrome.webNavigation</code> API to receive notifications about the status of navigation requests in-flight."]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "TransitionType" , typescript_type = "TransitionType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Cause of the navigation. The same transition types as defined in the history API are used. These are the same transition types as defined in the <a href=\"history#transition_types\">history API</a> except with <code>\"start_page\"</code> in place of <code>\"auto_toplevel\"</code> (for backwards compatibility)."]
    pub type TransitionType;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "TransitionQualifier" , typescript_type = "TransitionQualifier")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type TransitionQualifier;
}
