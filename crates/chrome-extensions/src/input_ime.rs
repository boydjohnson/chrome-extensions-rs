#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>chrome.input.ime</code> API to implement a custom IME for Chrome OS. This allows your extension to handle keystrokes, set the composition, and manage the candidate window."]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "KeyboardEventType" , typescript_type = "KeyboardEventType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type KeyboardEventType;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "KeyboardEvent" , typescript_type = "KeyboardEvent")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "See http://www.w3.org/TR/DOM-Level-3-Events/#events-KeyboardEvent"]
    pub type KeyboardEvent;
    # [wasm_bindgen (method , getter , js_class = KeyboardEvent)]
    #[doc = "Whether or not the ALT key is pressed."]
    pub fn altKey(this: &KeyboardEvent) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = KeyboardEvent)]
    #[doc = "Whether or not the ALTGR key is pressed."]
    pub fn altgrKey(this: &KeyboardEvent) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = KeyboardEvent)]
    #[doc = "Whether or not the CAPS_LOCK is enabled."]
    pub fn capsLock(this: &KeyboardEvent) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = KeyboardEvent)]
    #[doc = "Value of the physical key being pressed. The value is not affected by current keyboard layout or modifier state."]
    pub fn code(this: &KeyboardEvent) -> ::js_sys::JsString;
    # [wasm_bindgen (method , getter , js_class = KeyboardEvent)]
    #[doc = "Whether or not the CTRL key is pressed."]
    pub fn ctrlKey(this: &KeyboardEvent) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = KeyboardEvent)]
    #[doc = "The extension ID of the sender of this keyevent."]
    pub fn extensionId(this: &KeyboardEvent) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = KeyboardEvent)]
    #[doc = "Value of the key being pressed"]
    pub fn key(this: &KeyboardEvent) -> ::js_sys::JsString;
    # [wasm_bindgen (method , getter , js_class = KeyboardEvent)]
    #[doc = "The deprecated HTML keyCode, which is system- and implementation-dependent numerical code signifying the unmodified identifier associated with the key pressed."]
    pub fn keyCode(this: &KeyboardEvent) -> Option<::js_sys::Number>;
    # [wasm_bindgen (method , getter , js_class = KeyboardEvent)]
    #[doc = "(Deprecated) The ID of the request. Use the <code>requestId</code> param from the <code>onKeyEvent</code> event instead."]
    pub fn requestId(this: &KeyboardEvent) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = KeyboardEvent)]
    #[doc = "Whether or not the SHIFT key is pressed."]
    pub fn shiftKey(this: &KeyboardEvent) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = KeyboardEvent)]
    #[doc = "One of keyup or keydown."]
    pub fn type_(this: &KeyboardEvent) -> i32;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "InputContextType" , typescript_type = "InputContextType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Type of value this text field edits, (Text, Number, URL, etc)"]
    pub type InputContextType;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "AutoCapitalizeType" , typescript_type = "AutoCapitalizeType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The auto-capitalize type of the text field."]
    pub type AutoCapitalizeType;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "InputContext" , typescript_type = "InputContext")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Describes an input Context"]
    pub type InputContext;
    # [wasm_bindgen (method , getter , js_class = InputContext)]
    #[doc = "The auto-capitalize type of the text field."]
    pub fn autoCapitalize(this: &InputContext) -> i32;
    # [wasm_bindgen (method , getter , js_class = InputContext)]
    #[doc = "Whether the text field wants auto-complete."]
    pub fn autoComplete(this: &InputContext) -> ::js_sys::Boolean;
    # [wasm_bindgen (method , getter , js_class = InputContext)]
    #[doc = "Whether the text field wants auto-correct."]
    pub fn autoCorrect(this: &InputContext) -> ::js_sys::Boolean;
    # [wasm_bindgen (method , getter , js_class = InputContext)]
    #[doc = "This is used to specify targets of text field operations.  This ID becomes invalid as soon as onBlur is called."]
    pub fn contextID(this: &InputContext) -> ::js_sys::Number;
    # [wasm_bindgen (method , getter , js_class = InputContext)]
    #[doc = "Whether text entered into the text field should be used to improve typing suggestions for the user."]
    pub fn shouldDoLearning(this: &InputContext) -> ::js_sys::Boolean;
    # [wasm_bindgen (method , getter , js_class = InputContext)]
    #[doc = "Whether the text field wants spell-check."]
    pub fn spellCheck(this: &InputContext) -> ::js_sys::Boolean;
    # [wasm_bindgen (method , getter , js_class = InputContext)]
    #[doc = "Type of value this text field edits, (Text, Number, URL, etc)"]
    pub fn type_(this: &InputContext) -> i32;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "MenuItemStyle" , typescript_type = "MenuItemStyle")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The type of menu item. Radio buttons between separators are considered grouped."]
    pub type MenuItemStyle;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "MenuItem" , typescript_type = "MenuItem")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "A menu item used by an input method to interact with the user from the language menu."]
    pub type MenuItem;
    # [wasm_bindgen (method , getter , js_class = MenuItem)]
    #[doc = "Indicates this item should be drawn with a check."]
    pub fn checked(this: &MenuItem) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = MenuItem)]
    #[doc = "Indicates this item is enabled."]
    pub fn enabled(this: &MenuItem) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = MenuItem)]
    #[doc = "String that will be passed to callbacks referencing this MenuItem."]
    pub fn id(this: &MenuItem) -> ::js_sys::JsString;
    # [wasm_bindgen (method , getter , js_class = MenuItem)]
    #[doc = "Text displayed in the menu for this item."]
    pub fn label(this: &MenuItem) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = MenuItem)]
    #[doc = "The type of menu item."]
    pub fn style(this: &MenuItem) -> Option<i32>;
    # [wasm_bindgen (method , getter , js_class = MenuItem)]
    #[doc = "Indicates this item is visible."]
    pub fn visible(this: &MenuItem) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "UnderlineStyle" , typescript_type = "UnderlineStyle")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The type of the underline to modify this segment."]
    pub type UnderlineStyle;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "WindowPosition" , typescript_type = "WindowPosition")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Where to display the candidate window. If set to 'cursor', the window follows the cursor. If set to 'composition', the window is locked to the beginning of the composition."]
    pub type WindowPosition;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "ScreenType" , typescript_type = "ScreenType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The screen type under which the IME is activated."]
    pub type ScreenType;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "MouseButton" , typescript_type = "MouseButton")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Which mouse buttons was clicked."]
    pub type MouseButton;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "AssistiveWindowType" , typescript_type = "AssistiveWindowType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Type of assistive window."]
    pub type AssistiveWindowType;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "AssistiveWindowProperties" , typescript_type = "AssistiveWindowProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Properties of the assistive window."]
    pub type AssistiveWindowProperties;
    # [wasm_bindgen (method , getter , js_class = AssistiveWindowProperties)]
    #[doc = "Strings for ChromeVox to announce."]
    pub fn announceString(this: &AssistiveWindowProperties) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = AssistiveWindowProperties)]
    #[doc = ""]
    pub fn type_(this: &AssistiveWindowProperties) -> i32;
    # [wasm_bindgen (method , getter , js_class = AssistiveWindowProperties)]
    #[doc = "Sets true to show AssistiveWindow, sets false to hide."]
    pub fn visible(this: &AssistiveWindowProperties) -> ::js_sys::Boolean;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "AssistiveWindowButton" , typescript_type = "AssistiveWindowButton")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "ID of buttons in assistive window."]
    pub type AssistiveWindowButton;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "MenuParameters" , typescript_type = "MenuParameters")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type MenuParameters;
    # [wasm_bindgen (method , getter , js_class = MenuParameters)]
    #[doc = "ID of the engine to use."]
    pub fn engineID(this: &MenuParameters) -> ::js_sys::JsString;
    # [wasm_bindgen (method , getter , js_class = MenuParameters)]
    #[doc = "MenuItems to add or update. They will be added in the order they exist in the array."]
    pub fn items(this: &MenuParameters) -> ::js_sys::Array;
}
