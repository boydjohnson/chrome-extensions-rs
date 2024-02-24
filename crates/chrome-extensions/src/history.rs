#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>chrome.history</code> API to interact with the browser's record of visited pages. You can add, remove, and query for URLs in the browser's history. To override the history page with your own version, see <a href='develop/ui/override-chrome-pages'>Override Pages</a>."]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "TransitionType" , typescript_type = "TransitionType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The <a href='#transition_types'>transition type</a> for this visit from its referrer."]
    pub type TransitionType;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "HistoryItem" , typescript_type = "HistoryItem")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "An object encapsulating one result of a history query."]
    pub type HistoryItem;
    # [wasm_bindgen (method , getter , js_class = HistoryItem)]
    #[doc = "The unique identifier for the item."]
    pub fn id(this: &HistoryItem) -> ::js_sys::JsString;
    # [wasm_bindgen (method , getter , js_class = HistoryItem)]
    #[doc = "When this page was last loaded, represented in milliseconds since the epoch."]
    pub fn lastVisitTime(this: &HistoryItem) -> Option<::js_sys::Number>;
    # [wasm_bindgen (method , getter , js_class = HistoryItem)]
    #[doc = "The title of the page when it was last loaded."]
    pub fn title(this: &HistoryItem) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = HistoryItem)]
    #[doc = "The number of times the user has navigated to this page by typing in the address."]
    pub fn typedCount(this: &HistoryItem) -> Option<::js_sys::Number>;
    # [wasm_bindgen (method , getter , js_class = HistoryItem)]
    #[doc = "The URL navigated to by a user."]
    pub fn url(this: &HistoryItem) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = HistoryItem)]
    #[doc = "The number of times the user has navigated to this page."]
    pub fn visitCount(this: &HistoryItem) -> Option<::js_sys::Number>;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "VisitItem" , typescript_type = "VisitItem")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "An object encapsulating one visit to a URL."]
    pub type VisitItem;
    # [wasm_bindgen (method , getter , js_class = VisitItem)]
    #[doc = "The unique identifier for the corresponding $(ref:history.HistoryItem)."]
    pub fn id(this: &VisitItem) -> ::js_sys::JsString;
    # [wasm_bindgen (method , getter , js_class = VisitItem)]
    #[doc = "True if the visit originated on this device. False if it was synced from a different device."]
    pub fn isLocal(this: &VisitItem) -> ::js_sys::Boolean;
    # [wasm_bindgen (method , getter , js_class = VisitItem)]
    #[doc = "The visit ID of the referrer."]
    pub fn referringVisitId(this: &VisitItem) -> ::js_sys::JsString;
    # [wasm_bindgen (method , getter , js_class = VisitItem)]
    #[doc = "The <a href='#transition_types'>transition type</a> for this visit from its referrer."]
    pub fn transition(this: &VisitItem) -> i32;
    # [wasm_bindgen (method , getter , js_class = VisitItem)]
    #[doc = "The unique identifier for this visit."]
    pub fn visitId(this: &VisitItem) -> ::js_sys::JsString;
    # [wasm_bindgen (method , getter , js_class = VisitItem)]
    #[doc = "When this visit occurred, represented in milliseconds since the epoch."]
    pub fn visitTime(this: &VisitItem) -> Option<::js_sys::Number>;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "UrlDetails" , typescript_type = "UrlDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type UrlDetails;
    # [wasm_bindgen (method , getter , js_class = UrlDetails)]
    #[doc = "The URL for the operation. It must be in the format as returned from a call to <code>history.search()</code>."]
    pub fn url(this: &UrlDetails) -> ::js_sys::JsString;
}
