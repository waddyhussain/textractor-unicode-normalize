extern crate unicode_normalization;

use unicode_normalization::UnicodeNormalization;
use widestring::{U16String, U16CString};
use std::ptr;
use std::ffi::c_void;
use windows_sys::Win32::System::Memory::{HeapReAlloc, GetProcessHeap, HEAP_GENERATE_EXCEPTIONS};

use crate::textractor::{TextNumber, SentenceInfo, InfoForExtension};

mod textractor;

#[unsafe(no_mangle)]
pub extern "C" fn ProcessSentence(sentence: &mut U16String, sentence_info: &SentenceInfo) -> bool {
    let text_number = sentence_info.get_text_number();
    if text_number == TextNumber::Console {
        return false
    }

    let normalized = sentence.to_string_lossy().nfkc().collect::<String>();
    *sentence = U16String::from_str(&normalized);

    true
}

#[unsafe(no_mangle)]
pub extern "C" fn OnNewSentence(
    sentence: *mut u16,
    sentence_info: *const InfoForExtension,
) -> *const u16 {
    let mut sentence_copy = unsafe { U16CString::from_ptr_str(sentence) }.into_ustring();
    let old_len = sentence_copy.len();

    let sentence_info = SentenceInfo::new(sentence_info);

    if ProcessSentence(&mut sentence_copy, &sentence_info) {
        if sentence_copy.len() > old_len {
            // Need to realloc buffer via Windows HeapReAlloc
            let new_size = (sentence_copy.len() + 1) * std::mem::size_of::<u16>();

            let new_ptr = unsafe {
                HeapReAlloc(
                    GetProcessHeap(),
                    HEAP_GENERATE_EXCEPTIONS,
                    sentence as *mut c_void,
                    new_size,
                ) as *mut u16
            };

            if new_ptr.is_null() {
                return sentence;
            }

            // Copy updated string into new buffer (including null terminator)
            unsafe {
                ptr::copy_nonoverlapping(sentence_copy.as_ptr(), new_ptr, sentence_copy.len());
                *new_ptr.add(sentence_copy.len()) = 0;
            }
            return new_ptr;
        } else {
            // The existing buffer is large enough, just copy contents and terminate string
            unsafe {
                ptr::copy_nonoverlapping(sentence_copy.as_ptr(), sentence, sentence_copy.len());
                *sentence.add(sentence_copy.len()) = 0;
            }
        }
    }

    sentence
}
