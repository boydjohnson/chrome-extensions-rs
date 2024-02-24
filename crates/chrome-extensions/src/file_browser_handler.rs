#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>chrome.fileBrowserHandler</code> API to extend the Chrome OS file browser. For example, you can use this API to enable users to upload files to your website."]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "FileHandlerExecuteEventDetails" , typescript_type = "FileHandlerExecuteEventDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Event details payload for fileBrowserHandler.onExecute event."]
    pub type FileHandlerExecuteEventDetails;
    # [wasm_bindgen (method , getter , js_class = FileHandlerExecuteEventDetails)]
    #[doc = "Array of Entry instances representing files that are targets of this action (selected in ChromeOS file browser)."]
    pub fn entries(this: &FileHandlerExecuteEventDetails) -> ::js_sys::Array;
    # [wasm_bindgen (method , getter , js_class = FileHandlerExecuteEventDetails)]
    #[doc = "The ID of the tab that raised this event. Tab IDs are unique within a browser session."]
    pub fn tab_id(this: &FileHandlerExecuteEventDetails) -> Option<::js_sys::Number>;
}
