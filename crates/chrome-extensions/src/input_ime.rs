#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>chrome.input.ime</code> API to implement a custom IME for Chrome OS. This allows your extension to handle keystrokes, set the composition, and manage the candidate window."]
# [wasm_bindgen (js_namespace = chrome)]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "input.ime.KeyboardEventType" , typescript_type = "input.ime.KeyboardEventType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type KeyboardEventType;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "input.ime.KeyboardEvent" , typescript_type = "input.ime.KeyboardEvent")]
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
    pub fn type_(this: &KeyboardEvent) -> KeyboardEventType;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "input.ime.InputContextType" , typescript_type = "input.ime.InputContextType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Type of value this text field edits, (Text, Number, URL, etc)"]
    pub type InputContextType;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "input.ime.AutoCapitalizeType" , typescript_type = "input.ime.AutoCapitalizeType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The auto-capitalize type of the text field."]
    pub type AutoCapitalizeType;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "input.ime.InputContext" , typescript_type = "input.ime.InputContext")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Describes an input Context"]
    pub type InputContext;
    # [wasm_bindgen (method , getter , js_class = InputContext)]
    #[doc = "The auto-capitalize type of the text field."]
    pub fn autoCapitalize(this: &InputContext) -> AutoCapitalizeType;
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
    pub fn type_(this: &InputContext) -> InputContextType;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "input.ime.MenuItemStyle" , typescript_type = "input.ime.MenuItemStyle")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The type of menu item. Radio buttons between separators are considered grouped."]
    pub type MenuItemStyle;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "input.ime.MenuItem" , typescript_type = "input.ime.MenuItem")]
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
    pub fn style(this: &MenuItem) -> Option<MenuItemStyle>;
    # [wasm_bindgen (method , getter , js_class = MenuItem)]
    #[doc = "Indicates this item is visible."]
    pub fn visible(this: &MenuItem) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "input.ime.UnderlineStyle" , typescript_type = "input.ime.UnderlineStyle")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The type of the underline to modify this segment."]
    pub type UnderlineStyle;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "input.ime.WindowPosition" , typescript_type = "input.ime.WindowPosition")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Where to display the candidate window. If set to 'cursor', the window follows the cursor. If set to 'composition', the window is locked to the beginning of the composition."]
    pub type WindowPosition;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "input.ime.ScreenType" , typescript_type = "input.ime.ScreenType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The screen type under which the IME is activated."]
    pub type ScreenType;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "input.ime.MouseButton" , typescript_type = "input.ime.MouseButton")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Which mouse buttons was clicked."]
    pub type MouseButton;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "input.ime.AssistiveWindowType" , typescript_type = "input.ime.AssistiveWindowType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Type of assistive window."]
    pub type AssistiveWindowType;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "input.ime.AssistiveWindowProperties" , typescript_type = "input.ime.AssistiveWindowProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Properties of the assistive window."]
    pub type AssistiveWindowProperties;
    # [wasm_bindgen (method , getter , js_class = AssistiveWindowProperties)]
    #[doc = "Strings for ChromeVox to announce."]
    pub fn announceString(this: &AssistiveWindowProperties) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = AssistiveWindowProperties)]
    #[doc = ""]
    pub fn type_(this: &AssistiveWindowProperties) -> AssistiveWindowType;
    # [wasm_bindgen (method , getter , js_class = AssistiveWindowProperties)]
    #[doc = "Sets true to show AssistiveWindow, sets false to hide."]
    pub fn visible(this: &AssistiveWindowProperties) -> ::js_sys::Boolean;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "input.ime.AssistiveWindowButton" , typescript_type = "input.ime.AssistiveWindowButton")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "ID of buttons in assistive window."]
    pub type AssistiveWindowButton;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "input.ime.MenuParameters" , typescript_type = "input.ime.MenuParameters")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type MenuParameters;
    # [wasm_bindgen (method , getter , js_class = MenuParameters)]
    #[doc = "ID of the engine to use."]
    pub fn engineID(this: &MenuParameters) -> ::js_sys::JsString;
    # [wasm_bindgen (method , getter , js_class = MenuParameters)]
    #[doc = "MenuItems to add or update. They will be added in the order they exist in the array."]
    pub fn items(this: &MenuParameters) -> ::js_sys::Array;
    #[doc = "Set the current composition. If this extension does not own the active IME, this fails."]
    #[wasm_bindgen(js_name = "input.ime.setComposition", catch)]
    pub async fn setComposition(
        parameters: ::js_sys::Object,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Clear the current composition. If this extension does not own the active IME, this fails."]
    #[wasm_bindgen(js_name = "input.ime.clearComposition", catch)]
    pub async fn clearComposition(
        parameters: ::js_sys::Object,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Commits the provided text to the current input."]
    #[wasm_bindgen(js_name = "input.ime.commitText", catch)]
    pub async fn commitText(
        parameters: ::js_sys::Object,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Sends the key events.  This function is expected to be used by virtual keyboards.  When key(s) on a virtual keyboard is pressed by a user, this function is used to propagate that event to the system."]
    #[wasm_bindgen(js_name = "input.ime.sendKeyEvents", catch)]
    pub async fn sendKeyEvents(parameters: ::js_sys::Object)
        -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Sets the properties of the candidate window. This fails if the extension doesn't own the active IME"]
    #[wasm_bindgen(js_name = "input.ime.setCandidateWindowProperties", catch)]
    pub async fn setCandidateWindowProperties(
        parameters: ::js_sys::Object,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Sets the current candidate list. This fails if this extension doesn't own the active IME"]
    #[wasm_bindgen(js_name = "input.ime.setCandidates", catch)]
    pub async fn setCandidates(
        parameters: ::js_sys::Object,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Set the position of the cursor in the candidate window. This is a no-op if this extension does not own the active IME."]
    #[wasm_bindgen(js_name = "input.ime.setCursorPosition", catch)]
    pub async fn setCursorPosition(
        parameters: ::js_sys::Object,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Shows/Hides an assistive window with the given properties."]
    #[wasm_bindgen(js_name = "input.ime.setAssistiveWindowProperties", catch)]
    pub async fn setAssistiveWindowProperties(
        parameters: ::js_sys::Object,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Highlights/Unhighlights a button in an assistive window."]
    #[wasm_bindgen(js_name = "input.ime.setAssistiveWindowButtonHighlighted", catch)]
    pub async fn setAssistiveWindowButtonHighlighted(
        parameters: ::js_sys::Object,
    ) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Adds the provided menu items to the language menu when this IME is active."]
    #[wasm_bindgen(js_name = "input.ime.setMenuItems", catch)]
    pub async fn setMenuItems(parameters: MenuParameters) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Updates the state of the MenuItems specified"]
    #[wasm_bindgen(js_name = "input.ime.updateMenuItems", catch)]
    pub async fn updateMenuItems(parameters: MenuParameters)
        -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Deletes the text around the caret."]
    #[wasm_bindgen(js_name = "input.ime.deleteSurroundingText", catch)]
    pub async fn deleteSurroundingText(
        parameters: ::js_sys::Object,
    ) -> Result<(), ::wasm_bindgen::JsValue>;
}
#[wasm_bindgen]
pub async fn input_ime_set_composition(
    parameters: ::js_sys::Object,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    setComposition(parameters).await
}
#[wasm_bindgen]
pub async fn input_ime_clear_composition(
    parameters: ::js_sys::Object,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    clearComposition(parameters).await
}
#[wasm_bindgen]
pub async fn input_ime_commit_text(
    parameters: ::js_sys::Object,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    commitText(parameters).await
}
#[wasm_bindgen]
pub async fn input_ime_send_key_events(
    parameters: ::js_sys::Object,
) -> Result<(), ::wasm_bindgen::JsValue> {
    sendKeyEvents(parameters).await
}
#[wasm_bindgen]
pub async fn input_ime_set_candidate_window_properties(
    parameters: ::js_sys::Object,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    setCandidateWindowProperties(parameters).await
}
#[wasm_bindgen]
pub async fn input_ime_set_candidates(
    parameters: ::js_sys::Object,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    setCandidates(parameters).await
}
#[wasm_bindgen]
pub async fn input_ime_set_cursor_position(
    parameters: ::js_sys::Object,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    setCursorPosition(parameters).await
}
#[wasm_bindgen]
pub async fn input_ime_set_assistive_window_properties(
    parameters: ::js_sys::Object,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    setAssistiveWindowProperties(parameters).await
}
#[wasm_bindgen]
pub async fn input_ime_set_assistive_window_button_highlighted(
    parameters: ::js_sys::Object,
) -> Result<(), ::wasm_bindgen::JsValue> {
    setAssistiveWindowButtonHighlighted(parameters).await
}
#[wasm_bindgen]
pub async fn input_ime_set_menu_items(
    parameters: MenuParameters,
) -> Result<(), ::wasm_bindgen::JsValue> {
    setMenuItems(parameters).await
}
#[wasm_bindgen]
pub async fn input_ime_update_menu_items(
    parameters: MenuParameters,
) -> Result<(), ::wasm_bindgen::JsValue> {
    updateMenuItems(parameters).await
}
#[wasm_bindgen]
pub async fn input_ime_delete_surrounding_text(
    parameters: ::js_sys::Object,
) -> Result<(), ::wasm_bindgen::JsValue> {
    deleteSurroundingText(parameters).await
}
