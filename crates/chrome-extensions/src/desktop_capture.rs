#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "The Desktop Capture API captures the content of the screen, individual windows, or individual tabs."]
# [wasm_bindgen (js_namespace = chrome)]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "desktopCapture.DesktopCaptureSourceType" , typescript_type = "desktopCapture.DesktopCaptureSourceType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Enum used to define set of desktop media sources used in chooseDesktopMedia()."]
    pub type DesktopCaptureSourceType;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "desktopCapture.SystemAudioPreferenceEnum" , typescript_type = "desktopCapture.SystemAudioPreferenceEnum")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Mirrors <a href=\"https://w3c.github.io/mediacapture-screen-share/#dom-systemaudiopreferenceenum\">SystemAudioPreferenceEnum</a>."]
    pub type SystemAudioPreferenceEnum;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "desktopCapture.SelfCapturePreferenceEnum" , typescript_type = "desktopCapture.SelfCapturePreferenceEnum")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Mirrors <a href=\"https://w3c.github.io/mediacapture-screen-share/#dom-selfcapturepreferenceenum\">SelfCapturePreferenceEnum</a>."]
    pub type SelfCapturePreferenceEnum;
    #[doc = "Shows desktop media picker UI with the specified set of sources."]
    #[wasm_bindgen(js_name = "desktopCapture.chooseDesktopMedia")]
    pub fn chooseDesktopMedia(
        sources: ::js_sys::Array,
        targetTab: Option<crate::tabs::Tab>,
        options: Option<::js_sys::Object>,
    ) -> ::js_sys::Number;
}
#[wasm_bindgen]
pub fn desktop_capture_choose_desktop_media(
    sources: ::js_sys::Array,
    targetTab: Option<crate::tabs::Tab>,
    options: Option<::js_sys::Object>,
) -> ::js_sys::Number {
    chooseDesktopMedia(sources, targetTab, options)
}
