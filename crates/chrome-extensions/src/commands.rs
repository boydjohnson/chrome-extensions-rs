#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the commands API to add keyboard shortcuts that trigger actions in your extension, for example, an action to open the browser action or send a command to the extension."]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "Command" , typescript_type = "Command")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type Command;
    # [wasm_bindgen (method , getter , js_class = Command)]
    #[doc = "The Extension Command description"]
    pub fn description(this: &Command) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = Command)]
    #[doc = "The name of the Extension Command"]
    pub fn name(this: &Command) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = Command)]
    #[doc = "The shortcut active for this command, or blank if not active."]
    pub fn shortcut(this: &Command) -> Option<::js_sys::JsString>;
}
