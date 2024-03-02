#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the commands API to add keyboard shortcuts that trigger actions in your extension, for example, an action to open the browser action or send a command to the extension."]
# [wasm_bindgen (js_namespace = chrome)]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "commands.Command" , typescript_type = "commands.Command")]
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
    #[doc = "Returns all the registered extension commands for this extension and their shortcut (if active). Before Chrome 110, this command did not return <code>_execute_action</code>."]
    #[wasm_bindgen(js_name = "commands.getAll", catch)]
    pub async fn getAll() -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
}
#[wasm_bindgen]
pub async fn commands_get_all() -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    getAll().await
}
