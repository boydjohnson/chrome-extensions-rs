#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>chrome.i18n</code> infrastructure to implement internationalization across your whole app or extension."]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "LanguageCode" , typescript_type = "LanguageCode")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "An ISO language code such as <code>en</code> or <code>fr</code>. For a complete list of languages supported by this method, see <a href='http://src.chromium.org/viewvc/chrome/trunk/src/third_party/cld/languages/internal/languages.cc'>kLanguageInfoTable</a>. For an unknown language, <code>und</code> will be returned, which means that [percentage] of the text is unknown to CLD"]
    pub type LanguageCode;
}
