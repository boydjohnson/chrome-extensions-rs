#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "The <code>chrome.extensionTypes</code> API contains type declarations for Chrome extensions."]
# [wasm_bindgen (js_namespace = chrome)]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "extensionTypes.ImageFormat" , typescript_type = "extensionTypes.ImageFormat")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The format of an image."]
    pub type ImageFormat;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "extensionTypes.ImageDetails" , typescript_type = "extensionTypes.ImageDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Details about the format and quality of an image."]
    pub type ImageDetails;
    # [wasm_bindgen (method , getter , js_class = ImageDetails)]
    #[doc = "The format of the resulting image.  Default is <code>\"jpeg\"</code>."]
    pub fn format(this: &ImageDetails) -> Option<ImageFormat>;
    # [wasm_bindgen (method , getter , js_class = ImageDetails)]
    #[doc = "When format is <code>\"jpeg\"</code>, controls the quality of the resulting image.  This value is ignored for PNG images.  As quality is decreased, the resulting image will have more visual artifacts, and the number of bytes needed to store it will decrease."]
    pub fn quality(this: &ImageDetails) -> Option<::js_sys::Number>;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "extensionTypes.RunAt" , typescript_type = "extensionTypes.RunAt")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The soonest that the JavaScript or CSS will be injected into the tab."]
    pub type RunAt;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "extensionTypes.CSSOrigin" , typescript_type = "extensionTypes.CSSOrigin")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The <a href=\"https://www.w3.org/TR/css3-cascade/#cascading-origins\">origin</a> of injected CSS."]
    pub type CSSOrigin;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "extensionTypes.InjectDetails" , typescript_type = "extensionTypes.InjectDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Details of the script or CSS to inject. Either the code or the file property must be set, but both may not be set at the same time."]
    pub type InjectDetails;
    # [wasm_bindgen (method , getter , js_class = InjectDetails)]
    #[doc = "If allFrames is <code>true</code>, implies that the JavaScript or CSS should be injected into all frames of current page. By default, it's <code>false</code> and is only injected into the top frame. If <code>true</code> and <code>frameId</code> is set, then the code is inserted in the selected frame and all of its child frames."]
    pub fn allFrames(this: &InjectDetails) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = InjectDetails)]
    #[doc = "JavaScript or CSS code to inject. <br><br><aside class='warning'><b>Warning:</b> Be careful using the <code>code</code> parameter. Incorrect use of it may open your extension to <a href='https://en.wikipedia.org/wiki/Cross-site_scripting'>cross site scripting</a> attacks</aside>"]
    pub fn code(this: &InjectDetails) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = InjectDetails)]
    #[doc = "The <a href=\"https://www.w3.org/TR/css3-cascade/#cascading-origins\">origin</a> of the CSS to inject. This may only be specified for CSS, not JavaScript. Defaults to <code>\"author\"</code>."]
    pub fn cssOrigin(this: &InjectDetails) -> Option<CSSOrigin>;
    # [wasm_bindgen (method , getter , js_class = InjectDetails)]
    #[doc = "JavaScript or CSS file to inject."]
    pub fn file(this: &InjectDetails) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = InjectDetails)]
    #[doc = "The <a href='webNavigation#frame_ids'>frame</a> where the script or CSS should be injected. Defaults to 0 (the top-level frame)."]
    pub fn frameId(this: &InjectDetails) -> Option<::js_sys::Number>;
    # [wasm_bindgen (method , getter , js_class = InjectDetails)]
    #[doc = "If matchAboutBlank is true, then the code is also injected in about:blank and about:srcdoc frames if your extension has access to its parent document. Code cannot be inserted in top-level about:-frames. By default it is <code>false</code>."]
    pub fn matchAboutBlank(this: &InjectDetails) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = InjectDetails)]
    #[doc = "The soonest that the JavaScript or CSS will be injected into the tab. Defaults to \"document_idle\"."]
    pub fn runAt(this: &InjectDetails) -> Option<RunAt>;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "extensionTypes.DeleteInjectionDetails" , typescript_type = "extensionTypes.DeleteInjectionDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Details of the CSS to remove. Either the code or the file property must be set, but both may not be set at the same time."]
    pub type DeleteInjectionDetails;
    # [wasm_bindgen (method , getter , js_class = DeleteInjectionDetails)]
    #[doc = "If allFrames is <code>true</code>, implies that the CSS should be removed from all frames of current page. By default, it's <code>false</code> and is only removed from the top frame. If <code>true</code> and <code>frameId</code> is set, then the code is removed from the selected frame and all of its child frames."]
    pub fn allFrames(this: &DeleteInjectionDetails) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = DeleteInjectionDetails)]
    #[doc = "CSS code to remove."]
    pub fn code(this: &DeleteInjectionDetails) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = DeleteInjectionDetails)]
    #[doc = "The <a href=\"https://www.w3.org/TR/css3-cascade/#cascading-origins\">origin</a> of the CSS to remove. Defaults to <code>\"author\"</code>."]
    pub fn cssOrigin(this: &DeleteInjectionDetails) -> Option<CSSOrigin>;
    # [wasm_bindgen (method , getter , js_class = DeleteInjectionDetails)]
    #[doc = "CSS file to remove."]
    pub fn file(this: &DeleteInjectionDetails) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = DeleteInjectionDetails)]
    #[doc = "The <a href='webNavigation#frame_ids'>frame</a> from where the CSS should be removed. Defaults to 0 (the top-level frame)."]
    pub fn frameId(this: &DeleteInjectionDetails) -> Option<::js_sys::Number>;
    # [wasm_bindgen (method , getter , js_class = DeleteInjectionDetails)]
    #[doc = "If matchAboutBlank is true, then the code is also removed from about:blank and about:srcdoc frames if your extension has access to its parent document. By default it is <code>false</code>."]
    pub fn matchAboutBlank(this: &DeleteInjectionDetails) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "extensionTypes.FrameType" , typescript_type = "extensionTypes.FrameType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The type of frame."]
    pub type FrameType;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "extensionTypes.DocumentLifecycle" , typescript_type = "extensionTypes.DocumentLifecycle")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The document lifecycle of the frame."]
    pub type DocumentLifecycle;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "extensionTypes.ExecutionWorld" , typescript_type = "extensionTypes.ExecutionWorld")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The JavaScript world for a script to execute within. Can either be an isolated world unique to this extension, the main world of the DOM which is shared with the page's JavaScript, or a user scripts world that is only available for scripts registered with the User Scripts API."]
    pub type ExecutionWorld;
}
