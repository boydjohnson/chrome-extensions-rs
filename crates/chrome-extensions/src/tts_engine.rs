#![allow(unused_imports)]
#![allow(clippy::all)]
use {super::*, wasm_bindgen::prelude::*};
#[doc = "Use the <code>chrome.ttsEngine</code> API to implement a text-to-speech(TTS) engine using an extension. If your extension registers using this API, it will receive events containing an utterance to be spoken and other parameters when any extension or Chrome App uses the <a href='tts'>tts</a> API to generate speech. Your extension can then use any available web technology to synthesize and output the speech, and send events back to the calling function to report the status."]
# [wasm_bindgen (js_namespace = chrome)]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: JsString , js_name = "ttsEngine.VoiceGender" , typescript_type = "ttsEngine.VoiceGender")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = ""]
    pub type VoiceGender;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "ttsEngine.SpeakOptions" , typescript_type = "ttsEngine.SpeakOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Options specified to the tts.speak() method."]
    pub type SpeakOptions;
    # [wasm_bindgen (method , getter , js_class = SpeakOptions)]
    #[doc = "Gender of voice for synthesized speech."]
    pub fn gender(this: &SpeakOptions) -> Option<VoiceGender>;
    # [wasm_bindgen (method , getter , js_class = SpeakOptions)]
    #[doc = "The language to be used for synthesis, in the form <em>language</em>-<em>region</em>. Examples: 'en', 'en-US', 'en-GB', 'zh-CN'."]
    pub fn lang(this: &SpeakOptions) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = SpeakOptions)]
    #[doc = "Speaking pitch between 0 and 2 inclusive, with 0 being lowest and 2 being highest. 1.0 corresponds to this voice's default pitch."]
    pub fn pitch(this: &SpeakOptions) -> Option<::js_sys::Number>;
    # [wasm_bindgen (method , getter , js_class = SpeakOptions)]
    #[doc = "Speaking rate relative to the default rate for this voice. 1.0 is the default rate, normally around 180 to 220 words per minute. 2.0 is twice as fast, and 0.5 is half as fast. This value is guaranteed to be between 0.1 and 10.0, inclusive. When a voice does not support this full range of rates, don't return an error. Instead, clip the rate to the range the voice supports."]
    pub fn rate(this: &SpeakOptions) -> Option<::js_sys::Number>;
    # [wasm_bindgen (method , getter , js_class = SpeakOptions)]
    #[doc = "The name of the voice to use for synthesis."]
    pub fn voiceName(this: &SpeakOptions) -> Option<::js_sys::JsString>;
    # [wasm_bindgen (method , getter , js_class = SpeakOptions)]
    #[doc = "Speaking volume between 0 and 1 inclusive, with 0 being lowest and 1 being highest, with a default of 1.0."]
    pub fn volume(this: &SpeakOptions) -> Option<::js_sys::Number>;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "ttsEngine.AudioStreamOptions" , typescript_type = "ttsEngine.AudioStreamOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Contains the audio stream format expected to be produced by an engine."]
    pub type AudioStreamOptions;
    # [wasm_bindgen (method , getter , js_class = AudioStreamOptions)]
    #[doc = "The number of samples within an audio buffer."]
    pub fn bufferSize(this: &AudioStreamOptions) -> ::js_sys::Number;
    # [wasm_bindgen (method , getter , js_class = AudioStreamOptions)]
    #[doc = "The sample rate expected in an audio buffer."]
    pub fn sampleRate(this: &AudioStreamOptions) -> ::js_sys::Number;
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = "ttsEngine.AudioBuffer" , typescript_type = "ttsEngine.AudioBuffer")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "Parameters containing an audio buffer and associated data."]
    pub type AudioBuffer;
    # [wasm_bindgen (method , getter , js_class = AudioBuffer)]
    #[doc = "The audio buffer from the text-to-speech engine. It should have length exactly audioStreamOptions.bufferSize and encoded as mono, at audioStreamOptions.sampleRate, and as linear pcm, 32-bit signed float i.e. the Float32Array type in javascript."]
    pub fn audioBuffer(this: &AudioBuffer) -> ::web_sys::Blob;
    # [wasm_bindgen (method , getter , js_class = AudioBuffer)]
    #[doc = "The character index associated with this audio buffer."]
    pub fn charIndex(this: &AudioBuffer) -> Option<::js_sys::Number>;
    # [wasm_bindgen (method , getter , js_class = AudioBuffer)]
    #[doc = "True if this audio buffer is the last for the text being spoken."]
    pub fn isLastBuffer(this: &AudioBuffer) -> Option<::js_sys::Boolean>;
}
