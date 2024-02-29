#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>chrome.topSites</code> API to access the top sites (i.e. most visited sites) that are displayed on the new tab page. These do not include shortcuts customized by the user."]
# [wasm_bindgen (js_namespace = chrome)]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "topSites.MostVisitedURL" , typescript_type = "topSites.MostVisitedURL")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "An object encapsulating a most visited URL, such as the default shortcuts on the new tab page."]
    pub type MostVisitedURL;
    # [wasm_bindgen (method , getter , js_class = MostVisitedURL)]
    #[doc = "The title of the page"]
    pub fn title(this: &MostVisitedURL) -> ::js_sys::JsString;
    # [wasm_bindgen (method , getter , js_class = MostVisitedURL)]
    #[doc = "The most visited URL."]
    pub fn url(this: &MostVisitedURL) -> ::js_sys::JsString;
    #[doc = "Gets a list of top sites."]
    #[wasm_bindgen(js_name = "topSites.get", catch)]
    pub async fn get() -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
}
