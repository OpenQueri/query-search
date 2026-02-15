use std::ffi::{CStr, CString};
use std::os::raw::c_char;

extern "C" {
    pub fn detect_language(text: *const c_char, length: i32) -> *const c_char;
    pub fn detect_language_detailed(
        text: *const c_char,
        length: i32,
        lang_code: *mut c_char,
        confidence: *mut f32,
    ) -> i32;
}

pub fn detect_lang(text: &str) -> String {
    let c_text = CString::new(text).expect("Invalid text");
    let result = unsafe {
        let ptr = detect_language(c_text.as_ptr(), text.len() as i32);
        CStr::from_ptr(ptr).to_string_lossy().into_owned()
    };
    result
}

pub fn detect_lang_with_confidence(text: &str) -> (String, f32) {
    let c_text = CString::new(text).expect("Invalid text");
    let mut lang_buf = [0u8; 16];
    let mut confidence: f32 = 0.0;
    
    unsafe {
        detect_language_detailed(
            c_text.as_ptr(),
            text.len() as i32,
            lang_buf.as_mut_ptr() as *mut c_char,
            &mut confidence as *mut f32,
        );
        
        let lang = CStr::from_ptr(lang_buf.as_ptr() as *const c_char)
            .to_string_lossy()
            .into_owned();
        (lang, confidence)
    }
}

pub fn is_ukrainian(text: &str) -> bool {
    detect_lang(text) == "uk"
}

pub fn is_russian(text: &str) -> bool {
    detect_lang(text) == "ru"
}

pub fn is_english(text: &str) -> bool {
    detect_lang(text) == "en"
}