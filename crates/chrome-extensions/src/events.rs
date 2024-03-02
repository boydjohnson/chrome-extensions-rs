#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "The <code>chrome.events</code> namespace contains common types used by APIs dispatching events to notify you when something interesting happens."]
# [wasm_bindgen (js_namespace = chrome)]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "events.Rule" , typescript_type = "events.Rule")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Description of a declarative rule for handling events."]
    pub type Rule;
    # [wasm_bindgen (method , getter , js_class = Rule)]
    #[doc = "List of actions that are triggered if one of the conditions is fulfilled."]
    pub fn actions(this: &Rule) -> ::js_sys::Array;
    # [wasm_bindgen (method , getter , js_class = Rule)]
    #[doc = "List of conditions that can trigger the actions."]
    pub fn conditions(this: &Rule) -> ::js_sys::Array;
    # [wasm_bindgen (method , getter , js_class = Rule)]
    #[doc = "Optional identifier that allows referencing this rule."]
    pub fn id(this: &Rule) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = Rule)]
    #[doc = "Optional priority of this rule. Defaults to 100."]
    pub fn priority(this: &Rule) -> Option<::js_sys::Number>;
    # [wasm_bindgen (method , getter , js_class = Rule)]
    #[doc = "Tags can be used to annotate rules and perform operations on sets of rules."]
    pub fn tags(this: &Rule) -> Option<::js_sys::Array>;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "events.Event" , typescript_type = "events.Event")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "An object which allows the addition and removal of listeners for a Chrome event."]
    pub type Event;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "events.UrlFilter" , typescript_type = "events.UrlFilter")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Filters URLs for various criteria. See <a href='events#filtered'>event filtering</a>. All criteria are case sensitive."]
    pub type UrlFilter;
    # [wasm_bindgen (method , getter , js_class = UrlFilter)]
    #[doc = "Matches if the host part of the URL is an IP address and is contained in any of the CIDR blocks specified in the array."]
    pub fn cidrBlocks(this: &UrlFilter) -> Option<::js_sys::Array>;
    # [wasm_bindgen (method , getter , js_class = UrlFilter)]
    #[doc = "Matches if the host name of the URL contains a specified string. To test whether a host name component has a prefix 'foo', use hostContains: '.foo'. This matches 'www.foobar.com' and 'foo.com', because an implicit dot is added at the beginning of the host name. Similarly, hostContains can be used to match against component suffix ('foo.') and to exactly match against components ('.foo.'). Suffix- and exact-matching for the last components need to be done separately using hostSuffix, because no implicit dot is added at the end of the host name."]
    pub fn hostContains(this: &UrlFilter) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = UrlFilter)]
    #[doc = "Matches if the host name of the URL is equal to a specified string."]
    pub fn hostEquals(this: &UrlFilter) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = UrlFilter)]
    #[doc = "Matches if the host name of the URL starts with a specified string."]
    pub fn hostPrefix(this: &UrlFilter) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = UrlFilter)]
    #[doc = "Matches if the host name of the URL ends with a specified string."]
    pub fn hostSuffix(this: &UrlFilter) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = UrlFilter)]
    #[doc = "Matches if the URL without query segment and fragment identifier matches a specified regular expression. Port numbers are stripped from the URL if they match the default port number. The regular expressions use the <a href=\"https://github.com/google/re2/blob/master/doc/syntax.txt\">RE2 syntax</a>."]
    pub fn originAndPathMatches(this: &UrlFilter) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = UrlFilter)]
    #[doc = "Matches if the path segment of the URL contains a specified string."]
    pub fn pathContains(this: &UrlFilter) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = UrlFilter)]
    #[doc = "Matches if the path segment of the URL is equal to a specified string."]
    pub fn pathEquals(this: &UrlFilter) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = UrlFilter)]
    #[doc = "Matches if the path segment of the URL starts with a specified string."]
    pub fn pathPrefix(this: &UrlFilter) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = UrlFilter)]
    #[doc = "Matches if the path segment of the URL ends with a specified string."]
    pub fn pathSuffix(this: &UrlFilter) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = UrlFilter)]
    #[doc = "Matches if the port of the URL is contained in any of the specified port lists. For example <code>[80, 443, [1000, 1200]]</code> matches all requests on port 80, 443 and in the range 1000-1200."]
    pub fn ports(this: &UrlFilter) -> Option<::js_sys::Array>;
    # [wasm_bindgen (method , getter , js_class = UrlFilter)]
    #[doc = "Matches if the query segment of the URL contains a specified string."]
    pub fn queryContains(this: &UrlFilter) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = UrlFilter)]
    #[doc = "Matches if the query segment of the URL is equal to a specified string."]
    pub fn queryEquals(this: &UrlFilter) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = UrlFilter)]
    #[doc = "Matches if the query segment of the URL starts with a specified string."]
    pub fn queryPrefix(this: &UrlFilter) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = UrlFilter)]
    #[doc = "Matches if the query segment of the URL ends with a specified string."]
    pub fn querySuffix(this: &UrlFilter) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = UrlFilter)]
    #[doc = "Matches if the scheme of the URL is equal to any of the schemes specified in the array."]
    pub fn schemes(this: &UrlFilter) -> Option<::js_sys::Array>;
    # [wasm_bindgen (method , getter , js_class = UrlFilter)]
    #[doc = "Matches if the URL (without fragment identifier) contains a specified string. Port numbers are stripped from the URL if they match the default port number."]
    pub fn urlContains(this: &UrlFilter) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = UrlFilter)]
    #[doc = "Matches if the URL (without fragment identifier) is equal to a specified string. Port numbers are stripped from the URL if they match the default port number."]
    pub fn urlEquals(this: &UrlFilter) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = UrlFilter)]
    #[doc = "Matches if the URL (without fragment identifier) matches a specified regular expression. Port numbers are stripped from the URL if they match the default port number. The regular expressions use the <a href=\"https://github.com/google/re2/blob/master/doc/syntax.txt\">RE2 syntax</a>."]
    pub fn urlMatches(this: &UrlFilter) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = UrlFilter)]
    #[doc = "Matches if the URL (without fragment identifier) starts with a specified string. Port numbers are stripped from the URL if they match the default port number."]
    pub fn urlPrefix(this: &UrlFilter) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = UrlFilter)]
    #[doc = "Matches if the URL (without fragment identifier) ends with a specified string. Port numbers are stripped from the URL if they match the default port number."]
    pub fn urlSuffix(this: &UrlFilter) -> Option<::js_sys::JsString>;
}
