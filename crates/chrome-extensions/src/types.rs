#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "The <code>chrome.types</code> API contains type declarations for Chrome."]
# [wasm_bindgen (js_namespace = chrome)]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "types.ChromeSettingScope" , typescript_type = "types.ChromeSettingScope")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The scope of the ChromeSetting. One of<ul><li><var>regular</var>: setting for the regular profile (which is inherited by the incognito profile if not overridden elsewhere),</li><li><var>regular_only</var>: setting for the regular profile only (not inherited by the incognito profile),</li><li><var>incognito_persistent</var>: setting for the incognito profile that survives browser restarts (overrides regular preferences),</li><li><var>incognito_session_only</var>: setting for the incognito profile that can only be set during an incognito session and is deleted when the incognito session ends (overrides regular and incognito_persistent preferences).</li></ul>"]
    pub type ChromeSettingScope;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "types.LevelOfControl" , typescript_type = "types.LevelOfControl")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "One of<ul><li><var>not_controllable</var>: cannot be controlled by any extension</li><li><var>controlled_by_other_extensions</var>: controlled by extensions with higher precedence</li><li><var>controllable_by_this_extension</var>: can be controlled by this extension</li><li><var>controlled_by_this_extension</var>: controlled by this extension</li></ul>"]
    pub type LevelOfControl;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "types.ChromeSetting" , typescript_type = "types.ChromeSetting")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "An interface that allows access to a Chrome browser setting. See $(ref:accessibilityFeatures) for an example."]
    pub type ChromeSetting;
}
