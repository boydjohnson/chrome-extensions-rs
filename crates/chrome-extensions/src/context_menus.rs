#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>chrome.contextMenus</code> API to add items to Google Chrome's context menu. You can choose what types of objects your context menu additions apply to, such as images, hyperlinks, and pages."]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "ContextType" , typescript_type = "ContextType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The different contexts a menu can appear in. Specifying 'all' is equivalent to the combination of all other contexts except for 'launcher'. The 'launcher' context is only supported by apps and is used to add menu items to the context menu that appears when clicking the app icon in the launcher/taskbar/dock/etc. Different platforms might put limitations on what is actually supported in a launcher context menu."]
    pub type ContextType;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "ItemType" , typescript_type = "ItemType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The type of menu item."]
    pub type ItemType;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "OnClickData" , typescript_type = "OnClickData")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Information sent when a context menu item is clicked."]
    pub type OnClickData;
    # [wasm_bindgen (method , getter , js_class = OnClickData)]
    #[doc = "A flag indicating the state of a checkbox or radio item after it is clicked."]
    pub fn checked(this: &OnClickData) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = OnClickData)]
    #[doc = "A flag indicating whether the element is editable (text input, textarea, etc.)."]
    pub fn editable(this: &OnClickData) -> ::js_sys::Boolean;
    # [wasm_bindgen (method , getter , js_class = OnClickData)]
    #[doc = " The <a href='webNavigation#frame_ids'>ID of the frame</a> of the element where the context menu was clicked, if it was in a frame."]
    pub fn frameId(this: &OnClickData) -> Option<::js_sys::Number>;
    # [wasm_bindgen (method , getter , js_class = OnClickData)]
    #[doc = " The URL of the frame of the element where the context menu was clicked, if it was in a frame."]
    pub fn frameUrl(this: &OnClickData) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = OnClickData)]
    #[doc = "If the element is a link, the URL it points to."]
    pub fn linkUrl(this: &OnClickData) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = OnClickData)]
    #[doc = "One of 'image', 'video', or 'audio' if the context menu was activated on one of these types of elements."]
    pub fn mediaType(this: &OnClickData) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = OnClickData)]
    #[doc = "The ID of the menu item that was clicked."]
    pub fn menuItemId(this: &OnClickData) -> wasm_bindgen::JsValue;
    # [wasm_bindgen (method , getter , js_class = OnClickData)]
    #[doc = "The URL of the page where the menu item was clicked. This property is not set if the click occured in a context where there is no current page, such as in a launcher context menu."]
    pub fn pageUrl(this: &OnClickData) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = OnClickData)]
    #[doc = "The parent ID, if any, for the item clicked."]
    pub fn parentMenuItemId(this: &OnClickData) -> wasm_bindgen::JsValue;
    # [wasm_bindgen (method , getter , js_class = OnClickData)]
    #[doc = "The text for the context selection, if any."]
    pub fn selectionText(this: &OnClickData) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = OnClickData)]
    #[doc = "Will be present for elements with a 'src' URL."]
    pub fn srcUrl(this: &OnClickData) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = OnClickData)]
    #[doc = "A flag indicating the state of a checkbox or radio item before it was clicked."]
    pub fn wasChecked(this: &OnClickData) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "CreateProperties" , typescript_type = "CreateProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Properties of the new context menu item."]
    pub type CreateProperties;
    # [wasm_bindgen (method , getter , js_class = CreateProperties)]
    #[doc = "The initial state of a checkbox or radio button: <code>true</code> for selected, <code>false</code> for unselected. Only one radio button can be selected at a time in a given group."]
    pub fn checked(this: &CreateProperties) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = CreateProperties)]
    #[doc = "List of contexts this menu item will appear in. Defaults to <code>['page']</code>."]
    pub fn contexts(this: &CreateProperties) -> Option<::js_sys::Array>;
    # [wasm_bindgen (method , getter , js_class = CreateProperties)]
    #[doc = "Restricts the item to apply only to documents or frames whose URL matches one of the given patterns. For details on pattern formats, see <a href='/docs/extensions/develop/concepts/match-patterns'>Match Patterns</a>."]
    pub fn documentUrlPatterns(this: &CreateProperties) -> Option<::js_sys::Array>;
    # [wasm_bindgen (method , getter , js_class = CreateProperties)]
    #[doc = "Whether this context menu item is enabled or disabled. Defaults to <code>true</code>."]
    pub fn enabled(this: &CreateProperties) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = CreateProperties)]
    #[doc = "The unique ID to assign to this item. Mandatory for event pages. Cannot be the same as another ID for this extension."]
    pub fn id(this: &CreateProperties) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = CreateProperties)]
    #[doc = "The ID of a parent menu item; this makes the item a child of a previously added item."]
    pub fn parentId(this: &CreateProperties) -> wasm_bindgen::JsValue;
    # [wasm_bindgen (method , getter , js_class = CreateProperties)]
    #[doc = "Similar to <code>documentUrlPatterns</code>, filters based on the <code>src</code> attribute of <code>img</code>, <code>audio</code>, and <code>video</code> tags and the <code>href</code> attribute of <code>a</code> tags."]
    pub fn targetUrlPatterns(this: &CreateProperties) -> Option<::js_sys::Array>;
    # [wasm_bindgen (method , getter , js_class = CreateProperties)]
    #[doc = "The text to display in the item; this is <em>required</em> unless <code>type</code> is <code>separator</code>. When the context is <code>selection</code>, use <code>%s</code> within the string to show the selected text. For example, if this parameter's value is \"Translate '%s' to Pig Latin\" and the user selects the word \"cool\", the context menu item for the selection is \"Translate 'cool' to Pig Latin\"."]
    pub fn title(this: &CreateProperties) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = CreateProperties)]
    #[doc = "The type of menu item. Defaults to <code>normal</code>."]
    pub fn type_(this: &CreateProperties) -> Option<i32>;
    # [wasm_bindgen (method , getter , js_class = CreateProperties)]
    #[doc = "Whether the item is visible in the menu."]
    pub fn visible(this: &CreateProperties) -> Option<::js_sys::Boolean>;
}
