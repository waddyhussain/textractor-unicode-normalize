/*
 * https://github.com/kuroahna/textractor_websocket
 *
 * MIT License
 *
 * Copyright (c) 2023 kuroahna
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use std::ffi::{c_char, CStr};

enum PropertyName {
    CurrentSelect,
    TextNumber,
}

impl PropertyName {
    fn as_str(&self) -> &'static str {
        match self {
            PropertyName::CurrentSelect => "current select",
            PropertyName::TextNumber => "text number",
        }
    }
}

#[derive(PartialEq)]
pub enum CurrentSelect {
    NotUserSelectedTextThread,
    UserSelectedTextThread(i64),
}

#[derive(PartialEq)]
pub enum TextNumber {
    Console,
    Clipboard,
    TextThread(i64),
}

#[repr(C)]
pub struct InfoForExtension {
    name: *mut c_char,
    value: i64,
}

pub struct SentenceInfo {
    info_array: *const InfoForExtension,
}

impl SentenceInfo {
    pub fn new(info_array: *const InfoForExtension) -> Self {
        Self { info_array }
    }

    pub fn get_current_select(&self) -> CurrentSelect {
        let value = self
            .get_property_value(PropertyName::CurrentSelect)
            .unwrap_or_else(|| {
                panic!(
                    "The sentence info array should always contain the property `{}`",
                    PropertyName::CurrentSelect.as_str()
                )
            });
        // "current select": always 0 unless the sentence is in the text thread
        // selected by the user
        match value {
            0 => CurrentSelect::NotUserSelectedTextThread,
            value => CurrentSelect::UserSelectedTextThread(value),
        }
    }

    pub fn get_text_number(&self) -> TextNumber {
        let value = self
            .get_property_value(PropertyName::TextNumber)
            .unwrap_or_else(|| {
                panic!(
                    "The sentence info array should always contain the property `{}`",
                    PropertyName::TextNumber.as_str()
                )
            });
        // "text number": number of the current text thread. Counts up one by one
        // as text threads are created. 0 for console, 1 for clipboard
        match value {
            0 => TextNumber::Console,
            1 => TextNumber::Clipboard,
            value => TextNumber::TextThread(value),
        }
    }

    fn get_property_value(&self, property_name: PropertyName) -> Option<i64> {
        let mut pointer = self.info_array;

        while !pointer.is_null() {
            // SAFETY: The pointer dereference is safe because we checked that
            // the pointer is not null, and Textractor should provide a name and
            // a value for each element in the info array. Also, constructing a
            // CStr is safe because Textractor should provide a string with a
            // nul terminator at the end of the string
            let name = unsafe {
                let name = (*pointer).name;
                CStr::from_ptr(name).to_str().unwrap_or_else(|e| {
                    panic!(
                        "The property name `{}` is not a valid UTF-8 string: {:?}",
                        CStr::from_ptr(name).to_string_lossy(),
                        e
                    )
                })
            };
            // SAFETY: The pointer dereference is safe because we checked
            // that the pointer is not null, and Textractor should provide
            // a name and a value for each element in the info array
            let value = unsafe { (*pointer).value };
            if name == property_name.as_str() {
                return Some(value);
            }
            // SAFETY: The pointer addition is safe because we are adding 1 byte
            // past the end of the allocated object
            pointer = unsafe { pointer.add(1) }
        }

        None
    }
}
