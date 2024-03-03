#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>chrome.i18n</code> infrastructure to implement internationalization across your whole app or extension."]
# [wasm_bindgen (js_namespace = chrome)]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "i18n.LanguageCode" , typescript_type = "i18n.LanguageCode")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "An ISO language code such as <code>en</code> or <code>fr</code>. For a complete list of languages supported by this method, see <a href='http://src.chromium.org/viewvc/chrome/trunk/src/third_party/cld/languages/internal/languages.cc'>kLanguageInfoTable</a>. For an unknown language, <code>und</code> will be returned, which means that [percentage] of the text is unknown to CLD"]
    pub type LanguageCode;
    #[doc = "Gets the accept-languages of the browser. This is different from the locale used by the browser; to get the locale, use $(ref:i18n.getUILanguage)."]
    #[wasm_bindgen(js_name = "i18n.getAcceptLanguages", catch)]
    pub async fn get_accept_languages() -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Gets the accept-languages of the browser. This is different from the locale used by the browser; to get the locale, use $(ref:i18n.getUILanguage)."]
    #[wasm_bindgen(js_name = "i18n.getAcceptLanguages")]
    pub fn get_accept_languages_callback(callback: &::js_sys::Function);
    #[doc = "Gets the localized string for the specified message. If the message is missing, this method returns an empty string (''). If the format of the <code>getMessage()</code> call is wrong &mdash; for example, <em>messageName</em> is not a string or the <em>substitutions</em> array has more than 9 elements &mdash; this method returns <code>undefined</code>."]
    #[wasm_bindgen(js_name = "i18n.getMessage")]
    pub fn get_message(
        messageName: ::js_sys::JsString,
        substitutions: ::wasm_bindgen::JsValue,
        options: Option<::js_sys::Object>,
    ) -> ::js_sys::JsString;
    #[doc = "Gets the browser UI language of the browser. This is different from $(ref:i18n.getAcceptLanguages) which returns the preferred user languages."]
    #[wasm_bindgen(js_name = "i18n.getUILanguage")]
    pub fn get_ui_language() -> ::js_sys::JsString;
    #[doc = "Detects the language of the provided text using CLD."]
    #[wasm_bindgen(js_name = "i18n.detectLanguage", catch)]
    pub async fn detect_language(
        text: ::js_sys::JsString,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Detects the language of the provided text using CLD."]
    #[wasm_bindgen(js_name = "i18n.detectLanguage")]
    pub fn detect_language_callback(text: ::js_sys::JsString, callback: &::js_sys::Function);
}
#[wasm_bindgen]
pub async fn i18n_get_accept_languages() -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>
{
    get_accept_languages().await
}
#[wasm_bindgen]
pub fn i18n_get_accept_languages_callback(callback: &::js_sys::Function) {
    get_accept_languages_callback(callback);
}
#[wasm_bindgen]
pub fn i18n_get_message(
    messageName: ::js_sys::JsString,
    substitutions: ::wasm_bindgen::JsValue,
    options: Option<::js_sys::Object>,
) -> ::js_sys::JsString {
    get_message(messageName, substitutions, options)
}
#[wasm_bindgen]
pub fn i18n_get_ui_language() -> ::js_sys::JsString {
    get_ui_language()
}
#[wasm_bindgen]
pub async fn i18n_detect_language(
    text: ::js_sys::JsString,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    detect_language(text).await
}
#[wasm_bindgen]
pub fn i18n_detect_language_callback(text: ::js_sys::JsString, callback: &::js_sys::Function) {
    detect_language_callback(text, callback);
}
