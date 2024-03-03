#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>chrome.wallpaper</code> API to change the ChromeOS wallpaper."]
# [wasm_bindgen (js_namespace = chrome)]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "wallpaper.WallpaperLayout" , typescript_type = "wallpaper.WallpaperLayout")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The supported wallpaper layouts."]
    pub type WallpaperLayout;
    #[doc = "Sets wallpaper to the image at <em>url</em> or <em>wallpaperData</em> with the specified <em>layout</em>"]
    #[wasm_bindgen(js_name = "wallpaper.setWallpaper", catch)]
    pub async fn set_wallpaper(
        details: ::js_sys::Object,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Sets wallpaper to the image at <em>url</em> or <em>wallpaperData</em> with the specified <em>layout</em>"]
    #[wasm_bindgen(js_name = "wallpaper.setWallpaper")]
    pub fn set_wallpaper_callback(details: ::js_sys::Object, callback: &::js_sys::Function);
}
#[wasm_bindgen]
pub async fn wallpaper_set_wallpaper(
    details: ::js_sys::Object,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    set_wallpaper(details).await
}
#[wasm_bindgen]
pub fn wallpaper_set_wallpaper_callback(details: ::js_sys::Object, callback: &::js_sys::Function) {
    set_wallpaper_callback(details, callback);
}
