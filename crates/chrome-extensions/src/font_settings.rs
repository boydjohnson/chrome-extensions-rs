#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>chrome.fontSettings</code> API to manage Chrome's font settings."]
# [wasm_bindgen (js_namespace = chrome)]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "fontSettings.FontName" , typescript_type = "fontSettings.FontName")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Represents a font name."]
    pub type FontName;
    # [wasm_bindgen (method , getter , js_class = FontName)]
    #[doc = "The display name of the font."]
    pub fn displayName(this: &FontName) -> ::js_sys::JsString;
    # [wasm_bindgen (method , getter , js_class = FontName)]
    #[doc = "The font ID."]
    pub fn fontId(this: &FontName) -> ::js_sys::JsString;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "fontSettings.ScriptCode" , typescript_type = "fontSettings.ScriptCode")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "An ISO 15924 script code. The default, or global, script is represented by script code \"Zyyy\"."]
    pub type ScriptCode;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "fontSettings.GenericFamily" , typescript_type = "fontSettings.GenericFamily")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "A CSS generic font family."]
    pub type GenericFamily;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "fontSettings.LevelOfControl" , typescript_type = "fontSettings.LevelOfControl")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "One of<br><var>not_controllable</var>: cannot be controlled by any extension<br><var>controlled_by_other_extensions</var>: controlled by extensions with higher precedence<br><var>controllable_by_this_extension</var>: can be controlled by this extension<br><var>controlled_by_this_extension</var>: controlled by this extension"]
    pub type LevelOfControl;
    #[doc = "Clears the font set by this extension, if any."]
    #[wasm_bindgen(js_name = "fontSettings.clearFont", catch)]
    pub async fn clearFont(details: ::js_sys::Object) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Gets the font for a given script and generic font family."]
    #[wasm_bindgen(js_name = "fontSettings.getFont", catch)]
    pub async fn getFont(
        details: ::js_sys::Object,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Sets the font for a given script and generic font family."]
    #[wasm_bindgen(js_name = "fontSettings.setFont", catch)]
    pub async fn setFont(details: ::js_sys::Object) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Gets a list of fonts on the system."]
    #[wasm_bindgen(js_name = "fontSettings.getFontList", catch)]
    pub async fn getFontList() -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Clears the default font size set by this extension, if any."]
    #[wasm_bindgen(js_name = "fontSettings.clearDefaultFontSize", catch)]
    pub async fn clearDefaultFontSize(
        details: Option<::js_sys::Object>,
    ) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Gets the default font size."]
    #[wasm_bindgen(js_name = "fontSettings.getDefaultFontSize", catch)]
    pub async fn getDefaultFontSize(
        details: Option<::js_sys::Object>,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Sets the default font size."]
    #[wasm_bindgen(js_name = "fontSettings.setDefaultFontSize", catch)]
    pub async fn setDefaultFontSize(
        details: ::js_sys::Object,
    ) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Clears the default fixed font size set by this extension, if any."]
    #[wasm_bindgen(js_name = "fontSettings.clearDefaultFixedFontSize", catch)]
    pub async fn clearDefaultFixedFontSize(
        details: Option<::js_sys::Object>,
    ) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Gets the default size for fixed width fonts."]
    #[wasm_bindgen(js_name = "fontSettings.getDefaultFixedFontSize", catch)]
    pub async fn getDefaultFixedFontSize(
        details: Option<::js_sys::Object>,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Sets the default size for fixed width fonts."]
    #[wasm_bindgen(js_name = "fontSettings.setDefaultFixedFontSize", catch)]
    pub async fn setDefaultFixedFontSize(
        details: ::js_sys::Object,
    ) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Clears the minimum font size set by this extension, if any."]
    #[wasm_bindgen(js_name = "fontSettings.clearMinimumFontSize", catch)]
    pub async fn clearMinimumFontSize(
        details: Option<::js_sys::Object>,
    ) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Gets the minimum font size."]
    #[wasm_bindgen(js_name = "fontSettings.getMinimumFontSize", catch)]
    pub async fn getMinimumFontSize(
        details: Option<::js_sys::Object>,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Sets the minimum font size."]
    #[wasm_bindgen(js_name = "fontSettings.setMinimumFontSize", catch)]
    pub async fn setMinimumFontSize(
        details: ::js_sys::Object,
    ) -> Result<(), ::wasm_bindgen::JsValue>;
}
#[wasm_bindgen]
pub async fn font_settings_clear_font(
    details: ::js_sys::Object,
) -> Result<(), ::wasm_bindgen::JsValue> {
    clearFont(details).await
}
#[wasm_bindgen]
pub async fn font_settings_get_font(
    details: ::js_sys::Object,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    getFont(details).await
}
#[wasm_bindgen]
pub async fn font_settings_set_font(
    details: ::js_sys::Object,
) -> Result<(), ::wasm_bindgen::JsValue> {
    setFont(details).await
}
#[wasm_bindgen]
pub async fn font_settings_get_font_list(
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    getFontList().await
}
#[wasm_bindgen]
pub async fn font_settings_clear_default_font_size(
    details: Option<::js_sys::Object>,
) -> Result<(), ::wasm_bindgen::JsValue> {
    clearDefaultFontSize(details).await
}
#[wasm_bindgen]
pub async fn font_settings_get_default_font_size(
    details: Option<::js_sys::Object>,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    getDefaultFontSize(details).await
}
#[wasm_bindgen]
pub async fn font_settings_set_default_font_size(
    details: ::js_sys::Object,
) -> Result<(), ::wasm_bindgen::JsValue> {
    setDefaultFontSize(details).await
}
#[wasm_bindgen]
pub async fn font_settings_clear_default_fixed_font_size(
    details: Option<::js_sys::Object>,
) -> Result<(), ::wasm_bindgen::JsValue> {
    clearDefaultFixedFontSize(details).await
}
#[wasm_bindgen]
pub async fn font_settings_get_default_fixed_font_size(
    details: Option<::js_sys::Object>,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    getDefaultFixedFontSize(details).await
}
#[wasm_bindgen]
pub async fn font_settings_set_default_fixed_font_size(
    details: ::js_sys::Object,
) -> Result<(), ::wasm_bindgen::JsValue> {
    setDefaultFixedFontSize(details).await
}
#[wasm_bindgen]
pub async fn font_settings_clear_minimum_font_size(
    details: Option<::js_sys::Object>,
) -> Result<(), ::wasm_bindgen::JsValue> {
    clearMinimumFontSize(details).await
}
#[wasm_bindgen]
pub async fn font_settings_get_minimum_font_size(
    details: Option<::js_sys::Object>,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    getMinimumFontSize(details).await
}
#[wasm_bindgen]
pub async fn font_settings_set_minimum_font_size(
    details: ::js_sys::Object,
) -> Result<(), ::wasm_bindgen::JsValue> {
    setMinimumFontSize(details).await
}
