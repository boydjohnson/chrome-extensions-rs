#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "The <code>chrome.management</code> API provides ways to manage the list of extensions/apps that are installed and running. It is particularly useful for extensions that <a href='develop/ui/override-chrome-pages'>override</a> the built-in New Tab page."]
# [wasm_bindgen (js_namespace = chrome)]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "management.IconInfo" , typescript_type = "management.IconInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Information about an icon belonging to an extension, app, or theme."]
    pub type IconInfo;
    # [wasm_bindgen (method , getter , js_class = IconInfo)]
    #[doc = "A number representing the width and height of the icon. Likely values include (but are not limited to) 128, 48, 24, and 16."]
    pub fn size(this: &IconInfo) -> ::js_sys::Number;
    # [wasm_bindgen (method , getter , js_class = IconInfo)]
    #[doc = "The URL for this icon image. To display a grayscale version of the icon (to indicate that an extension is disabled, for example), append <code>?grayscale=true</code> to the URL."]
    pub fn url(this: &IconInfo) -> ::js_sys::JsString;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "management.LaunchType" , typescript_type = "management.LaunchType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "These are all possible app launch types."]
    pub type LaunchType;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "management.ExtensionDisabledReason" , typescript_type = "management.ExtensionDisabledReason")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "A reason the item is disabled."]
    pub type ExtensionDisabledReason;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "management.ExtensionType" , typescript_type = "management.ExtensionType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The type of this extension, app, or theme."]
    pub type ExtensionType;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "management.ExtensionInstallType" , typescript_type = "management.ExtensionInstallType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "How the extension was installed. One of<br><var>admin</var>: The extension was installed because of an administrative policy,<br><var>development</var>: The extension was loaded unpacked in developer mode,<br><var>normal</var>: The extension was installed normally via a .crx file,<br><var>sideload</var>: The extension was installed by other software on the machine,<br><var>other</var>: The extension was installed by other means."]
    pub type ExtensionInstallType;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "management.ExtensionInfo" , typescript_type = "management.ExtensionInfo")]
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
    pub fn disabledReason(this: &ExtensionInfo) -> Option<ExtensionDisabledReason>;
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
    pub fn installType(this: &ExtensionInfo) -> ExtensionInstallType;
    # [wasm_bindgen (method , getter , js_class = ExtensionInfo)]
    #[doc = "True if this is an app."]
    pub fn isApp(this: &ExtensionInfo) -> ::js_sys::Boolean;
    # [wasm_bindgen (method , getter , js_class = ExtensionInfo)]
    #[doc = "The app launch type (only present for apps)."]
    pub fn launchType(this: &ExtensionInfo) -> Option<LaunchType>;
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
    pub fn type_(this: &ExtensionInfo) -> ExtensionType;
    # [wasm_bindgen (method , getter , js_class = ExtensionInfo)]
    #[doc = "The update URL of this extension, app, or theme."]
    pub fn updateUrl(this: &ExtensionInfo) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = ExtensionInfo)]
    #[doc = "The <a href='reference/manifest/version'>version</a> of this extension, app, or theme."]
    pub fn version(this: &ExtensionInfo) -> ::js_sys::JsString;
    # [wasm_bindgen (method , getter , js_class = ExtensionInfo)]
    #[doc = "The <a href='reference/manifest/version#version_name'>version name</a> of this extension, app, or theme if the manifest specified one."]
    pub fn versionName(this: &ExtensionInfo) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "management.UninstallOptions" , typescript_type = "management.UninstallOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Options for how to handle the extension's uninstallation."]
    pub type UninstallOptions;
    # [wasm_bindgen (method , getter , js_class = UninstallOptions)]
    #[doc = "Whether or not a confirm-uninstall dialog should prompt the user. Defaults to false for self uninstalls. If an extension uninstalls another extension, this parameter is ignored and the dialog is always shown."]
    pub fn showConfirmDialog(this: &UninstallOptions) -> Option<::js_sys::Boolean>;
    #[doc = "Returns a list of information about installed extensions and apps."]
    #[wasm_bindgen(js_name = "management.getAll", catch)]
    pub async fn get_all() -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Returns information about the installed extension, app, or theme that has the given ID."]
    #[wasm_bindgen(js_name = "management.get", catch)]
    pub async fn get(
        id: ::js_sys::JsString,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Returns information about the calling extension, app, or theme. Note: This function can be used without requesting the 'management' permission in the manifest."]
    #[wasm_bindgen(js_name = "management.getSelf", catch)]
    pub async fn get_self() -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Returns a list of <a href='develop/concepts/permission-warnings'>permission warnings</a> for the given extension id."]
    #[wasm_bindgen(js_name = "management.getPermissionWarningsById", catch)]
    pub async fn get_permission_warnings_by_id(
        id: ::js_sys::JsString,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Returns a list of <a href='develop/concepts/permission-warnings'>permission warnings</a> for the given extension manifest string. Note: This function can be used without requesting the 'management' permission in the manifest."]
    #[wasm_bindgen(js_name = "management.getPermissionWarningsByManifest", catch)]
    pub async fn get_permission_warnings_by_manifest(
        manifestStr: ::js_sys::JsString,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Enables or disables an app or extension. In most cases this function must be called in the context of a user gesture (e.g. an onclick handler for a button), and may present the user with a native confirmation UI as a way of preventing abuse."]
    #[wasm_bindgen(js_name = "management.setEnabled", catch)]
    pub async fn set_enabled(
        id: ::js_sys::JsString,
        enabled: ::js_sys::Boolean,
    ) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Uninstalls a currently installed app or extension. Note: This function does not work in managed environments when the user is not allowed to uninstall the specified extension/app. If the uninstall fails (e.g. the user cancels the dialog) the promise will be rejected or the callback will be called with $(ref:runtime.lastError) set."]
    #[wasm_bindgen(js_name = "management.uninstall", catch)]
    pub async fn uninstall(
        id: ::js_sys::JsString,
        options: Option<UninstallOptions>,
    ) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Uninstalls the calling extension. Note: This function can be used without requesting the 'management' permission in the manifest. This function does not work in managed environments when the user is not allowed to uninstall the specified extension/app."]
    #[wasm_bindgen(js_name = "management.uninstallSelf", catch)]
    pub async fn uninstall_self(
        options: Option<UninstallOptions>,
    ) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Launches an application."]
    #[wasm_bindgen(js_name = "management.launchApp", catch)]
    pub async fn launch_app(id: ::js_sys::JsString) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Display options to create shortcuts for an app. On Mac, only packaged app shortcuts can be created."]
    #[wasm_bindgen(js_name = "management.createAppShortcut", catch)]
    pub async fn create_app_shortcut(id: ::js_sys::JsString)
        -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Set the launch type of an app."]
    #[wasm_bindgen(js_name = "management.setLaunchType", catch)]
    pub async fn set_launch_type(
        id: ::js_sys::JsString,
        launchType: LaunchType,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Generate an app for a URL. Returns the generated bookmark app."]
    #[wasm_bindgen(js_name = "management.generateAppForLink", catch)]
    pub async fn generate_app_for_link(
        url: ::js_sys::JsString,
        title: ::js_sys::JsString,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Launches the replacement_web_app specified in the manifest. Prompts the user to install if not already installed."]
    #[wasm_bindgen(js_name = "management.installReplacementWebApp", catch)]
    pub async fn install_replacement_web_app() -> Result<(), ::wasm_bindgen::JsValue>;
}
#[wasm_bindgen]
pub async fn management_get_all() -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    get_all().await
}
#[wasm_bindgen]
pub async fn management_get(
    id: ::js_sys::JsString,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    get(id).await
}
#[wasm_bindgen]
pub async fn management_get_self() -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    get_self().await
}
#[wasm_bindgen]
pub async fn management_get_permission_warnings_by_id(
    id: ::js_sys::JsString,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    get_permission_warnings_by_id(id).await
}
#[wasm_bindgen]
pub async fn management_get_permission_warnings_by_manifest(
    manifestStr: ::js_sys::JsString,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    get_permission_warnings_by_manifest(manifestStr).await
}
#[wasm_bindgen]
pub async fn management_set_enabled(
    id: ::js_sys::JsString,
    enabled: ::js_sys::Boolean,
) -> Result<(), ::wasm_bindgen::JsValue> {
    set_enabled(id, enabled).await
}
#[wasm_bindgen]
pub async fn management_uninstall(
    id: ::js_sys::JsString,
    options: Option<UninstallOptions>,
) -> Result<(), ::wasm_bindgen::JsValue> {
    uninstall(id, options).await
}
#[wasm_bindgen]
pub async fn management_uninstall_self(
    options: Option<UninstallOptions>,
) -> Result<(), ::wasm_bindgen::JsValue> {
    uninstall_self(options).await
}
#[wasm_bindgen]
pub async fn management_launch_app(id: ::js_sys::JsString) -> Result<(), ::wasm_bindgen::JsValue> {
    launch_app(id).await
}
#[wasm_bindgen]
pub async fn management_create_app_shortcut(
    id: ::js_sys::JsString,
) -> Result<(), ::wasm_bindgen::JsValue> {
    create_app_shortcut(id).await
}
#[wasm_bindgen]
pub async fn management_set_launch_type(
    id: ::js_sys::JsString,
    launchType: LaunchType,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    set_launch_type(id, launchType).await
}
#[wasm_bindgen]
pub async fn management_generate_app_for_link(
    url: ::js_sys::JsString,
    title: ::js_sys::JsString,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    generate_app_for_link(url, title).await
}
#[wasm_bindgen]
pub async fn management_install_replacement_web_app() -> Result<(), ::wasm_bindgen::JsValue> {
    install_replacement_web_app().await
}
