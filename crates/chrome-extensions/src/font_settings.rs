#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>chrome.fontSettings</code> API to manage Chrome's font settings."]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "FontName" , typescript_type = "FontName")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Represents a font name."]
    pub type FontName;
    # [wasm_bindgen (method , getter , js_class = FontName)]
    #[doc = "The display name of the font."]
    pub fn displayName(this: &FontName) -> ::js_sys::JsString;
    # [wasm_bindgen (method , getter , js_class = FontName)]
    #[doc = "The font ID."]
    pub fn fontId(this: &FontName) -> ::js_sys::JsString;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "ScriptCode" , typescript_type = "ScriptCode")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "An ISO 15924 script code. The default, or global, script is represented by script code \"Zyyy\"."]
    pub type ScriptCode;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "GenericFamily" , typescript_type = "GenericFamily")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "A CSS generic font family."]
    pub type GenericFamily;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "LevelOfControl" , typescript_type = "LevelOfControl")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "One of<br><var>not_controllable</var>: cannot be controlled by any extension<br><var>controlled_by_other_extensions</var>: controlled by extensions with higher precedence<br><var>controllable_by_this_extension</var>: can be controlled by this extension<br><var>controlled_by_this_extension</var>: controlled by this extension"]
    pub type LevelOfControl;
}
