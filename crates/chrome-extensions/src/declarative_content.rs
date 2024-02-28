#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>chrome.declarativeContent</code> API to take actions depending on the content of a page, without requiring permission to read the page's content."]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: web_sys :: Blob , js_name = "ImageDataType" , typescript_type = "ImageDataType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "See <a href=\"https://developer.mozilla.org/en-US/docs/Web/API/ImageData\">https://developer.mozilla.org/en-US/docs/Web/API/ImageData</a>."]
    pub type ImageDataType;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "PageStateMatcherInstanceType" , typescript_type = "PageStateMatcherInstanceType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type PageStateMatcherInstanceType;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "ShowPageActionInstanceType" , typescript_type = "ShowPageActionInstanceType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type ShowPageActionInstanceType;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "ShowActionInstanceType" , typescript_type = "ShowActionInstanceType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type ShowActionInstanceType;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "SetIconInstanceType" , typescript_type = "SetIconInstanceType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type SetIconInstanceType;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "RequestContentScriptInstanceType" , typescript_type = "RequestContentScriptInstanceType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type RequestContentScriptInstanceType;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "PageStateMatcher" , typescript_type = "PageStateMatcher")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Matches the state of a web page based on various criteria."]
    pub type PageStateMatcher;
    # [wasm_bindgen (method , getter , js_class = PageStateMatcher)]
    #[doc = "Matches if all of the CSS selectors in the array match displayed elements in a frame with the same origin as the page's main frame. All selectors in this array must be <a href=\"http://www.w3.org/TR/selectors4/#compound\">compound selectors</a> to speed up matching. Note: Listing hundreds of CSS selectors or listing CSS selectors that match hundreds of times per page can slow down web sites."]
    pub fn css(this: &PageStateMatcher) -> Option<::js_sys::Array>;
    # [wasm_bindgen (method , getter , js_class = PageStateMatcher)]
    #[doc = ""]
    pub fn instanceType(this: &PageStateMatcher) -> PageStateMatcherInstanceType;
    # [wasm_bindgen (method , getter , js_class = PageStateMatcher)]
    #[doc = "Matches if the bookmarked state of the page is equal to the specified value. Requres the <a href='/docs/extensions/develop/concepts/declare-permissions'>bookmarks permission</a>."]
    pub fn isBookmarked(this: &PageStateMatcher) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = PageStateMatcher)]
    #[doc = "Matches if the conditions of the <code>UrlFilter</code> are fulfilled for the top-level URL of the page."]
    pub fn pageUrl(this: &PageStateMatcher) -> Option<crate::events::UrlFilter>;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "ShowPageAction" , typescript_type = "ShowPageAction")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "A declarative event action that sets the extension's $(ref:pageAction page action) to an enabled state while the corresponding conditions are met. This action can be used without <a href=\"/docs/extensions/develop/concepts/declare-permissions#host-permissions\">host permissions</a>, but the extension must have a page action. If the extension has the <a href=\"/docs/extensions/develop/concepts/activeTab\">activeTab</a> permission, clicking the page action grants access to the active tab.<p>On pages where the conditions are not met the extension's toolbar action will be grey-scale, and clicking it will open the context menu, instead of triggering the action.</p>"]
    pub type ShowPageAction;
    # [wasm_bindgen (method , getter , js_class = ShowPageAction)]
    #[doc = ""]
    pub fn instanceType(this: &ShowPageAction) -> ShowPageActionInstanceType;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "ShowAction" , typescript_type = "ShowAction")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "A declarative event action that sets the extension's toolbar $(ref:action action) to an enabled state while the corresponding conditions are met. This action can be used without <a href=\"/docs/extensions/develop/concepts/declare-permissions#host-permissions\">host permissions</a>. If the extension has the <a href=\"/docs/extensions/develop/concepts/activeTab\">activeTab</a> permission, clicking the page action grants access to the active tab.<p>On pages where the conditions are not met the extension's toolbar action will be grey-scale, and clicking it will open the context menu, instead of triggering the action.</p>"]
    pub type ShowAction;
    # [wasm_bindgen (method , getter , js_class = ShowAction)]
    #[doc = ""]
    pub fn instanceType(this: &ShowAction) -> ShowActionInstanceType;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "SetIcon" , typescript_type = "SetIcon")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Declarative event action that sets the n-<abbr title=\"device-independent pixel\">dip</abbr> square icon for the extension's $(ref:pageAction page action) or $(ref:browserAction browser action) while the corresponding conditions are met. This action can be used without <a href=\"/docs/extensions/develop/concepts/declare-permissions#host-permissions\">host permissions</a>, but the extension must have a page or browser action.<p>Exactly one of <code>imageData</code> or <code>path</code> must be specified. Both are dictionaries mapping a number of pixels to an image representation. The image representation in <code>imageData</code> is an <a href=\"https://developer.mozilla.org/en-US/docs/Web/API/ImageData\">ImageData</a> object; for example, from a <code>canvas</code> element, while the image representation in <code>path</code> is the path to an image file relative to the extension's manifest. If <code>scale</code> screen pixels fit into a device-independent pixel, the <code>scale * n</code> icon is used. If that scale is missing, another image is resized to the required size.</p>"]
    pub type SetIcon;
    # [wasm_bindgen (method , getter , js_class = SetIcon)]
    #[doc = "Either an <code>ImageData</code> object or a dictionary {size -> ImageData} representing an icon to be set. If the icon is specified as a dictionary, the image used is chosen depending on the screen's pixel density. If the number of image pixels that fit into one screen space unit equals <code>scale</code>, then an image with size <code>scale * n</code> is selected, where <i>n</i> is the size of the icon in the UI. At least one image must be specified. Note that <code>details.imageData = foo</code> is equivalent to <code>details.imageData = {'16': foo}</code>."]
    pub fn imageData(this: &SetIcon) -> wasm_bindgen::JsValue;
    # [wasm_bindgen (method , getter , js_class = SetIcon)]
    #[doc = ""]
    pub fn instanceType(this: &SetIcon) -> SetIconInstanceType;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "RequestContentScript" , typescript_type = "RequestContentScript")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Declarative event action that injects a content script. <p><b>WARNING:</b> This action is still experimental and is not supported on stable builds of Chrome.</p>"]
    pub type RequestContentScript;
    # [wasm_bindgen (method , getter , js_class = RequestContentScript)]
    #[doc = "Whether the content script runs in all frames of the matching page, or in only the top frame. Default is <code>false</code>."]
    pub fn allFrames(this: &RequestContentScript) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = RequestContentScript)]
    #[doc = "Names of CSS files to be injected as a part of the content script."]
    pub fn css(this: &RequestContentScript) -> Option<::js_sys::Array>;
    # [wasm_bindgen (method , getter , js_class = RequestContentScript)]
    #[doc = ""]
    pub fn instanceType(this: &RequestContentScript) -> RequestContentScriptInstanceType;
    # [wasm_bindgen (method , getter , js_class = RequestContentScript)]
    #[doc = "Names of JavaScript files to be injected as a part of the content script."]
    pub fn js(this: &RequestContentScript) -> Option<::js_sys::Array>;
    # [wasm_bindgen (method , getter , js_class = RequestContentScript)]
    #[doc = "Whether to insert the content script on <code>about:blank</code> and <code>about:srcdoc</code>. Default is <code>false</code>."]
    pub fn matchAboutBlank(this: &RequestContentScript) -> Option<::js_sys::Boolean>;
}
