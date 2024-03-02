#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>chrome.history</code> API to interact with the browser's record of visited pages. You can add, remove, and query for URLs in the browser's history. To override the history page with your own version, see <a href='develop/ui/override-chrome-pages'>Override Pages</a>."]
# [wasm_bindgen (js_namespace = chrome)]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "history.TransitionType" , typescript_type = "history.TransitionType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The <a href='#transition_types'>transition type</a> for this visit from its referrer."]
    pub type TransitionType;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "history.HistoryItem" , typescript_type = "history.HistoryItem")]
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
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "history.VisitItem" , typescript_type = "history.VisitItem")]
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
    pub fn transition(this: &VisitItem) -> TransitionType;
    # [wasm_bindgen (method , getter , js_class = VisitItem)]
    #[doc = "The unique identifier for this visit."]
    pub fn visitId(this: &VisitItem) -> ::js_sys::JsString;
    # [wasm_bindgen (method , getter , js_class = VisitItem)]
    #[doc = "When this visit occurred, represented in milliseconds since the epoch."]
    pub fn visitTime(this: &VisitItem) -> Option<::js_sys::Number>;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "history.UrlDetails" , typescript_type = "history.UrlDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type UrlDetails;
    # [wasm_bindgen (method , getter , js_class = UrlDetails)]
    #[doc = "The URL for the operation. It must be in the format as returned from a call to history.search."]
    pub fn url(this: &UrlDetails) -> ::js_sys::JsString;
    #[doc = "Searches the history for the last visit time of each page matching the query."]
    #[wasm_bindgen(js_name = "history.search", catch)]
    pub async fn search(
        query: ::js_sys::Object,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Retrieves information about visits to a URL."]
    #[wasm_bindgen(js_name = "history.getVisits", catch)]
    pub async fn getVisits(
        details: UrlDetails,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Adds a URL to the history at the current time with a <a href='#transition_types'>transition type</a> of \"link\"."]
    #[wasm_bindgen(js_name = "history.addUrl", catch)]
    pub async fn addUrl(details: UrlDetails) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Removes all occurrences of the given URL from the history."]
    #[wasm_bindgen(js_name = "history.deleteUrl", catch)]
    pub async fn deleteUrl(details: UrlDetails) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Removes all items within the specified date range from the history.  Pages will not be removed from the history unless all visits fall within the range."]
    #[wasm_bindgen(js_name = "history.deleteRange", catch)]
    pub async fn deleteRange(range: ::js_sys::Object) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Deletes all items from the history."]
    #[wasm_bindgen(js_name = "history.deleteAll", catch)]
    pub async fn deleteAll() -> Result<(), ::wasm_bindgen::JsValue>;
}
#[wasm_bindgen]
pub async fn history_search(
    query: ::js_sys::Object,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    search(query).await
}
#[wasm_bindgen]
pub async fn history_get_visits(
    details: UrlDetails,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    getVisits(details).await
}
#[wasm_bindgen]
pub async fn history_add_url(details: UrlDetails) -> Result<(), ::wasm_bindgen::JsValue> {
    addUrl(details).await
}
#[wasm_bindgen]
pub async fn history_delete_url(details: UrlDetails) -> Result<(), ::wasm_bindgen::JsValue> {
    deleteUrl(details).await
}
#[wasm_bindgen]
pub async fn history_delete_range(range: ::js_sys::Object) -> Result<(), ::wasm_bindgen::JsValue> {
    deleteRange(range).await
}
#[wasm_bindgen]
pub async fn history_delete_all() -> Result<(), ::wasm_bindgen::JsValue> {
    deleteAll().await
}
