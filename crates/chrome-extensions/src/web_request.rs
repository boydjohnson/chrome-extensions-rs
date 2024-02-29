#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>chrome.webRequest</code> API to observe and analyze traffic and to intercept, block, or modify requests in-flight."]
# [wasm_bindgen (js_namespace = chrome)]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "webRequest.ResourceType" , typescript_type = "webRequest.ResourceType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type ResourceType;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "webRequest.OnBeforeRequestOptions" , typescript_type = "webRequest.OnBeforeRequestOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type OnBeforeRequestOptions;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "webRequest.OnBeforeSendHeadersOptions" , typescript_type = "webRequest.OnBeforeSendHeadersOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type OnBeforeSendHeadersOptions;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "webRequest.OnSendHeadersOptions" , typescript_type = "webRequest.OnSendHeadersOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type OnSendHeadersOptions;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "webRequest.OnHeadersReceivedOptions" , typescript_type = "webRequest.OnHeadersReceivedOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type OnHeadersReceivedOptions;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "webRequest.OnAuthRequiredOptions" , typescript_type = "webRequest.OnAuthRequiredOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type OnAuthRequiredOptions;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "webRequest.OnResponseStartedOptions" , typescript_type = "webRequest.OnResponseStartedOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type OnResponseStartedOptions;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "webRequest.OnBeforeRedirectOptions" , typescript_type = "webRequest.OnBeforeRedirectOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type OnBeforeRedirectOptions;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "webRequest.OnCompletedOptions" , typescript_type = "webRequest.OnCompletedOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type OnCompletedOptions;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "webRequest.OnErrorOccurredOptions" , typescript_type = "webRequest.OnErrorOccurredOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type OnErrorOccurredOptions;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "webRequest.RequestFilter" , typescript_type = "webRequest.RequestFilter")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "An object describing filters to apply to webRequest events."]
    pub type RequestFilter;
    # [wasm_bindgen (method , getter , js_class = RequestFilter)]
    #[doc = ""]
    pub fn tabId(this: &RequestFilter) -> Option<::js_sys::Number>;
    # [wasm_bindgen (method , getter , js_class = RequestFilter)]
    #[doc = "A list of request types. Requests that cannot match any of the types will be filtered out."]
    pub fn types(this: &RequestFilter) -> Option<::js_sys::Array>;
    # [wasm_bindgen (method , getter , js_class = RequestFilter)]
    #[doc = "A list of URLs or URL patterns. Requests that cannot match any of the URLs will be filtered out."]
    pub fn urls(this: &RequestFilter) -> ::js_sys::Array;
    # [wasm_bindgen (method , getter , js_class = RequestFilter)]
    #[doc = ""]
    pub fn windowId(this: &RequestFilter) -> Option<::js_sys::Number>;
    # [wasm_bindgen (extends = :: js_sys :: Array , js_name = "webRequest.HttpHeaders" , typescript_type = "webRequest.HttpHeaders")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "An array of HTTP headers. Each header is represented as a dictionary containing the keys <code>name</code> and either <code>value</code> or <code>binaryValue</code>."]
    pub type HttpHeaders;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "webRequest.BlockingResponse" , typescript_type = "webRequest.BlockingResponse")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Returns value for event handlers that have the 'blocking' extraInfoSpec applied. Allows the event handler to modify network requests."]
    pub type BlockingResponse;
    # [wasm_bindgen (method , getter , js_class = BlockingResponse)]
    #[doc = "Only used as a response to the onAuthRequired event. If set, the request is made using the supplied credentials."]
    pub fn authCredentials(this: &BlockingResponse) -> Option<::js_sys::Object>;
    # [wasm_bindgen (method , getter , js_class = BlockingResponse)]
    #[doc = "If true, the request is cancelled. This prevents the request from being sent. This can be used as a response to the onBeforeRequest, onBeforeSendHeaders, onHeadersReceived and onAuthRequired events."]
    pub fn cancel(this: &BlockingResponse) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = BlockingResponse)]
    #[doc = "Only used as a response to the onBeforeRequest and onHeadersReceived events. If set, the original request is prevented from being sent/completed and is instead redirected to the given URL. Redirections to non-HTTP schemes such as <code>data:</code> are allowed. Redirects initiated by a redirect action use the original request method for the redirect, with one exception: If the redirect is initiated at the onHeadersReceived stage, then the redirect will be issued using the GET method. Redirects from URLs with <code>ws://</code> and <code>wss://</code> schemes are <b>ignored</b>."]
    pub fn redirectUrl(this: &BlockingResponse) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = BlockingResponse)]
    #[doc = "Only used as a response to the onBeforeSendHeaders event. If set, the request is made with these request headers instead."]
    pub fn requestHeaders(this: &BlockingResponse) -> Option<HttpHeaders>;
    # [wasm_bindgen (method , getter , js_class = BlockingResponse)]
    #[doc = "Only used as a response to the onHeadersReceived event. If set, the server is assumed to have responded with these response headers instead. Only return <code>responseHeaders</code> if you really want to modify the headers in order to limit the number of conflicts (only one extension may modify <code>responseHeaders</code> for each request)."]
    pub fn responseHeaders(this: &BlockingResponse) -> Option<HttpHeaders>;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "webRequest.UploadData" , typescript_type = "webRequest.UploadData")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Contains data uploaded in a URL request."]
    pub type UploadData;
    # [wasm_bindgen (method , getter , js_class = UploadData)]
    #[doc = "An ArrayBuffer with a copy of the data."]
    pub fn bytes(this: &UploadData) -> ::wasm_bindgen::JsValue;
    # [wasm_bindgen (method , getter , js_class = UploadData)]
    #[doc = "A string with the file's path and name."]
    pub fn file(this: &UploadData) -> Option<::js_sys::JsString>;
    #[wasm_bindgen(
        js_name = "webRequest.FormDataItem",
        typescript_type = "webRequest.FormDataItem"
    )]
    #[derive(Debug, Clone, PartialEq)]
    #[doc = "Contains data passed within form data. For urlencoded form it is stored as string if data is utf-8 string and as ArrayBuffer otherwise. For form-data it is ArrayBuffer. If form-data represents uploading file, it is string with filename, if the filename is provided."]
    pub type FormDataItem;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "webRequest.IgnoredActionType" , typescript_type = "webRequest.IgnoredActionType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type IgnoredActionType;
    #[doc = "Needs to be called when the behavior of the webRequest handlers has changed to prevent incorrect handling due to caching. This function call is expensive. Don't call it often."]
    #[wasm_bindgen(js_name = "webRequest.handlerBehaviorChanged", catch)]
    pub async fn handlerBehaviorChanged() -> Result<(), ::wasm_bindgen::JsValue>;
}
