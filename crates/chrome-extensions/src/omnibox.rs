#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "The omnibox API allows you to register a keyword with Google Chrome's address bar, which is also known as the omnibox."]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "DescriptionStyleType" , typescript_type = "DescriptionStyleType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The style type."]
    pub type DescriptionStyleType;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "OnInputEnteredDisposition" , typescript_type = "OnInputEnteredDisposition")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The window disposition for the omnibox query. This is the recommended context to display results. For example, if the omnibox command is to navigate to a certain URL, a disposition of 'newForegroundTab' means the navigation should take place in a new selected tab."]
    pub type OnInputEnteredDisposition;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "MatchClassification" , typescript_type = "MatchClassification")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The style ranges for the description, as provided by the extension."]
    pub type MatchClassification;
    # [wasm_bindgen (method , getter , js_class = MatchClassification)]
    #[doc = ""]
    pub fn length(this: &MatchClassification) -> Option<::js_sys::Number>;
    # [wasm_bindgen (method , getter , js_class = MatchClassification)]
    #[doc = ""]
    pub fn offset(this: &MatchClassification) -> ::js_sys::Number;
    # [wasm_bindgen (method , getter , js_class = MatchClassification)]
    #[doc = "The style type"]
    pub fn type_(this: &MatchClassification) -> DescriptionStyleType;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "SuggestResult" , typescript_type = "SuggestResult")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "A suggest result."]
    pub type SuggestResult;
    # [wasm_bindgen (method , getter , js_class = SuggestResult)]
    #[doc = "The text that is put into the URL bar, and that is sent to the extension when the user chooses this entry."]
    pub fn content(this: &SuggestResult) -> ::js_sys::JsString;
    # [wasm_bindgen (method , getter , js_class = SuggestResult)]
    #[doc = "Whether the suggest result can be deleted by the user."]
    pub fn deletable(this: &SuggestResult) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = SuggestResult)]
    #[doc = "The text that is displayed in the URL dropdown. Can contain XML-style markup for styling. The supported tags are 'url' (for a literal URL), 'match' (for highlighting text that matched what the user's query), and 'dim' (for dim helper text). The styles can be nested, eg. <dim><match>dimmed match</match></dim>. You must escape the five predefined entities to display them as text: stackoverflow.com/a/1091953/89484 "]
    pub fn description(this: &SuggestResult) -> ::js_sys::JsString;
    # [wasm_bindgen (method , getter , js_class = SuggestResult)]
    #[doc = "An array of style ranges for the description, as provided by the extension."]
    pub fn descriptionStyles(this: &SuggestResult) -> Option<::js_sys::Array>;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "DefaultSuggestResult" , typescript_type = "DefaultSuggestResult")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "A suggest result."]
    pub type DefaultSuggestResult;
    # [wasm_bindgen (method , getter , js_class = DefaultSuggestResult)]
    #[doc = "The text that is displayed in the URL dropdown. Can contain XML-style markup for styling. The supported tags are 'url' (for a literal URL), 'match' (for highlighting text that matched what the user's query), and 'dim' (for dim helper text). The styles can be nested, eg. <dim><match>dimmed match</match></dim>."]
    pub fn description(this: &DefaultSuggestResult) -> ::js_sys::JsString;
    # [wasm_bindgen (method , getter , js_class = DefaultSuggestResult)]
    #[doc = "An array of style ranges for the description, as provided by the extension."]
    pub fn descriptionStyles(this: &DefaultSuggestResult) -> Option<::js_sys::Array>;
}
