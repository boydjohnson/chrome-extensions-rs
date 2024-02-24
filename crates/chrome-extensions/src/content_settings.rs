#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>chrome.contentSettings</code> API to change settings that control whether websites can use features such as cookies, JavaScript, and plugins. More generally speaking, content settings allow you to customize Chrome's behavior on a per-site basis instead of globally."]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "ResourceIdentifier" , typescript_type = "ResourceIdentifier")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The only content type using resource identifiers is $(ref:contentSettings.plugins). For more information, see <a href=\"contentSettings#resource-identifiers\">Resource Identifiers</a>."]
    pub type ResourceIdentifier;
    # [wasm_bindgen (method , getter , js_class = ResourceIdentifier)]
    #[doc = "A human readable description of the resource."]
    pub fn description(this: &ResourceIdentifier) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = ResourceIdentifier)]
    #[doc = "The resource identifier for the given content type."]
    pub fn id(this: &ResourceIdentifier) -> ::js_sys::JsString;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "Scope" , typescript_type = "Scope")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The scope of the ContentSetting. One of<br><var>regular</var>: setting for regular profile (which is inherited by the incognito profile if not overridden elsewhere),<br><var>incognito_session_only</var>: setting for incognito profile that can only be set during an incognito session and is deleted when the incognito session ends (overrides regular settings)."]
    pub type Scope;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "ContentSetting" , typescript_type = "ContentSetting")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type ContentSetting;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "AutoVerifyContentSetting" , typescript_type = "AutoVerifyContentSetting")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type AutoVerifyContentSetting;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "ClipboardContentSetting" , typescript_type = "ClipboardContentSetting")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type ClipboardContentSetting;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "CookiesContentSetting" , typescript_type = "CookiesContentSetting")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type CookiesContentSetting;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "ImagesContentSetting" , typescript_type = "ImagesContentSetting")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type ImagesContentSetting;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "JavascriptContentSetting" , typescript_type = "JavascriptContentSetting")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type JavascriptContentSetting;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "LocationContentSetting" , typescript_type = "LocationContentSetting")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type LocationContentSetting;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "PluginsContentSetting" , typescript_type = "PluginsContentSetting")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type PluginsContentSetting;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "PopupsContentSetting" , typescript_type = "PopupsContentSetting")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type PopupsContentSetting;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "NotificationsContentSetting" , typescript_type = "NotificationsContentSetting")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type NotificationsContentSetting;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "FullscreenContentSetting" , typescript_type = "FullscreenContentSetting")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type FullscreenContentSetting;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "MouselockContentSetting" , typescript_type = "MouselockContentSetting")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type MouselockContentSetting;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "MicrophoneContentSetting" , typescript_type = "MicrophoneContentSetting")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type MicrophoneContentSetting;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "CameraContentSetting" , typescript_type = "CameraContentSetting")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type CameraContentSetting;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "PpapiBrokerContentSetting" , typescript_type = "PpapiBrokerContentSetting")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type PpapiBrokerContentSetting;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "MultipleAutomaticDownloadsContentSetting" , typescript_type = "MultipleAutomaticDownloadsContentSetting")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type MultipleAutomaticDownloadsContentSetting;
}
