#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "none"]
# [wasm_bindgen (js_namespace = chrome)]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "guestViewInternal.Size" , typescript_type = "guestViewInternal.Size")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type Size;
    # [wasm_bindgen (method , getter , js_class = Size)]
    #[doc = ""]
    pub fn height(this: &Size) -> ::js_sys::Number;
    # [wasm_bindgen (method , getter , js_class = Size)]
    #[doc = ""]
    pub fn width(this: &Size) -> ::js_sys::Number;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "guestViewInternal.SizeParams" , typescript_type = "guestViewInternal.SizeParams")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Size parameters."]
    pub type SizeParams;
    # [wasm_bindgen (method , getter , js_class = SizeParams)]
    #[doc = ""]
    pub fn enableAutoSize(this: &SizeParams) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = SizeParams)]
    #[doc = ""]
    pub fn max(this: &SizeParams) -> Option<Size>;
    # [wasm_bindgen (method , getter , js_class = SizeParams)]
    #[doc = ""]
    pub fn min(this: &SizeParams) -> Option<Size>;
    # [wasm_bindgen (method , getter , js_class = SizeParams)]
    #[doc = ""]
    pub fn normal(this: &SizeParams) -> Option<Size>;
    #[doc = ""]
    #[wasm_bindgen(js_name = "guestViewInternal.createGuest", catch)]
    pub async fn createGuest(
        viewType: ::js_sys::JsString,
        ownerFrameToken: ::js_sys::JsString,
        createParams: ::js_sys::Object,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = ""]
    #[wasm_bindgen(js_name = "guestViewInternal.setSize", catch)]
    pub async fn setSize(
        instanceId: ::js_sys::Number,
        params: SizeParams,
    ) -> Result<(), ::wasm_bindgen::JsValue>;
}
#[wasm_bindgen]
pub async fn guest_view_internal_create_guest(
    viewType: ::js_sys::JsString,
    ownerFrameToken: ::js_sys::JsString,
    createParams: ::js_sys::Object,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    createGuest(viewType, ownerFrameToken, createParams).await
}
#[wasm_bindgen]
pub async fn guest_view_internal_set_size(
    instanceId: ::js_sys::Number,
    params: SizeParams,
) -> Result<(), ::wasm_bindgen::JsValue> {
    setSize(instanceId, params).await
}
