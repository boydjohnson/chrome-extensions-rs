#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>chrome.accessibilityFeatures</code> API to manage Chrome's accessibility features. This API relies on the <a href='types#ChromeSetting'>ChromeSetting prototype of the type API</a> for getting and setting individual accessibility features. In order to get feature states the extension must request <code>accessibilityFeatures.read</code> permission. For modifying feature state, the extension needs <code>accessibilityFeatures.modify</code> permission. Note that <code>accessibilityFeatures.modify</code> does not imply <code>accessibilityFeatures.read</code> permission."]
#[wasm_bindgen]
extern "C" {}
