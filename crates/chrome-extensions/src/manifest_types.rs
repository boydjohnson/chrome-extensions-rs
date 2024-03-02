#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Schemas for structured manifest entries"]
# [wasm_bindgen (js_namespace = chrome)]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "manifestTypes.ChromeSettingsOverrides" , typescript_type = "manifestTypes.ChromeSettingsOverrides")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Chrome settings which can be overriden by an extension."]
    pub type ChromeSettingsOverrides;
    # [wasm_bindgen (method , getter , js_class = ChromeSettingsOverrides)]
    #[doc = "New value for the homepage."]
    pub fn homepage(this: &ChromeSettingsOverrides) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = ChromeSettingsOverrides)]
    #[doc = "A search engine"]
    pub fn search_provider(this: &ChromeSettingsOverrides) -> Option<::js_sys::Object>;
    # [wasm_bindgen (method , getter , js_class = ChromeSettingsOverrides)]
    #[doc = "An array of length one containing a URL to be used as the startup page."]
    pub fn startup_pages(this: &ChromeSettingsOverrides) -> Option<::js_sys::Array>;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "manifestTypes.FileSystemProviderSource" , typescript_type = "manifestTypes.FileSystemProviderSource")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "For <code>\"file\"</code> the source is a file passed via <code>onLaunched</code> event. For <code>\"device\"</code> contents are fetched from an external device (eg. plugged via USB), without using <code>file_handlers</code>. Finally, for <code>\"network\"</code> source, contents should be fetched via network."]
    pub type FileSystemProviderSource;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "manifestTypes.FileSystemProviderCapabilities" , typescript_type = "manifestTypes.FileSystemProviderCapabilities")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Represents capabilities of a providing extension."]
    pub type FileSystemProviderCapabilities;
    # [wasm_bindgen (method , getter , js_class = FileSystemProviderCapabilities)]
    #[doc = "Whether configuring via <code>onConfigureRequested</code> is supported. By default: <code>false</code>."]
    pub fn configurable(this: &FileSystemProviderCapabilities) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = FileSystemProviderCapabilities)]
    #[doc = "Whether multiple (more than one) mounted file systems are supported. By default: <code>false</code>."]
    pub fn multiple_mounts(this: &FileSystemProviderCapabilities) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = FileSystemProviderCapabilities)]
    #[doc = "Source of data for mounted file systems."]
    pub fn source(this: &FileSystemProviderCapabilities) -> FileSystemProviderSource;
    # [wasm_bindgen (method , getter , js_class = FileSystemProviderCapabilities)]
    #[doc = "Whether setting watchers and notifying about changes is supported. By default: <code>false</code>."]
    pub fn watchable(this: &FileSystemProviderCapabilities) -> Option<::js_sys::Boolean>;
}
