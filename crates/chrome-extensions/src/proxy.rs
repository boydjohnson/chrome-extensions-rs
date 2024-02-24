#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>chrome.proxy</code> API to manage Chrome's proxy settings. This API relies on the <a href='reference/api/types#type-ChromeSetting'>ChromeSetting prototype of the type API</a> for getting and setting the proxy configuration."]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "Scheme" , typescript_type = "Scheme")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type Scheme;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "Mode" , typescript_type = "Mode")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type Mode;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "ProxyServer" , typescript_type = "ProxyServer")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "An object encapsulating a single proxy server's specification."]
    pub type ProxyServer;
    # [wasm_bindgen (method , getter , js_class = ProxyServer)]
    #[doc = "The hostname or IP address of the proxy server. Hostnames must be in ASCII (in Punycode format). IDNA is not supported, yet."]
    pub fn host(this: &ProxyServer) -> ::js_sys::JsString;
    # [wasm_bindgen (method , getter , js_class = ProxyServer)]
    #[doc = "The port of the proxy server. Defaults to a port that depends on the scheme."]
    pub fn port(this: &ProxyServer) -> Option<::js_sys::Number>;
    # [wasm_bindgen (method , getter , js_class = ProxyServer)]
    #[doc = "The scheme (protocol) of the proxy server itself. Defaults to 'http'."]
    pub fn scheme(this: &ProxyServer) -> Option<i32>;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "ProxyRules" , typescript_type = "ProxyRules")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "An object encapsulating the set of proxy rules for all protocols. Use either 'singleProxy' or (a subset of) 'proxyForHttp', 'proxyForHttps', 'proxyForFtp' and 'fallbackProxy'."]
    pub type ProxyRules;
    # [wasm_bindgen (method , getter , js_class = ProxyRules)]
    #[doc = "List of servers to connect to without a proxy server."]
    pub fn bypassList(this: &ProxyRules) -> Option<::js_sys::Array>;
    # [wasm_bindgen (method , getter , js_class = ProxyRules)]
    #[doc = "The proxy server to be used for everthing else or if any of the specific proxyFor... is not specified."]
    pub fn fallbackProxy(this: &ProxyRules) -> Option<i32>;
    # [wasm_bindgen (method , getter , js_class = ProxyRules)]
    #[doc = "The proxy server to be used for FTP requests."]
    pub fn proxyForFtp(this: &ProxyRules) -> Option<i32>;
    # [wasm_bindgen (method , getter , js_class = ProxyRules)]
    #[doc = "The proxy server to be used for HTTP requests."]
    pub fn proxyForHttp(this: &ProxyRules) -> Option<i32>;
    # [wasm_bindgen (method , getter , js_class = ProxyRules)]
    #[doc = "The proxy server to be used for HTTPS requests."]
    pub fn proxyForHttps(this: &ProxyRules) -> Option<i32>;
    # [wasm_bindgen (method , getter , js_class = ProxyRules)]
    #[doc = "The proxy server to be used for all per-URL requests (that is http, https, and ftp)."]
    pub fn singleProxy(this: &ProxyRules) -> Option<i32>;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "PacScript" , typescript_type = "PacScript")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "An object holding proxy auto-config information. Exactly one of the fields should be non-empty."]
    pub type PacScript;
    # [wasm_bindgen (method , getter , js_class = PacScript)]
    #[doc = "A PAC script."]
    pub fn data(this: &PacScript) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = PacScript)]
    #[doc = "If true, an invalid PAC script will prevent the network stack from falling back to direct connections. Defaults to false."]
    pub fn mandatory(this: &PacScript) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = PacScript)]
    #[doc = "URL of the PAC file to be used."]
    pub fn url(this: &PacScript) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "ProxyConfig" , typescript_type = "ProxyConfig")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "An object encapsulating a complete proxy configuration."]
    pub type ProxyConfig;
    # [wasm_bindgen (method , getter , js_class = ProxyConfig)]
    #[doc = "'direct' = Never use a proxy<br>'auto_detect' = Auto detect proxy settings<br>'pac_script' = Use specified PAC script<br>'fixed_servers' = Manually specify proxy servers<br>'system' = Use system proxy settings"]
    pub fn mode(this: &ProxyConfig) -> i32;
    # [wasm_bindgen (method , getter , js_class = ProxyConfig)]
    #[doc = "The proxy auto-config (PAC) script for this configuration. Use this for 'pac_script' mode."]
    pub fn pacScript(this: &ProxyConfig) -> Option<i32>;
    # [wasm_bindgen (method , getter , js_class = ProxyConfig)]
    #[doc = "The proxy rules describing this configuration. Use this for 'fixed_servers' mode."]
    pub fn rules(this: &ProxyConfig) -> Option<i32>;
}
