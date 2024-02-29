#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "<em><strong>Note:</strong> this API is deprecated. Check out the $(ref:declarativeNetRequest) API instead.</em> Use the <code>chrome.declarativeWebRequest</code> API to intercept, block, or modify requests in-flight. It is significantly faster than the <a href='webRequest'><code>chrome.webRequest</code> API</a> because you can register rules that are evaluated in the browser rather than the JavaScript engine, which reduces roundtrip latencies and allows higher efficiency."]
# [wasm_bindgen (js_namespace = chrome)]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "declarativeWebRequest.RequestMatcherInstanceType" , typescript_type = "declarativeWebRequest.RequestMatcherInstanceType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type RequestMatcherInstanceType;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "declarativeWebRequest.CancelRequestInstanceType" , typescript_type = "declarativeWebRequest.CancelRequestInstanceType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type CancelRequestInstanceType;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "declarativeWebRequest.RedirectRequestInstanceType" , typescript_type = "declarativeWebRequest.RedirectRequestInstanceType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type RedirectRequestInstanceType;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "declarativeWebRequest.RedirectToTransparentImageInstanceType" , typescript_type = "declarativeWebRequest.RedirectToTransparentImageInstanceType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type RedirectToTransparentImageInstanceType;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "declarativeWebRequest.RedirectToEmptyDocumentInstanceType" , typescript_type = "declarativeWebRequest.RedirectToEmptyDocumentInstanceType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type RedirectToEmptyDocumentInstanceType;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "declarativeWebRequest.RedirectByRegExInstanceType" , typescript_type = "declarativeWebRequest.RedirectByRegExInstanceType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type RedirectByRegExInstanceType;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "declarativeWebRequest.SetRequestHeaderInstanceType" , typescript_type = "declarativeWebRequest.SetRequestHeaderInstanceType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type SetRequestHeaderInstanceType;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "declarativeWebRequest.RemoveRequestHeaderInstanceType" , typescript_type = "declarativeWebRequest.RemoveRequestHeaderInstanceType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type RemoveRequestHeaderInstanceType;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "declarativeWebRequest.AddResponseHeaderInstanceType" , typescript_type = "declarativeWebRequest.AddResponseHeaderInstanceType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type AddResponseHeaderInstanceType;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "declarativeWebRequest.RemoveResponseHeaderInstanceType" , typescript_type = "declarativeWebRequest.RemoveResponseHeaderInstanceType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type RemoveResponseHeaderInstanceType;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "declarativeWebRequest.IgnoreRulesInstanceType" , typescript_type = "declarativeWebRequest.IgnoreRulesInstanceType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type IgnoreRulesInstanceType;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "declarativeWebRequest.SendMessageToExtensionInstanceType" , typescript_type = "declarativeWebRequest.SendMessageToExtensionInstanceType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type SendMessageToExtensionInstanceType;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "declarativeWebRequest.AddRequestCookieInstanceType" , typescript_type = "declarativeWebRequest.AddRequestCookieInstanceType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type AddRequestCookieInstanceType;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "declarativeWebRequest.AddResponseCookieInstanceType" , typescript_type = "declarativeWebRequest.AddResponseCookieInstanceType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type AddResponseCookieInstanceType;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "declarativeWebRequest.EditRequestCookieInstanceType" , typescript_type = "declarativeWebRequest.EditRequestCookieInstanceType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type EditRequestCookieInstanceType;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "declarativeWebRequest.EditResponseCookieInstanceType" , typescript_type = "declarativeWebRequest.EditResponseCookieInstanceType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type EditResponseCookieInstanceType;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "declarativeWebRequest.RemoveRequestCookieInstanceType" , typescript_type = "declarativeWebRequest.RemoveRequestCookieInstanceType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type RemoveRequestCookieInstanceType;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "declarativeWebRequest.RemoveResponseCookieInstanceType" , typescript_type = "declarativeWebRequest.RemoveResponseCookieInstanceType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type RemoveResponseCookieInstanceType;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "declarativeWebRequest.Stage" , typescript_type = "declarativeWebRequest.Stage")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type Stage;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "declarativeWebRequest.HeaderFilter" , typescript_type = "declarativeWebRequest.HeaderFilter")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Filters request headers for various criteria. Multiple criteria are evaluated as a conjunction."]
    pub type HeaderFilter;
    # [wasm_bindgen (method , getter , js_class = HeaderFilter)]
    #[doc = "Matches if the header name contains all of the specified strings."]
    pub fn nameContains(this: &HeaderFilter) -> wasm_bindgen::JsValue;
    # [wasm_bindgen (method , getter , js_class = HeaderFilter)]
    #[doc = "Matches if the header name is equal to the specified string."]
    pub fn nameEquals(this: &HeaderFilter) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = HeaderFilter)]
    #[doc = "Matches if the header name starts with the specified string."]
    pub fn namePrefix(this: &HeaderFilter) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = HeaderFilter)]
    #[doc = "Matches if the header name ends with the specified string."]
    pub fn nameSuffix(this: &HeaderFilter) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = HeaderFilter)]
    #[doc = "Matches if the header value contains all of the specified strings."]
    pub fn valueContains(this: &HeaderFilter) -> wasm_bindgen::JsValue;
    # [wasm_bindgen (method , getter , js_class = HeaderFilter)]
    #[doc = "Matches if the header value is equal to the specified string."]
    pub fn valueEquals(this: &HeaderFilter) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = HeaderFilter)]
    #[doc = "Matches if the header value starts with the specified string."]
    pub fn valuePrefix(this: &HeaderFilter) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = HeaderFilter)]
    #[doc = "Matches if the header value ends with the specified string."]
    pub fn valueSuffix(this: &HeaderFilter) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "declarativeWebRequest.RequestMatcher" , typescript_type = "declarativeWebRequest.RequestMatcher")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Matches network events by various criteria."]
    pub type RequestMatcher;
    # [wasm_bindgen (method , getter , js_class = RequestMatcher)]
    #[doc = "Matches if the MIME media type of a response (from the HTTP Content-Type header) is contained in the list."]
    pub fn contentType(this: &RequestMatcher) -> Option<::js_sys::Array>;
    # [wasm_bindgen (method , getter , js_class = RequestMatcher)]
    #[doc = "Matches if the MIME media type of a response (from the HTTP Content-Type header) is <em>not</em> contained in the list."]
    pub fn excludeContentType(this: &RequestMatcher) -> Option<::js_sys::Array>;
    # [wasm_bindgen (method , getter , js_class = RequestMatcher)]
    #[doc = "Matches if none of the request headers is matched by any of the HeaderFilters."]
    pub fn excludeRequestHeaders(this: &RequestMatcher) -> Option<::js_sys::Array>;
    # [wasm_bindgen (method , getter , js_class = RequestMatcher)]
    #[doc = "Matches if none of the response headers is matched by any of the HeaderFilters."]
    pub fn excludeResponseHeaders(this: &RequestMatcher) -> Option<::js_sys::Array>;
    # [wasm_bindgen (method , getter , js_class = RequestMatcher)]
    #[doc = "Matches if the conditions of the UrlFilter are fulfilled for the 'first party' URL of the request. The 'first party' URL of a request, when present, can be different from the request's target URL, and describes what is considered 'first party' for the sake of third-party checks for cookies."]
    pub fn firstPartyForCookiesUrl(this: &RequestMatcher) -> Option<crate::events::UrlFilter>;
    # [wasm_bindgen (method , getter , js_class = RequestMatcher)]
    #[doc = ""]
    pub fn instanceType(this: &RequestMatcher) -> RequestMatcherInstanceType;
    # [wasm_bindgen (method , getter , js_class = RequestMatcher)]
    #[doc = "Matches if some of the request headers is matched by one of the HeaderFilters."]
    pub fn requestHeaders(this: &RequestMatcher) -> Option<::js_sys::Array>;
    # [wasm_bindgen (method , getter , js_class = RequestMatcher)]
    #[doc = "Matches if the request type of a request is contained in the list. Requests that cannot match any of the types will be filtered out."]
    pub fn resourceType(this: &RequestMatcher) -> Option<::js_sys::Array>;
    # [wasm_bindgen (method , getter , js_class = RequestMatcher)]
    #[doc = "Matches if some of the response headers is matched by one of the HeaderFilters."]
    pub fn responseHeaders(this: &RequestMatcher) -> Option<::js_sys::Array>;
    # [wasm_bindgen (method , getter , js_class = RequestMatcher)]
    #[doc = "Contains a list of strings describing stages. Allowed values are 'onBeforeRequest', 'onBeforeSendHeaders', 'onHeadersReceived', 'onAuthRequired'. If this attribute is present, then it limits the applicable stages to those listed. Note that the whole condition is only applicable in stages compatible with all attributes."]
    pub fn stages(this: &RequestMatcher) -> Option<::js_sys::Array>;
    # [wasm_bindgen (method , getter , js_class = RequestMatcher)]
    #[doc = "If set to true, matches requests that are subject to third-party cookie policies. If set to false, matches all other requests."]
    pub fn thirdPartyForCookies(this: &RequestMatcher) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = RequestMatcher)]
    #[doc = "Matches if the conditions of the UrlFilter are fulfilled for the URL of the request."]
    pub fn url(this: &RequestMatcher) -> Option<crate::events::UrlFilter>;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "declarativeWebRequest.CancelRequest" , typescript_type = "declarativeWebRequest.CancelRequest")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Declarative event action that cancels a network request."]
    pub type CancelRequest;
    # [wasm_bindgen (method , getter , js_class = CancelRequest)]
    #[doc = ""]
    pub fn instanceType(this: &CancelRequest) -> CancelRequestInstanceType;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "declarativeWebRequest.RedirectRequest" , typescript_type = "declarativeWebRequest.RedirectRequest")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Declarative event action that redirects a network request."]
    pub type RedirectRequest;
    # [wasm_bindgen (method , getter , js_class = RedirectRequest)]
    #[doc = ""]
    pub fn instanceType(this: &RedirectRequest) -> RedirectRequestInstanceType;
    # [wasm_bindgen (method , getter , js_class = RedirectRequest)]
    #[doc = "Destination to where the request is redirected."]
    pub fn redirectUrl(this: &RedirectRequest) -> ::js_sys::JsString;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "declarativeWebRequest.declarativeWebRequest.RedirectToTransparentImage" , typescript_type = "declarativeWebRequest.declarativeWebRequest.RedirectToTransparentImage")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Declarative event action that redirects a network request to a transparent image."]
    pub type RedirectToTransparentImage;
    # [wasm_bindgen (method , getter , js_class = RedirectToTransparentImage)]
    #[doc = ""]
    pub fn instanceType(
        this: &RedirectToTransparentImage,
    ) -> RedirectToTransparentImageInstanceType;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "declarativeWebRequest.declarativeWebRequest.RedirectToEmptyDocument" , typescript_type = "declarativeWebRequest.declarativeWebRequest.RedirectToEmptyDocument")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Declarative event action that redirects a network request to an empty document."]
    pub type RedirectToEmptyDocument;
    # [wasm_bindgen (method , getter , js_class = RedirectToEmptyDocument)]
    #[doc = ""]
    pub fn instanceType(this: &RedirectToEmptyDocument) -> RedirectToEmptyDocumentInstanceType;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "declarativeWebRequest.declarativeWebRequest.RedirectByRegEx" , typescript_type = "declarativeWebRequest.declarativeWebRequest.RedirectByRegEx")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Redirects a request by applying a regular expression on the URL. The regular expressions use the <a href=\"https://github.com/google/re2/blob/master/doc/syntax.txt\">RE2 syntax</a>."]
    pub type RedirectByRegEx;
    # [wasm_bindgen (method , getter , js_class = RedirectByRegEx)]
    #[doc = "A match pattern that may contain capture groups. Capture groups are referenced in the Perl syntax ($1, $2, ...) instead of the RE2 syntax (\\1, \\2, ...) in order to be closer to JavaScript Regular Expressions."]
    pub fn from(this: &RedirectByRegEx) -> ::js_sys::JsString;
    # [wasm_bindgen (method , getter , js_class = RedirectByRegEx)]
    #[doc = ""]
    pub fn instanceType(this: &RedirectByRegEx) -> RedirectByRegExInstanceType;
    # [wasm_bindgen (method , getter , js_class = RedirectByRegEx)]
    #[doc = "Destination pattern."]
    pub fn to(this: &RedirectByRegEx) -> ::js_sys::JsString;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "declarativeWebRequest.declarativeWebRequest.SetRequestHeader" , typescript_type = "declarativeWebRequest.declarativeWebRequest.SetRequestHeader")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Sets the request header of the specified name to the specified value. If a header with the specified name did not exist before, a new one is created. Header name comparison is always case-insensitive. Each request header name occurs only once in each request."]
    pub type SetRequestHeader;
    # [wasm_bindgen (method , getter , js_class = SetRequestHeader)]
    #[doc = ""]
    pub fn instanceType(this: &SetRequestHeader) -> SetRequestHeaderInstanceType;
    # [wasm_bindgen (method , getter , js_class = SetRequestHeader)]
    #[doc = "HTTP request header name."]
    pub fn name(this: &SetRequestHeader) -> ::js_sys::JsString;
    # [wasm_bindgen (method , getter , js_class = SetRequestHeader)]
    #[doc = "HTTP request header value."]
    pub fn value(this: &SetRequestHeader) -> ::js_sys::JsString;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "declarativeWebRequest.declarativeWebRequest.RemoveRequestHeader" , typescript_type = "declarativeWebRequest.declarativeWebRequest.RemoveRequestHeader")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Removes the request header of the specified name. Do not use SetRequestHeader and RemoveRequestHeader with the same header name on the same request. Each request header name occurs only once in each request."]
    pub type RemoveRequestHeader;
    # [wasm_bindgen (method , getter , js_class = RemoveRequestHeader)]
    #[doc = ""]
    pub fn instanceType(this: &RemoveRequestHeader) -> RemoveRequestHeaderInstanceType;
    # [wasm_bindgen (method , getter , js_class = RemoveRequestHeader)]
    #[doc = "HTTP request header name (case-insensitive)."]
    pub fn name(this: &RemoveRequestHeader) -> ::js_sys::JsString;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "declarativeWebRequest.declarativeWebRequest.AddResponseHeader" , typescript_type = "declarativeWebRequest.declarativeWebRequest.AddResponseHeader")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Adds the response header to the response of this web request. As multiple response headers may share the same name, you need to first remove and then add a new response header in order to replace one."]
    pub type AddResponseHeader;
    # [wasm_bindgen (method , getter , js_class = AddResponseHeader)]
    #[doc = ""]
    pub fn instanceType(this: &AddResponseHeader) -> AddResponseHeaderInstanceType;
    # [wasm_bindgen (method , getter , js_class = AddResponseHeader)]
    #[doc = "HTTP response header name."]
    pub fn name(this: &AddResponseHeader) -> ::js_sys::JsString;
    # [wasm_bindgen (method , getter , js_class = AddResponseHeader)]
    #[doc = "HTTP response header value."]
    pub fn value(this: &AddResponseHeader) -> ::js_sys::JsString;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "declarativeWebRequest.declarativeWebRequest.RemoveResponseHeader" , typescript_type = "declarativeWebRequest.declarativeWebRequest.RemoveResponseHeader")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Removes all response headers of the specified names and values."]
    pub type RemoveResponseHeader;
    # [wasm_bindgen (method , getter , js_class = RemoveResponseHeader)]
    #[doc = ""]
    pub fn instanceType(this: &RemoveResponseHeader) -> RemoveResponseHeaderInstanceType;
    # [wasm_bindgen (method , getter , js_class = RemoveResponseHeader)]
    #[doc = "HTTP request header name (case-insensitive)."]
    pub fn name(this: &RemoveResponseHeader) -> ::js_sys::JsString;
    # [wasm_bindgen (method , getter , js_class = RemoveResponseHeader)]
    #[doc = "HTTP request header value (case-insensitive)."]
    pub fn value(this: &RemoveResponseHeader) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "declarativeWebRequest.declarativeWebRequest.IgnoreRules" , typescript_type = "declarativeWebRequest.declarativeWebRequest.IgnoreRules")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Masks all rules that match the specified criteria."]
    pub type IgnoreRules;
    # [wasm_bindgen (method , getter , js_class = IgnoreRules)]
    #[doc = "If set, rules with the specified tag are ignored. This ignoring is not persisted, it affects only rules and their actions of the same network request stage. Note that rules are executed in descending order of their priorities. This action affects rules of lower priority than the current rule. Rules with the same priority may or may not be ignored."]
    pub fn hasTag(this: &IgnoreRules) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = IgnoreRules)]
    #[doc = ""]
    pub fn instanceType(this: &IgnoreRules) -> IgnoreRulesInstanceType;
    # [wasm_bindgen (method , getter , js_class = IgnoreRules)]
    #[doc = "If set, rules with a lower priority than the specified value are ignored. This boundary is not persisted, it affects only rules and their actions of the same network request stage."]
    pub fn lowerPriorityThan(this: &IgnoreRules) -> Option<::js_sys::Number>;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "declarativeWebRequest.declarativeWebRequest.SendMessageToExtension" , typescript_type = "declarativeWebRequest.declarativeWebRequest.SendMessageToExtension")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Triggers the $(ref:declarativeWebRequest.onMessage) event."]
    pub type SendMessageToExtension;
    # [wasm_bindgen (method , getter , js_class = SendMessageToExtension)]
    #[doc = ""]
    pub fn instanceType(this: &SendMessageToExtension) -> SendMessageToExtensionInstanceType;
    # [wasm_bindgen (method , getter , js_class = SendMessageToExtension)]
    #[doc = "The value that will be passed in the <code>message</code> attribute of the dictionary that is passed to the event handler."]
    pub fn message(this: &SendMessageToExtension) -> ::js_sys::JsString;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "declarativeWebRequest.declarativeWebRequest.RequestCookie" , typescript_type = "declarativeWebRequest.declarativeWebRequest.RequestCookie")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "A filter or specification of a cookie in HTTP Requests."]
    pub type RequestCookie;
    # [wasm_bindgen (method , getter , js_class = RequestCookie)]
    #[doc = "Name of a cookie."]
    pub fn name(this: &RequestCookie) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = RequestCookie)]
    #[doc = "Value of a cookie, may be padded in double-quotes."]
    pub fn value(this: &RequestCookie) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "declarativeWebRequest.declarativeWebRequest.ResponseCookie" , typescript_type = "declarativeWebRequest.declarativeWebRequest.ResponseCookie")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "A specification of a cookie in HTTP Responses."]
    pub type ResponseCookie;
    # [wasm_bindgen (method , getter , js_class = ResponseCookie)]
    #[doc = "Value of the Domain cookie attribute."]
    pub fn domain(this: &ResponseCookie) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = ResponseCookie)]
    #[doc = "Value of the Expires cookie attribute."]
    pub fn expires(this: &ResponseCookie) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = ResponseCookie)]
    #[doc = "Existence of the HttpOnly cookie attribute."]
    pub fn httpOnly(this: &ResponseCookie) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = ResponseCookie)]
    #[doc = "Value of the Max-Age cookie attribute"]
    pub fn maxAge(this: &ResponseCookie) -> Option<::js_sys::Number>;
    # [wasm_bindgen (method , getter , js_class = ResponseCookie)]
    #[doc = "Name of a cookie."]
    pub fn name(this: &ResponseCookie) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = ResponseCookie)]
    #[doc = "Value of the Path cookie attribute."]
    pub fn path(this: &ResponseCookie) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = ResponseCookie)]
    #[doc = "Existence of the Secure cookie attribute."]
    pub fn secure(this: &ResponseCookie) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = ResponseCookie)]
    #[doc = "Value of a cookie, may be padded in double-quotes."]
    pub fn value(this: &ResponseCookie) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "declarativeWebRequest.declarativeWebRequest.FilterResponseCookie" , typescript_type = "declarativeWebRequest.declarativeWebRequest.FilterResponseCookie")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "A filter of a cookie in HTTP Responses."]
    pub type FilterResponseCookie;
    # [wasm_bindgen (method , getter , js_class = FilterResponseCookie)]
    #[doc = "Inclusive lower bound on the cookie lifetime (specified in seconds after current time). Only cookies whose expiration date-time is set to 'now + ageLowerBound' or later fulfill this criterion. Session cookies do not meet the criterion of this filter. The cookie lifetime is calculated from either 'max-age' or 'expires' cookie attributes. If both are specified, 'max-age' is used to calculate the cookie lifetime."]
    pub fn ageLowerBound(this: &FilterResponseCookie) -> Option<::js_sys::Number>;
    # [wasm_bindgen (method , getter , js_class = FilterResponseCookie)]
    #[doc = "Inclusive upper bound on the cookie lifetime (specified in seconds after current time). Only cookies whose expiration date-time is in the interval [now, now + ageUpperBound] fulfill this criterion. Session cookies and cookies whose expiration date-time is in the past do not meet the criterion of this filter. The cookie lifetime is calculated from either 'max-age' or 'expires' cookie attributes. If both are specified, 'max-age' is used to calculate the cookie lifetime."]
    pub fn ageUpperBound(this: &FilterResponseCookie) -> Option<::js_sys::Number>;
    # [wasm_bindgen (method , getter , js_class = FilterResponseCookie)]
    #[doc = "Value of the Domain cookie attribute."]
    pub fn domain(this: &FilterResponseCookie) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = FilterResponseCookie)]
    #[doc = "Value of the Expires cookie attribute."]
    pub fn expires(this: &FilterResponseCookie) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = FilterResponseCookie)]
    #[doc = "Existence of the HttpOnly cookie attribute."]
    pub fn httpOnly(this: &FilterResponseCookie) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = FilterResponseCookie)]
    #[doc = "Value of the Max-Age cookie attribute"]
    pub fn maxAge(this: &FilterResponseCookie) -> Option<::js_sys::Number>;
    # [wasm_bindgen (method , getter , js_class = FilterResponseCookie)]
    #[doc = "Name of a cookie."]
    pub fn name(this: &FilterResponseCookie) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = FilterResponseCookie)]
    #[doc = "Value of the Path cookie attribute."]
    pub fn path(this: &FilterResponseCookie) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = FilterResponseCookie)]
    #[doc = "Existence of the Secure cookie attribute."]
    pub fn secure(this: &FilterResponseCookie) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = FilterResponseCookie)]
    #[doc = "Filters session cookies. Session cookies have no lifetime specified in any of 'max-age' or 'expires' attributes."]
    pub fn sessionCookie(this: &FilterResponseCookie) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = FilterResponseCookie)]
    #[doc = "Value of a cookie, may be padded in double-quotes."]
    pub fn value(this: &FilterResponseCookie) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "declarativeWebRequest.declarativeWebRequest.AddRequestCookie" , typescript_type = "declarativeWebRequest.declarativeWebRequest.AddRequestCookie")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Adds a cookie to the request or overrides a cookie, in case another cookie of the same name exists already. Note that it is preferred to use the Cookies API because this is computationally less expensive."]
    pub type AddRequestCookie;
    # [wasm_bindgen (method , getter , js_class = AddRequestCookie)]
    #[doc = "Cookie to be added to the request. No field may be undefined."]
    pub fn cookie(this: &AddRequestCookie) -> crate::declarative_web_request::RequestCookie;
    # [wasm_bindgen (method , getter , js_class = AddRequestCookie)]
    #[doc = ""]
    pub fn instanceType(this: &AddRequestCookie) -> AddRequestCookieInstanceType;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "declarativeWebRequest.declarativeWebRequest.AddResponseCookie" , typescript_type = "declarativeWebRequest.declarativeWebRequest.AddResponseCookie")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Adds a cookie to the response or overrides a cookie, in case another cookie of the same name exists already. Note that it is preferred to use the Cookies API because this is computationally less expensive."]
    pub type AddResponseCookie;
    # [wasm_bindgen (method , getter , js_class = AddResponseCookie)]
    #[doc = "Cookie to be added to the response. The name and value need to be specified."]
    pub fn cookie(this: &AddResponseCookie) -> crate::declarative_web_request::ResponseCookie;
    # [wasm_bindgen (method , getter , js_class = AddResponseCookie)]
    #[doc = ""]
    pub fn instanceType(this: &AddResponseCookie) -> AddResponseCookieInstanceType;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "declarativeWebRequest.declarativeWebRequest.EditRequestCookie" , typescript_type = "declarativeWebRequest.declarativeWebRequest.EditRequestCookie")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Edits one or more cookies of request. Note that it is preferred to use the Cookies API because this is computationally less expensive."]
    pub type EditRequestCookie;
    # [wasm_bindgen (method , getter , js_class = EditRequestCookie)]
    #[doc = "Filter for cookies that will be modified. All empty entries are ignored."]
    pub fn filter(this: &EditRequestCookie) -> crate::declarative_web_request::RequestCookie;
    # [wasm_bindgen (method , getter , js_class = EditRequestCookie)]
    #[doc = ""]
    pub fn instanceType(this: &EditRequestCookie) -> EditRequestCookieInstanceType;
    # [wasm_bindgen (method , getter , js_class = EditRequestCookie)]
    #[doc = "Attributes that shall be overridden in cookies that machted the filter. Attributes that are set to an empty string are removed."]
    pub fn modification(this: &EditRequestCookie) -> crate::declarative_web_request::RequestCookie;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "declarativeWebRequest.declarativeWebRequest.EditResponseCookie" , typescript_type = "declarativeWebRequest.declarativeWebRequest.EditResponseCookie")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Edits one or more cookies of response. Note that it is preferred to use the Cookies API because this is computationally less expensive."]
    pub type EditResponseCookie;
    # [wasm_bindgen (method , getter , js_class = EditResponseCookie)]
    #[doc = "Filter for cookies that will be modified. All empty entries are ignored."]
    pub fn filter(
        this: &EditResponseCookie,
    ) -> crate::declarative_web_request::FilterResponseCookie;
    # [wasm_bindgen (method , getter , js_class = EditResponseCookie)]
    #[doc = ""]
    pub fn instanceType(this: &EditResponseCookie) -> EditResponseCookieInstanceType;
    # [wasm_bindgen (method , getter , js_class = EditResponseCookie)]
    #[doc = "Attributes that shall be overridden in cookies that machted the filter. Attributes that are set to an empty string are removed."]
    pub fn modification(
        this: &EditResponseCookie,
    ) -> crate::declarative_web_request::ResponseCookie;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "declarativeWebRequest.declarativeWebRequest.RemoveRequestCookie" , typescript_type = "declarativeWebRequest.declarativeWebRequest.RemoveRequestCookie")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Removes one or more cookies of request. Note that it is preferred to use the Cookies API because this is computationally less expensive."]
    pub type RemoveRequestCookie;
    # [wasm_bindgen (method , getter , js_class = RemoveRequestCookie)]
    #[doc = "Filter for cookies that will be removed. All empty entries are ignored."]
    pub fn filter(this: &RemoveRequestCookie) -> crate::declarative_web_request::RequestCookie;
    # [wasm_bindgen (method , getter , js_class = RemoveRequestCookie)]
    #[doc = ""]
    pub fn instanceType(this: &RemoveRequestCookie) -> RemoveRequestCookieInstanceType;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "declarativeWebRequest.declarativeWebRequest.RemoveResponseCookie" , typescript_type = "declarativeWebRequest.declarativeWebRequest.RemoveResponseCookie")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Removes one or more cookies of response. Note that it is preferred to use the Cookies API because this is computationally less expensive."]
    pub type RemoveResponseCookie;
    # [wasm_bindgen (method , getter , js_class = RemoveResponseCookie)]
    #[doc = "Filter for cookies that will be removed. All empty entries are ignored."]
    pub fn filter(
        this: &RemoveResponseCookie,
    ) -> crate::declarative_web_request::FilterResponseCookie;
    # [wasm_bindgen (method , getter , js_class = RemoveResponseCookie)]
    #[doc = ""]
    pub fn instanceType(this: &RemoveResponseCookie) -> RemoveResponseCookieInstanceType;
}
