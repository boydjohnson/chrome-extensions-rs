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
    pub async fn setWallpaper(
        details: ::js_sys::Object,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
}
#[wasm_bindgen]
pub async fn wallpaper_set_wallpaper(
    details: ::js_sys::Object,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    setWallpaper(details).await
}
