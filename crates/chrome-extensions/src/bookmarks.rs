#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>chrome.bookmarks</code> API to create, organize, and otherwise manipulate bookmarks. Also see <a href='override'>Override Pages</a>, which you can use to create a custom Bookmark Manager page."]
# [wasm_bindgen (js_namespace = chrome)]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "bookmarks.BookmarkTreeNodeUnmodifiable" , typescript_type = "bookmarks.BookmarkTreeNodeUnmodifiable")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Indicates the reason why this node is unmodifiable. The <var>managed</var> value indicates that this node was configured by the system administrator. Omitted if the node can be modified by the user and the extension (default)."]
    pub type BookmarkTreeNodeUnmodifiable;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "bookmarks.BookmarkTreeNode" , typescript_type = "bookmarks.BookmarkTreeNode")]
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
    pub fn unmodifiable(this: &BookmarkTreeNode) -> Option<BookmarkTreeNodeUnmodifiable>;
    # [wasm_bindgen (method , getter , js_class = BookmarkTreeNode)]
    #[doc = "The URL navigated to when a user clicks the bookmark. Omitted for folders."]
    pub fn url(this: &BookmarkTreeNode) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "bookmarks.CreateDetails" , typescript_type = "bookmarks.CreateDetails")]
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
    #[doc = "Retrieves the specified BookmarkTreeNode(s)."]
    #[wasm_bindgen(js_name = "bookmarks.get", catch)]
    pub async fn get(
        idOrIdList: ::wasm_bindgen::JsValue,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Retrieves the children of the specified BookmarkTreeNode id."]
    #[wasm_bindgen(js_name = "bookmarks.getChildren", catch)]
    pub async fn getChildren(
        id: ::js_sys::JsString,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Retrieves the recently added bookmarks."]
    #[wasm_bindgen(js_name = "bookmarks.getRecent", catch)]
    pub async fn getRecent(
        numberOfItems: ::js_sys::Number,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Retrieves the entire Bookmarks hierarchy."]
    #[wasm_bindgen(js_name = "bookmarks.getTree", catch)]
    pub async fn getTree() -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Retrieves part of the Bookmarks hierarchy, starting at the specified node."]
    #[wasm_bindgen(js_name = "bookmarks.getSubTree", catch)]
    pub async fn getSubTree(
        id: ::js_sys::JsString,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Searches for BookmarkTreeNodes matching the given query. Queries specified with an object produce BookmarkTreeNodes matching all specified properties."]
    #[wasm_bindgen(js_name = "bookmarks.search", catch)]
    pub async fn search(
        query: ::wasm_bindgen::JsValue,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Creates a bookmark or folder under the specified parentId.  If url is NULL or missing, it will be a folder."]
    #[wasm_bindgen(js_name = "bookmarks.create", catch)]
    pub async fn create(
        bookmark: CreateDetails,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Moves the specified BookmarkTreeNode to the provided location."]
    #[wasm_bindgen(js_name = "bookmarks.move", catch)]
    pub async fn move_(
        id: ::js_sys::JsString,
        destination: ::js_sys::Object,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Updates the properties of a bookmark or folder. Specify only the properties that you want to change; unspecified properties will be left unchanged.  <b>Note:</b> Currently, only 'title' and 'url' are supported."]
    #[wasm_bindgen(js_name = "bookmarks.update", catch)]
    pub async fn update(
        id: ::js_sys::JsString,
        changes: ::js_sys::Object,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Removes a bookmark or an empty bookmark folder."]
    #[wasm_bindgen(js_name = "bookmarks.remove", catch)]
    pub async fn remove(id: ::js_sys::JsString) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Recursively removes a bookmark folder."]
    #[wasm_bindgen(js_name = "bookmarks.removeTree", catch)]
    pub async fn removeTree(id: ::js_sys::JsString) -> Result<(), ::wasm_bindgen::JsValue>;
}
#[wasm_bindgen]
pub async fn bookmarks_get(
    idOrIdList: ::wasm_bindgen::JsValue,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    get(idOrIdList).await
}
#[wasm_bindgen]
pub async fn bookmarks_get_children(
    id: ::js_sys::JsString,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    getChildren(id).await
}
#[wasm_bindgen]
pub async fn bookmarks_get_recent(
    numberOfItems: ::js_sys::Number,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    getRecent(numberOfItems).await
}
#[wasm_bindgen]
pub async fn bookmarks_get_tree() -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    getTree().await
}
#[wasm_bindgen]
pub async fn bookmarks_get_sub_tree(
    id: ::js_sys::JsString,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    getSubTree(id).await
}
#[wasm_bindgen]
pub async fn bookmarks_search(
    query: ::wasm_bindgen::JsValue,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    search(query).await
}
#[wasm_bindgen]
pub async fn bookmarks_create(
    bookmark: CreateDetails,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    create(bookmark).await
}
#[wasm_bindgen]
pub async fn bookmarks_move(
    id: ::js_sys::JsString,
    destination: ::js_sys::Object,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    move_(id, destination).await
}
#[wasm_bindgen]
pub async fn bookmarks_update(
    id: ::js_sys::JsString,
    changes: ::js_sys::Object,
) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
    update(id, changes).await
}
#[wasm_bindgen]
pub async fn bookmarks_remove(id: ::js_sys::JsString) -> Result<(), ::wasm_bindgen::JsValue> {
    remove(id).await
}
#[wasm_bindgen]
pub async fn bookmarks_remove_tree(id: ::js_sys::JsString) -> Result<(), ::wasm_bindgen::JsValue> {
    removeTree(id).await
}
