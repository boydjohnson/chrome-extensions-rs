#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>chrome.tts</code> API to play synthesized text-to-speech (TTS). See also the related <a href='http://developer.chrome.com/extensions/ttsEngine'>ttsEngine</a> API, which allows an extension to implement a speech engine."]
# [wasm_bindgen (js_namespace = chrome)]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "tts.EventType" , typescript_type = "tts.EventType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type EventType;
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "tts.VoiceGender" , typescript_type = "tts.VoiceGender")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type VoiceGender;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "tts.TtsOptions" , typescript_type = "tts.TtsOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The speech options for the TTS engine."]
    pub type TtsOptions;
    # [wasm_bindgen (method , getter , js_class = TtsOptions)]
    #[doc = "The TTS event types that you are interested in listening to. If missing, all event types may be sent."]
    pub fn desiredEventTypes(this: &TtsOptions) -> Option<::js_sys::Array>;
    # [wasm_bindgen (method , getter , js_class = TtsOptions)]
    #[doc = "If true, enqueues this utterance if TTS is already in progress. If false (the default), interrupts any current speech and flushes the speech queue before speaking this new utterance."]
    pub fn enqueue(this: &TtsOptions) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = TtsOptions)]
    #[doc = "The extension ID of the speech engine to use, if known."]
    pub fn extensionId(this: &TtsOptions) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = TtsOptions)]
    #[doc = "Gender of voice for synthesized speech."]
    pub fn gender(this: &TtsOptions) -> Option<VoiceGender>;
    # [wasm_bindgen (method , getter , js_class = TtsOptions)]
    #[doc = "The language to be used for synthesis, in the form <em>language</em>-<em>region</em>. Examples: 'en', 'en-US', 'en-GB', 'zh-CN'."]
    pub fn lang(this: &TtsOptions) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = TtsOptions)]
    #[doc = "Speaking pitch between 0 and 2 inclusive, with 0 being lowest and 2 being highest. 1.0 corresponds to a voice's default pitch."]
    pub fn pitch(this: &TtsOptions) -> Option<::js_sys::Number>;
    # [wasm_bindgen (method , getter , js_class = TtsOptions)]
    #[doc = "Speaking rate relative to the default rate for this voice. 1.0 is the default rate, normally around 180 to 220 words per minute. 2.0 is twice as fast, and 0.5 is half as fast. Values below 0.1 or above 10.0 are strictly disallowed, but many voices will constrain the minimum and maximum rates further&mdash;for example a particular voice may not actually speak faster than 3 times normal even if you specify a value larger than 3.0."]
    pub fn rate(this: &TtsOptions) -> Option<::js_sys::Number>;
    # [wasm_bindgen (method , getter , js_class = TtsOptions)]
    #[doc = "The TTS event types the voice must support."]
    pub fn requiredEventTypes(this: &TtsOptions) -> Option<::js_sys::Array>;
    # [wasm_bindgen (method , getter , js_class = TtsOptions)]
    #[doc = "The name of the voice to use for synthesis. If empty, uses any available voice."]
    pub fn voiceName(this: &TtsOptions) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = TtsOptions)]
    #[doc = "Speaking volume between 0 and 1 inclusive, with 0 being lowest and 1 being highest, with a default of 1.0."]
    pub fn volume(this: &TtsOptions) -> Option<::js_sys::Number>;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "tts.TtsEvent" , typescript_type = "tts.TtsEvent")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "An event from the TTS engine to communicate the status of an utterance."]
    pub type TtsEvent;
    # [wasm_bindgen (method , getter , js_class = TtsEvent)]
    #[doc = "The index of the current character in the utterance. For word events, the event fires at the end of one word and before the beginning of the next. The <code>charIndex</code> represents a point in the text at the beginning of the next word to be spoken."]
    pub fn charIndex(this: &TtsEvent) -> Option<::js_sys::Number>;
    # [wasm_bindgen (method , getter , js_class = TtsEvent)]
    #[doc = "The error description, if the event type is <code>error</code>."]
    pub fn errorMessage(this: &TtsEvent) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = TtsEvent)]
    #[doc = "True if this is the final event that will be sent to this handler."]
    pub fn isFinalEvent(this: &TtsEvent) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = TtsEvent)]
    #[doc = "The length of the next part of the utterance. For example, in a <code>word</code> event, this is the length of the word which will be spoken next. It will be set to -1 if not set by the speech engine."]
    pub fn length(this: &TtsEvent) -> Option<::js_sys::Number>;
    # [wasm_bindgen (method , getter , js_class = TtsEvent)]
    #[doc = "An ID unique to the calling function's context so that events can get routed back to the correct tts.speak call."]
    pub fn srcId(this: &TtsEvent) -> Option<::js_sys::Number>;
    # [wasm_bindgen (method , getter , js_class = TtsEvent)]
    #[doc = "The type can be <code>start</code> as soon as speech has started, <code>word</code> when a word boundary is reached, <code>sentence</code> when a sentence boundary is reached, <code>marker</code> when an SSML mark element is reached, <code>end</code> when the end of the utterance is reached, <code>interrupted</code> when the utterance is stopped or interrupted before reaching the end, <code>cancelled</code> when it's removed from the queue before ever being synthesized, or <code>error</code> when any other error occurs. When pausing speech, a <code>pause</code> event is fired if a particular utterance is paused in the middle, and <code>resume</code> if an utterance resumes speech. Note that pause and resume events may not fire if speech is paused in-between utterances."]
    pub fn type_(this: &TtsEvent) -> EventType;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "tts.TtsVoice" , typescript_type = "tts.TtsVoice")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "A description of a voice available for speech synthesis."]
    pub type TtsVoice;
    # [wasm_bindgen (method , getter , js_class = TtsVoice)]
    #[doc = "All of the callback event types that this voice is capable of sending."]
    pub fn eventTypes(this: &TtsVoice) -> Option<::js_sys::Array>;
    # [wasm_bindgen (method , getter , js_class = TtsVoice)]
    #[doc = "The ID of the extension providing this voice."]
    pub fn extensionId(this: &TtsVoice) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = TtsVoice)]
    #[doc = "This voice's gender."]
    pub fn gender(this: &TtsVoice) -> Option<VoiceGender>;
    # [wasm_bindgen (method , getter , js_class = TtsVoice)]
    #[doc = "The language that this voice supports, in the form <em>language</em>-<em>region</em>. Examples: 'en', 'en-US', 'en-GB', 'zh-CN'."]
    pub fn lang(this: &TtsVoice) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = TtsVoice)]
    #[doc = "If true, the synthesis engine is a remote network resource. It may be higher latency and may incur bandwidth costs."]
    pub fn remote(this: &TtsVoice) -> Option<::js_sys::Boolean>;
    # [wasm_bindgen (method , getter , js_class = TtsVoice)]
    #[doc = "The name of the voice."]
    pub fn voiceName(this: &TtsVoice) -> Option<::js_sys::JsString>;
    #[doc = "Speaks text using a text-to-speech engine."]
    #[wasm_bindgen(js_name = "tts.speak", catch)]
    pub async fn speak(
        utterance: ::js_sys::JsString,
        options: Option<TtsOptions>,
    ) -> Result<(), ::wasm_bindgen::JsValue>;
    #[doc = "Checks whether the engine is currently speaking. On Mac OS X, the result is true whenever the system speech engine is speaking, even if the speech wasn't initiated by Chrome."]
    #[wasm_bindgen(js_name = "tts.isSpeaking", catch)]
    pub async fn isSpeaking() -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
    #[doc = "Gets an array of all available voices."]
    #[wasm_bindgen(js_name = "tts.getVoices", catch)]
    pub async fn getVoices() -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
}
