#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>chrome.action</code> API to control the extension's icon in the Google Chrome toolbar."]
# [wasm_bindgen (js_namespace = chrome)]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "action.TabDetails" , typescript_type = "action.TabDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type TabDetails;
    # [wasm_bindgen (method , getter , js_class = TabDetails)]
    #[doc = "The ID of the tab to query state for. If no tab is specified, the non-tab-specific state is returned."]
    pub fn tabId(this: &TabDetails) -> Option<::js_sys::Number>;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "action.UserSettings" , typescript_type = "action.UserSettings")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The collection of user-specified settings relating to an extension's action."]
    pub type UserSettings;
    # [wasm_bindgen (method , getter , js_class = UserSettings)]
    #[doc = "Whether the extension's action icon is visible on browser windows' top-level toolbar (i.e., whether the extension has been 'pinned' by the user)."]
    pub fn isOnToolbar(this: &UserSettings) -> ::js_sys::Boolean;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "action.OpenPopupOptions" , typescript_type = "action.OpenPopupOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type OpenPopupOptions;
    # [wasm_bindgen (method , getter , js_class = OpenPopupOptions)]
    #[doc = "The ID of the window to open the action popup in. Defaults to the currently-active window if unspecified."]
    pub fn windowId(this: &OpenPopupOptions) -> Option<::js_sys::Number>;
    #[doc = "Sets the title of the action. This shows up in the tooltip."]
    #[wasm_bindgen(js_name = "action.setTitle", catch)]
    pub async fn set_title(details: ::js_sys::Object) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Gets the title of the action."]
    #[wasm_bindgen(js_name = "action.getTitle", catch)]
    pub async fn get_title(
        details: TabDetails,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Sets the icon for the action. The icon can be specified either as the path to an image file or as the pixel data from a canvas element, or as dictionary of either one of those. Either the <b>path</b> or the <b>imageData</b> property must be specified."]
    #[wasm_bindgen(js_name = "action.setIcon", catch)]
    pub async fn set_icon(details: ::js_sys::Object) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Sets the HTML document to be opened as a popup when the user clicks on the action's icon."]
    #[wasm_bindgen(js_name = "action.setPopup", catch)]
    pub async fn set_popup(details: ::js_sys::Object) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Gets the html document set as the popup for this action."]
    #[wasm_bindgen(js_name = "action.getPopup", catch)]
    pub async fn get_popup(
        details: TabDetails,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Sets the badge text for the action. The badge is displayed on top of the icon."]
    #[wasm_bindgen(js_name = "action.setBadgeText", catch)]
    pub async fn set_badge_text(details: ::js_sys::Object) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Gets the badge text of the action. If no tab is specified, the non-tab-specific badge text is returned. If <a href='declarativeNetRequest#setExtensionActionOptions'>displayActionCountAsBadgeText</a> is enabled, a placeholder text will be returned unless the <a href='/docs/extensions/develop/concepts/declare-permissions#declarativeNetRequestFeedback'>declarativeNetRequestFeedback</a> permission is present or tab-specific badge text was provided."]
    #[wasm_bindgen(js_name = "action.getBadgeText", catch)]
    pub async fn get_badge_text(
        details: TabDetails,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Sets the background color for the badge."]
    #[wasm_bindgen(js_name = "action.setBadgeBackgroundColor", catch)]
    pub async fn set_badge_background_color(
        details: ::js_sys::Object,
    ) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Gets the background color of the action."]
    #[wasm_bindgen(js_name = "action.getBadgeBackgroundColor", catch)]
    pub async fn get_badge_background_color(
        details: TabDetails,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Sets the text color for the badge."]
    #[wasm_bindgen(js_name = "action.setBadgeTextColor", catch)]
    pub async fn set_badge_text_color(
        details: ::js_sys::Object,
    ) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Gets the text color of the action."]
    #[wasm_bindgen(js_name = "action.getBadgeTextColor", catch)]
    pub async fn get_badge_text_color(
        details: TabDetails,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Enables the action for a tab. By default, actions are enabled."]
    #[wasm_bindgen(js_name = "action.enable", catch)]
    pub async fn enable(tabId: Option<::js_sys::Number>) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Disables the action for a tab."]
    #[wasm_bindgen(js_name = "action.disable", catch)]
    pub async fn disable(tabId: Option<::js_sys::Number>) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Indicates whether the extension action is enabled for a tab (or globally if no <code>tabId</code> is provided). Actions enabled using only $(ref:declarativeContent) always return false."]
    #[wasm_bindgen(js_name = "action.isEnabled", catch)]
    pub async fn is_enabled(
        tabId: Option<::js_sys::Number>,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Returns the user-specified settings relating to an extension's action."]
    #[wasm_bindgen(js_name = "action.getUserSettings", catch)]
    pub async fn get_user_settings() -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Opens the extension's popup."]
    #[wasm_bindgen(js_name = "action.openPopup", catch)]
    pub async fn open_popup(
        options: Option<OpenPopupOptions>,
    ) -> Result<(), ::wasm_bindgen::JsValue>;
}
#[wasm_bindgen]
pub async fn action_set_title(details: ::js_sys::Object) -> Result<(), ::wasm_bindgen::JsValue> {
    set_title(details).await
}
#[wasm_bindgen]
pub async fn action_get_title(
    details: TabDetails,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    get_title(details).await
}
#[wasm_bindgen]
pub async fn action_set_icon(details: ::js_sys::Object) -> Result<(), ::wasm_bindgen::JsValue> {
    set_icon(details).await
}
#[wasm_bindgen]
pub async fn action_set_popup(details: ::js_sys::Object) -> Result<(), ::wasm_bindgen::JsValue> {
    set_popup(details).await
}
#[wasm_bindgen]
pub async fn action_get_popup(
    details: TabDetails,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    get_popup(details).await
}
#[wasm_bindgen]
pub async fn action_set_badge_text(
    details: ::js_sys::Object,
) -> Result<(), ::wasm_bindgen::JsValue> {
    set_badge_text(details).await
}
#[wasm_bindgen]
pub async fn action_get_badge_text(
    details: TabDetails,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    get_badge_text(details).await
}
#[wasm_bindgen]
pub async fn action_set_badge_background_color(
    details: ::js_sys::Object,
) -> Result<(), ::wasm_bindgen::JsValue> {
    set_badge_background_color(details).await
}
#[wasm_bindgen]
pub async fn action_get_badge_background_color(
    details: TabDetails,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    get_badge_background_color(details).await
}
#[wasm_bindgen]
pub async fn action_set_badge_text_color(
    details: ::js_sys::Object,
) -> Result<(), ::wasm_bindgen::JsValue> {
    set_badge_text_color(details).await
}
#[wasm_bindgen]
pub async fn action_get_badge_text_color(
    details: TabDetails,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    get_badge_text_color(details).await
}
#[wasm_bindgen]
pub async fn action_enable(tabId: Option<::js_sys::Number>) -> Result<(), ::wasm_bindgen::JsValue> {
    enable(tabId).await
}
#[wasm_bindgen]
pub async fn action_disable(
    tabId: Option<::js_sys::Number>,
) -> Result<(), ::wasm_bindgen::JsValue> {
    disable(tabId).await
}
#[wasm_bindgen]
pub async fn action_is_enabled(
    tabId: Option<::js_sys::Number>,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    is_enabled(tabId).await
}
#[wasm_bindgen]
pub async fn action_get_user_settings() -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>
{
    get_user_settings().await
}
#[wasm_bindgen]
pub async fn action_open_popup(
    options: Option<OpenPopupOptions>,
) -> Result<(), ::wasm_bindgen::JsValue> {
    open_popup(options).await
}
