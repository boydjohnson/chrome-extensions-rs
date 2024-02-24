#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>chrome.wallpaper</code> API to change the ChromeOS wallpaper."]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "WallpaperLayout" , typescript_type = "WallpaperLayout")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The supported wallpaper layouts."]
    pub type WallpaperLayout;
}
