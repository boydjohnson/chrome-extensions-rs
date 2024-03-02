#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use browser actions to put icons in the main Google Chrome toolbar, to the right of the address bar. In addition to its <a href='browserAction#icon'>icon</a>, a browser action can have a <a href='browserAction#tooltip'>tooltip</a>, a <a href='browserAction#badge'>badge</a>, and a <a href='browserAction#popup'>popup</a>."]
# [wasm_bindgen (js_namespace = chrome)]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Array , js_name = "browserAction.ColorArray" , typescript_type = "browserAction.ColorArray")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type ColorArray;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "browserAction.ImageDataType" , typescript_type = "browserAction.ImageDataType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Pixel data for an image. Must be an ImageData object; for example, from a <code>canvas</code> element."]
    pub type ImageDataType;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "browserAction.TabDetails" , typescript_type = "browserAction.TabDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type TabDetails;
    # [wasm_bindgen (method , getter , js_class = TabDetails)]
    #[doc = "The ID of the tab to query state for. If no tab is specified, the non-tab-specific state is returned."]
    pub fn tabId(this: &TabDetails) -> Option<::js_sys::Number>;
    #[doc = "Sets the title of the browser action. This title appears in the tooltip."]
    #[wasm_bindgen(js_name = "browserAction.setTitle", catch)]
    pub async fn setTitle(details: ::js_sys::Object) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Gets the title of the browser action."]
    #[wasm_bindgen(js_name = "browserAction.getTitle", catch)]
    pub async fn getTitle(
        details: TabDetails,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Sets the icon for the browser action. The icon can be specified as the path to an image file, as the pixel data from a canvas element, or as a dictionary of one of those. Either the <code>path</code> or the <code>imageData</code> property must be specified."]
    #[wasm_bindgen(js_name = "browserAction.setIcon", catch)]
    pub async fn setIcon(details: ::js_sys::Object) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Sets the HTML document to be opened as a popup when the user clicks the browser action icon."]
    #[wasm_bindgen(js_name = "browserAction.setPopup", catch)]
    pub async fn setPopup(details: ::js_sys::Object) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Gets the HTML document that is set as the popup for this browser action."]
    #[wasm_bindgen(js_name = "browserAction.getPopup", catch)]
    pub async fn getPopup(
        details: TabDetails,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Sets the badge text for the browser action. The badge is displayed on top of the icon."]
    #[wasm_bindgen(js_name = "browserAction.setBadgeText", catch)]
    pub async fn setBadgeText(details: ::js_sys::Object) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Gets the badge text of the browser action. If no tab is specified, the non-tab-specific badge text is returned."]
    #[wasm_bindgen(js_name = "browserAction.getBadgeText", catch)]
    pub async fn getBadgeText(
        details: TabDetails,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Sets the background color for the badge."]
    #[wasm_bindgen(js_name = "browserAction.setBadgeBackgroundColor", catch)]
    pub async fn setBadgeBackgroundColor(
        details: ::js_sys::Object,
    ) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Gets the background color of the browser action."]
    #[wasm_bindgen(js_name = "browserAction.getBadgeBackgroundColor", catch)]
    pub async fn getBadgeBackgroundColor(
        details: TabDetails,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Enables the browser action for a tab. Defaults to enabled."]
    #[wasm_bindgen(js_name = "browserAction.enable", catch)]
    pub async fn enable(tabId: Option<::js_sys::Number>) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Disables the browser action for a tab."]
    #[wasm_bindgen(js_name = "browserAction.disable", catch)]
    pub async fn disable(tabId: Option<::js_sys::Number>) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Opens the extension popup window in the active window but does not grant tab permissions."]
    #[wasm_bindgen(js_name = "browserAction.openPopup", catch)]
    pub async fn openPopup() -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
}
#[wasm_bindgen]
pub async fn browser_action_set_title(
    details: ::js_sys::Object,
) -> Result<(), ::wasm_bindgen::JsValue> {
    setTitle(details).await
}
#[wasm_bindgen]
pub async fn browser_action_get_title(
    details: TabDetails,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    getTitle(details).await
}
#[wasm_bindgen]
pub async fn browser_action_set_icon(
    details: ::js_sys::Object,
) -> Result<(), ::wasm_bindgen::JsValue> {
    setIcon(details).await
}
#[wasm_bindgen]
pub async fn browser_action_set_popup(
    details: ::js_sys::Object,
) -> Result<(), ::wasm_bindgen::JsValue> {
    setPopup(details).await
}
#[wasm_bindgen]
pub async fn browser_action_get_popup(
    details: TabDetails,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    getPopup(details).await
}
#[wasm_bindgen]
pub async fn browser_action_set_badge_text(
    details: ::js_sys::Object,
) -> Result<(), ::wasm_bindgen::JsValue> {
    setBadgeText(details).await
}
#[wasm_bindgen]
pub async fn browser_action_get_badge_text(
    details: TabDetails,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    getBadgeText(details).await
}
#[wasm_bindgen]
pub async fn browser_action_set_badge_background_color(
    details: ::js_sys::Object,
) -> Result<(), ::wasm_bindgen::JsValue> {
    setBadgeBackgroundColor(details).await
}
#[wasm_bindgen]
pub async fn browser_action_get_badge_background_color(
    details: TabDetails,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    getBadgeBackgroundColor(details).await
}
#[wasm_bindgen]
pub async fn browser_action_enable(
    tabId: Option<::js_sys::Number>,
) -> Result<(), ::wasm_bindgen::JsValue> {
    enable(tabId).await
}
#[wasm_bindgen]
pub async fn browser_action_disable(
    tabId: Option<::js_sys::Number>,
) -> Result<(), ::wasm_bindgen::JsValue> {
    disable(tabId).await
}
#[wasm_bindgen]
pub async fn browser_action_open_popup() -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>
{
    openPopup().await
}
