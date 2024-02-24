#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>chrome.bookmarks</code> API to create, organize, and otherwise manipulate bookmarks. Also see <a href='override'>Override Pages</a>, which you can use to create a custom Bookmark Manager page."]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "BookmarkTreeNodeUnmodifiable" , typescript_type = "BookmarkTreeNodeUnmodifiable")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Indicates the reason why this node is unmodifiable. The <var>managed</var> value indicates that this node was configured by the system administrator. Omitted if the node can be modified by the user and the extension (default)."]
    pub type BookmarkTreeNodeUnmodifiable;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "BookmarkTreeNode" , typescript_type = "BookmarkTreeNode")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "A node (either a bookmark or a folder) in the bookmark tree.  Child nodes are ordered within their parent folder."]
    pub type BookmarkTreeNode;
    # [wasm_bindgen (method , getter , js_class = BookmarkTreeNode)]
    #[doc = "An ordered list of children of this node."]
    pub fn children(this: &BookmarkTreeNode) -> Option<::js_sys::Array>;
    # [wasm_bindgen (method , getter , js_class = BookmarkTreeNode)]
    #[doc = "When this node was created, in milliseconds since the epoch (<code>new Date(dateAdded)</code>)."]
    pub fn dateAdded(this: &BookmarkTreeNode) -> Option<::js_sys::Number>;
    # [wasm_bindgen (method , getter , js_class = BookmarkTreeNode)]
    #[doc = "When the contents of this folder last changed, in milliseconds since the epoch."]
    pub fn dateGroupModified(this: &BookmarkTreeNode) -> Option<::js_sys::Number>;
    # [wasm_bindgen (method , getter , js_class = BookmarkTreeNode)]
    #[doc = "When this node was last opened, in milliseconds since the epoch. Not set for folders."]
    pub fn dateLastUsed(this: &BookmarkTreeNode) -> Option<::js_sys::Number>;
    # [wasm_bindgen (method , getter , js_class = BookmarkTreeNode)]
    #[doc = "The unique identifier for the node. IDs are unique within the current profile, and they remain valid even after the browser is restarted."]
    pub fn id(this: &BookmarkTreeNode) -> ::js_sys::JsString;
    # [wasm_bindgen (method , getter , js_class = BookmarkTreeNode)]
    #[doc = "The 0-based position of this node within its parent folder."]
    pub fn index(this: &BookmarkTreeNode) -> Option<::js_sys::Number>;
    # [wasm_bindgen (method , getter , js_class = BookmarkTreeNode)]
    #[doc = "The <code>id</code> of the parent folder.  Omitted for the root node."]
    pub fn parentId(this: &BookmarkTreeNode) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = BookmarkTreeNode)]
    #[doc = "The text displayed for the node."]
    pub fn title(this: &BookmarkTreeNode) -> ::js_sys::JsString;
    # [wasm_bindgen (method , getter , js_class = BookmarkTreeNode)]
    #[doc = "Indicates the reason why this node is unmodifiable. The <var>managed</var> value indicates that this node was configured by the system administrator or by the custodian of a supervised user. Omitted if the node can be modified by the user and the extension (default)."]
    pub fn unmodifiable(this: &BookmarkTreeNode) -> Option<i32>;
    # [wasm_bindgen (method , getter , js_class = BookmarkTreeNode)]
    #[doc = "The URL navigated to when a user clicks the bookmark. Omitted for folders."]
    pub fn url(this: &BookmarkTreeNode) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "CreateDetails" , typescript_type = "CreateDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Object passed to the create() function."]
    pub type CreateDetails;
    # [wasm_bindgen (method , getter , js_class = CreateDetails)]
    #[doc = ""]
    pub fn index(this: &CreateDetails) -> Option<::js_sys::Number>;
    # [wasm_bindgen (method , getter , js_class = CreateDetails)]
    #[doc = "Defaults to the Other Bookmarks folder."]
    pub fn parentId(this: &CreateDetails) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = CreateDetails)]
    #[doc = ""]
    pub fn title(this: &CreateDetails) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = CreateDetails)]
    #[doc = ""]
    pub fn url(this: &CreateDetails) -> Option<::js_sys::JsString>;
}
