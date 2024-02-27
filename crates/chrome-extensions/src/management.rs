#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "The <code>chrome.management</code> API provides ways to manage the list of extensions/apps that are installed and running. It is particularly useful for extensions that <a href='develop/ui/override-chrome-pages'>override</a> the built-in New Tab page."]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "IconInfo" , typescript_type = "IconInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Information about an icon belonging to an extension, app, or theme."]
    pub type IconInfo;
    # [wasm_bindgen (method , getter , js_class = IconInfo)]
    #[doc = "A number representing the width and height of the icon. Likely values include (but are not limited to) 128, 48, 24, and 16."]
    pub fn size(this: &IconInfo) -> ::js_sys::Number;
    # [wasm_bindgen (method , getter , js_class = IconInfo)]
    #[doc = "The URL for this icon image. To display a grayscale version of the icon (to indicate that an extension is disabled, for example), append <code>?grayscale=true</code> to the URL."]
    pub fn url(this: &IconInfo) -> ::js_sys::JsString;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "LaunchType" , typescript_type = "LaunchType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "These are all possible app launch types."]
    pub type LaunchType;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "ExtensionDisabledReason" , typescript_type = "ExtensionDisabledReason")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "A reason the item is disabled."]
    pub type ExtensionDisabledReason;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "ExtensionType" , typescript_type = "ExtensionType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The type of this extension, app, or theme."]
    pub type ExtensionType;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "ExtensionInstallType" , typescript_type = "ExtensionInstallType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "How the extension was installed. One of<br><var>admin</var>: The extension was installed because of an administrative policy,<br><var>development</var>: The extension was loaded unpacked in developer mode,<br><var>normal</var>: The extension was installed normally via a .crx file,<br><var>sideload</var>: The extension was installed by other software on the machine,<br><var>other</var>: The extension was installed by other means."]
    pub type ExtensionInstallType;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "ExtensionInfo" , typescript_type = "ExtensionInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Information about an installed extension, app, or theme."]
    pub type ExtensionInfo;
    # [wasm_bindgen (method , getter , js_class = ExtensionInfo)]
    #[doc = "The launch url (only present for apps)."]
    pub fn appLaunchUrl(this: &ExtensionInfo) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = ExtensionInfo)]
    #[doc = "The currently available launch types (only present for apps)."]
    pub fn availableLaunchTypes(this: &ExtensionInfo) -> Option<::js_sys::Array>;
    # [wasm_bindgen (method , getter , js_class = ExtensionInfo)]
    #[doc = "The description of this extension, app, or theme."]
    pub fn description(this: &ExtensionInfo) -> ::js_sys::JsString;
    # [wasm_bindgen (method , getter , js_class = ExtensionInfo)]
    #[doc = "A reason the item is disabled."]
    pub fn disabledReason(this: &ExtensionInfo) -> Option<i32>;
    # [wasm_bindgen (method , getter , js_class = ExtensionInfo)]
    #[doc = "Whether it is currently enabled or disabled."]
    pub fn enabled(this: &ExtensionInfo) -> ::js_sys::Boolean;
    # [wasm_bindgen (method , getter , js_class = ExtensionInfo)]
    #[doc = "The URL of the homepage of this extension, app, or theme."]
    pub fn homepageUrl(this: &ExtensionInfo) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = ExtensionInfo)]
    #[doc = "Returns a list of host based permissions."]
    pub fn hostPermissions(this: &ExtensionInfo) -> ::js_sys::Array;
    # [wasm_bindgen (method , getter , js_class = ExtensionInfo)]
    #[doc = "A list of icon information. Note that this just reflects what was declared in the manifest, and the actual image at that url may be larger or smaller than what was declared, so you might consider using explicit width and height attributes on img tags referencing these images. See the <a href='reference/manifest/icons'>manifest documentation on icons</a> for more details."]
    pub fn icons(this: &ExtensionInfo) -> Option<::js_sys::Array>;
    # [wasm_bindgen (method , getter , js_class = ExtensionInfo)]
    #[doc = "The extension's unique identifier."]
    pub fn id(this: &ExtensionInfo) -> ::js_sys::JsString;
    # [wasm_bindgen (method , getter , js_class = ExtensionInfo)]
    #[doc = "How the extension was installed."]
    pub fn installType(this: &ExtensionInfo) -> i32;
    # [wasm_bindgen (method , getter , js_class = ExtensionInfo)]
    #[doc = "True if this is an app."]
    pub fn isApp(this: &ExtensionInfo) -> ::js_sys::Boolean;
    # [wasm_bindgen (method , getter , js_class = ExtensionInfo)]
    #[doc = "The app launch type (only present for apps)."]
    pub fn launchType(this: &ExtensionInfo) -> Option<i32>;
    # [wasm_bindgen (method , getter , js_class = ExtensionInfo)]
    #[doc = "Whether this extension can be disabled or uninstalled by the user."]
    pub fn mayDisable(this: &ExtensionInfo) -> ::js_sys::Boolean;
    # [wasm_bindgen (method , getter , js_class = ExtensionInfo)]
    #[doc = "Whether this extension can be enabled by the user. This is only returned for extensions which are not enabled."]
    pub fn mayEnable(this: &ExtensionInfo) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = ExtensionInfo)]
    #[doc = "The name of this extension, app, or theme."]
    pub fn name(this: &ExtensionInfo) -> ::js_sys::JsString;
    # [wasm_bindgen (method , getter , js_class = ExtensionInfo)]
    #[doc = "Whether the extension, app, or theme declares that it supports offline."]
    pub fn offlineEnabled(this: &ExtensionInfo) -> ::js_sys::Boolean;
    # [wasm_bindgen (method , getter , js_class = ExtensionInfo)]
    #[doc = "The url for the item's options page, if it has one."]
    pub fn optionsUrl(this: &ExtensionInfo) -> ::js_sys::JsString;
    # [wasm_bindgen (method , getter , js_class = ExtensionInfo)]
    #[doc = "Returns a list of API based permissions."]
    pub fn permissions(this: &ExtensionInfo) -> ::js_sys::Array;
    # [wasm_bindgen (method , getter , js_class = ExtensionInfo)]
    #[doc = "A short version of the name of this extension, app, or theme."]
    pub fn shortName(this: &ExtensionInfo) -> ::js_sys::JsString;
    # [wasm_bindgen (method , getter , js_class = ExtensionInfo)]
    #[doc = "The type of this extension, app, or theme."]
    pub fn type_(this: &ExtensionInfo) -> i32;
    # [wasm_bindgen (method , getter , js_class = ExtensionInfo)]
    #[doc = "The update URL of this extension, app, or theme."]
    pub fn updateUrl(this: &ExtensionInfo) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = ExtensionInfo)]
    #[doc = "The <a href='reference/manifest/version'>version</a> of this extension, app, or theme."]
    pub fn version(this: &ExtensionInfo) -> ::js_sys::JsString;
    # [wasm_bindgen (method , getter , js_class = ExtensionInfo)]
    #[doc = "The <a href='reference/manifest/version#version_name'>version name</a> of this extension, app, or theme if the manifest specified one."]
    pub fn versionName(this: &ExtensionInfo) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "UninstallOptions" , typescript_type = "UninstallOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Options for how to handle the extension's uninstallation."]
    pub type UninstallOptions;
    # [wasm_bindgen (method , getter , js_class = UninstallOptions)]
    #[doc = "Whether or not a confirm-uninstall dialog should prompt the user. Defaults to false for self uninstalls. If an extension uninstalls another extension, this parameter is ignored and the dialog is always shown."]
    pub fn showConfirmDialog(this: &UninstallOptions) -> Option<::js_sys::Boolean>;
}
