#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>chrome.contentSettings</code> API to change settings that control whether websites can use features such as cookies, JavaScript, and plugins. More generally speaking, content settings allow you to customize Chrome's behavior on a per-site basis instead of globally."]
# [wasm_bindgen (js_namespace = chrome)]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "contentSettings.ResourceIdentifier" , typescript_type = "contentSettings.ResourceIdentifier")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The only content type using resource identifiers is $(ref:contentSettings.plugins). For more information, see <a href=\"contentSettings#resource-identifiers\">Resource Identifiers</a>."]
    pub type ResourceIdentifier;
    # [wasm_bindgen (method , getter , js_class = ResourceIdentifier)]
    #[doc = "A human readable description of the resource."]
    pub fn description(this: &ResourceIdentifier) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = ResourceIdentifier)]
    #[doc = "The resource identifier for the given content type."]
    pub fn id(this: &ResourceIdentifier) -> ::js_sys::JsString;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "contentSettings.Scope" , typescript_type = "contentSettings.Scope")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The scope of the ContentSetting. One of<br><var>regular</var>: setting for regular profile (which is inherited by the incognito profile if not overridden elsewhere),<br><var>incognito_session_only</var>: setting for incognito profile that can only be set during an incognito session and is deleted when the incognito session ends (overrides regular settings)."]
    pub type Scope;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "contentSettings.ContentSetting" , typescript_type = "contentSettings.ContentSetting")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type ContentSetting;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "contentSettings.AutoVerifyContentSetting" , typescript_type = "contentSettings.AutoVerifyContentSetting")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type AutoVerifyContentSetting;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "contentSettings.ClipboardContentSetting" , typescript_type = "contentSettings.ClipboardContentSetting")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type ClipboardContentSetting;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "contentSettings.CookiesContentSetting" , typescript_type = "contentSettings.CookiesContentSetting")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type CookiesContentSetting;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "contentSettings.ImagesContentSetting" , typescript_type = "contentSettings.ImagesContentSetting")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type ImagesContentSetting;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "contentSettings.JavascriptContentSetting" , typescript_type = "contentSettings.JavascriptContentSetting")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type JavascriptContentSetting;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "contentSettings.LocationContentSetting" , typescript_type = "contentSettings.LocationContentSetting")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type LocationContentSetting;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "contentSettings.PluginsContentSetting" , typescript_type = "contentSettings.PluginsContentSetting")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type PluginsContentSetting;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "contentSettings.PopupsContentSetting" , typescript_type = "contentSettings.PopupsContentSetting")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type PopupsContentSetting;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "contentSettings.NotificationsContentSetting" , typescript_type = "contentSettings.NotificationsContentSetting")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type NotificationsContentSetting;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "contentSettings.FullscreenContentSetting" , typescript_type = "contentSettings.FullscreenContentSetting")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type FullscreenContentSetting;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "contentSettings.MouselockContentSetting" , typescript_type = "contentSettings.MouselockContentSetting")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type MouselockContentSetting;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "contentSettings.MicrophoneContentSetting" , typescript_type = "contentSettings.MicrophoneContentSetting")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type MicrophoneContentSetting;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "contentSettings.CameraContentSetting" , typescript_type = "contentSettings.CameraContentSetting")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type CameraContentSetting;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "contentSettings.PpapiBrokerContentSetting" , typescript_type = "contentSettings.PpapiBrokerContentSetting")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type PpapiBrokerContentSetting;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "contentSettings.MultipleAutomaticDownloadsContentSetting" , typescript_type = "contentSettings.MultipleAutomaticDownloadsContentSetting")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type MultipleAutomaticDownloadsContentSetting;
}
