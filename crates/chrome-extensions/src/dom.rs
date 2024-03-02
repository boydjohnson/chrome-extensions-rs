#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>chrome.dom</code> API to access special DOM APIs for Extensions"]
# [wasm_bindgen (js_namespace = chrome)]
extern "C" {
    #[doc = "Gets the open shadow root or the closed shadow root hosted by the specified element. If the element doesn't attach the shadow root, it will return null."]
    #[wasm_bindgen(js_name = "dom.openOrClosedShadowRoot")]
    pub fn openOrClosedShadowRoot(element: ::js_sys::Object) -> ::js_sys::Object;
}
#[wasm_bindgen]
pub fn dom_open_or_closed_shadow_root(element: ::js_sys::Object) -> ::js_sys::Object {
    openOrClosedShadowRoot(element)
}
