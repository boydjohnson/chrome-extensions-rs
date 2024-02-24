#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "The Desktop Capture API captures the content of the screen, individual windows, or individual tabs."]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "DesktopCaptureSourceType" , typescript_type = "DesktopCaptureSourceType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Enum used to define set of desktop media sources used in chooseDesktopMedia()."]
    pub type DesktopCaptureSourceType;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "SystemAudioPreferenceEnum" , typescript_type = "SystemAudioPreferenceEnum")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Mirrors <a href=\"https://w3c.github.io/mediacapture-screen-share/#dom-systemaudiopreferenceenum\">SystemAudioPreferenceEnum</a>."]
    pub type SystemAudioPreferenceEnum;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "SelfCapturePreferenceEnum" , typescript_type = "SelfCapturePreferenceEnum")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Mirrors <a href=\"https://w3c.github.io/mediacapture-screen-share/#dom-selfcapturepreferenceenum\">SelfCapturePreferenceEnum</a>."]
    pub type SelfCapturePreferenceEnum;
}
