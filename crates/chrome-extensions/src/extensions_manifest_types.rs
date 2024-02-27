#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Schemas for structured manifest entries"]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "automation", typescript_type = "automation")]
    #[derive(Debug, Clone, PartialEq)]
    #[doc = "This API provides programmatic access to the user interface elements of Chrome. This includes everything in the web view, and optionally Chrome's full user interface."]
    pub type automation;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "ContentCapabilities" , typescript_type = "ContentCapabilities")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The <code>content_capabilities</code> manifest entry allows an extension to grant certain additional capabilities to web contents whose locations match a given set of URL patterns."]
    pub type ContentCapabilities;
    # [wasm_bindgen (method , getter , js_class = ContentCapabilities)]
    #[doc = "The set of URL patterns to match against. If any of the given patterns match a URL, its contents will be granted the specified capabilities."]
    pub fn matches(this: &ContentCapabilities) -> ::js_sys::Array;
    # [wasm_bindgen (method , getter , js_class = ContentCapabilities)]
    #[doc = "The set of capabilities to grant matched contents. This is currently limited to <code>clipboardRead</code>, <code>clipboardWrite</code>, and <code>unlimitedStorage</code>."]
    pub fn permissions(this: &ContentCapabilities) -> ::js_sys::Array;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "ExternallyConnectable" , typescript_type = "ExternallyConnectable")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type ExternallyConnectable;
    # [wasm_bindgen (method , getter , js_class = ExternallyConnectable)]
    #[doc = "If <code>true</code>, messages sent via $(ref:runtime.connect) or $(ref:runtime.sendMessage) will set $(ref:runtime.MessageSender.tlsChannelId) if those methods request it to be. If <code>false</code>, $(ref:runtime.MessageSender.tlsChannelId) will never be set under any circumstance."]
    pub fn accepts_tls_channel_id(this: &ExternallyConnectable) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = ExternallyConnectable)]
    #[doc = "<p>The IDs of extensions or apps that are allowed to connect. If left empty or unspecified, no extensions or apps can connect.</p><p>The wildcard <code>\"*\"</code> will allow all extensions and apps to connect.</p>"]
    pub fn ids(this: &ExternallyConnectable) -> Option<::js_sys::Array>;
    # [wasm_bindgen (method , getter , js_class = ExternallyConnectable)]
    #[doc = "<p>The URL patterns for <em>web pages</em> that are allowed to connect. <em>This does not affect content scripts.</em> If left empty or unspecified, no web pages can connect.</p><p>Patterns cannot include wildcard domains nor subdomains of <a href=\"http://publicsuffix.org/list/\">(effective) top level domains</a>; <code>*://google.com/*</code> and <code>http://*.chromium.org/*</code> are valid, while <code>&lt;all_urls&gt;</code>, <code>http://*/*</code>, <code>*://*.com/*</code>, and even <code>http://*.appspot.com/*</code> are not.</p>"]
    pub fn matches(this: &ExternallyConnectable) -> Option<::js_sys::Array>;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "OptionsUI" , typescript_type = "OptionsUI")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The <code>options_ui</code> manifest property declares how the options page should be displayed."]
    pub type OptionsUI;
    # [wasm_bindgen (method , getter , js_class = OptionsUI)]
    #[doc = "If <code>true</code>, a Chrome user agent stylesheet will be applied to your options page. The default value is <code>false</code>. We do not recommend you enable it as it no longer results in a consistent UI with Chrome. This option will be removed in Manifest V3."]
    pub fn chrome_style(this: &OptionsUI) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = OptionsUI)]
    #[doc = "<p>If <code>true</code>, your extension's options page will be opened in a new tab rather than embedded in <em>chrome://extensions</em>. The default is <code>false</code>, and we recommend that you don't change it.</p><p><strong>This is only useful to delay the inevitable deprecation of the old options UI!</strong> It will be removed soon, so try not to use it. It will break.</p>"]
    pub fn open_in_tab(this: &OptionsUI) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = OptionsUI)]
    #[doc = "The path to your options page, relative to your extension's root."]
    pub fn page(this: &OptionsUI) -> ::js_sys::JsString;
    #[wasm_bindgen(js_name = "SocketHostPatterns", typescript_type = "SocketHostPatterns")]
    #[derive(Debug, Clone, PartialEq)]
    #[doc = "<p>A single string or a list of strings representing host:port patterns.</p>"]
    pub type SocketHostPatterns;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "sockets" , typescript_type = "sockets")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The <code>sockets</code> manifest property declares which sockets operations an app can issue."]
    pub type sockets;
    # [wasm_bindgen (method , getter , js_class = sockets)]
    #[doc = "The <code>tcp</code> manifest property declares which sockets.tcp operations an app can issue."]
    pub fn tcp(this: &sockets) -> Option<::js_sys::Object>;
    # [wasm_bindgen (method , getter , js_class = sockets)]
    #[doc = "The <code>tcpServer</code> manifest property declares which sockets.tcpServer operations an app can issue."]
    pub fn tcpServer(this: &sockets) -> Option<::js_sys::Object>;
    # [wasm_bindgen (method , getter , js_class = sockets)]
    #[doc = "The <code>udp</code> manifest property declares which sockets.udp operations an app can issue."]
    pub fn udp(this: &sockets) -> Option<::js_sys::Object>;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "bluetooth" , typescript_type = "bluetooth")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The <code>bluetooth</code> manifest property give permission to an app to use the $(ref:bluetooth) API. A list of UUIDs can be optionally specified to enable communication with devices."]
    pub type bluetooth;
    # [wasm_bindgen (method , getter , js_class = bluetooth)]
    #[doc = "If <code>true</code>, gives permission to an app to use the $(ref:bluetoothLowEnergy) API"]
    pub fn low_energy(this: &bluetooth) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = bluetooth)]
    #[doc = "If <code>true</code>, gives permission to an app to use the advertisement functions in the $(ref:bluetoothLowEnergy) API"]
    pub fn peripheral(this: &bluetooth) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = bluetooth)]
    #[doc = "If <code>true</code>, gives permission to an app to use the $(ref:bluetoothSocket) API"]
    pub fn socket(this: &bluetooth) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = bluetooth)]
    #[doc = "The <code>uuids</code> manifest property declares the list of protocols, profiles and services that an app can communicate using."]
    pub fn uuids(this: &bluetooth) -> Option<::js_sys::Array>;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "UsbPrinters" , typescript_type = "UsbPrinters")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The <code>usb_printers</code> manifest property lists the USB printers supported by an app implementing the $(ref:printerProvider) API."]
    pub type UsbPrinters;
    # [wasm_bindgen (method , getter , js_class = UsbPrinters)]
    #[doc = "A list of $(ref:usb.DeviceFilter USB device filters) matching supported devices. A device only needs to match one of the provided filters. A <code>vendorId</code> is required and only one of <code>productId</code> or <code>interfaceClass</code> may be provided."]
    pub fn filters(this: &UsbPrinters) -> ::js_sys::Array;
    # [wasm_bindgen (extends = :: js_sys :: Array , js_name = "KioskSecondaryApps" , typescript_type = "KioskSecondaryApps")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The <code>kiosk_secondary_apps</code> manifest property lists the secondary kiosk apps to be deployed by the primary kiosk app."]
    pub type KioskSecondaryApps;
}
