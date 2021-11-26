extern crate libc;
extern crate tokenizers as tk;

use std::borrow::Borrow;
use std::ffi::CString;
use std::ffi::CStr;
use std::u32;
use std::os::raw::c_char;
use std::ptr::null_mut;

use tk::tokenizer::EncodeInput;
use tk::tokenizer::InputSequence;
use tk::{Encoding, Tokenizer};

use libc::{boolean_t, size_t};
use std::ops::Deref;
use tk::FromPretrainedParameters;
use std::mem::ManuallyDrop;
use std::slice;

//JInputSequence
//JEncoding

//from Vec<String>
//from String
//- When ``is_pretokenized=False``: :data:`~TextInputSequence` (InputSequence) union types
//struct TextInputSequence<'s>(tk::InputSequence<'s>);
type Result<T> = std::result::Result<T, JError>;
pub struct JError;

//remove the J as its private and the pub
pub struct JInputSequence<'s> {
    pub input_sequence: tk::InputSequence<'s>,
}
//todo: make from pair
impl JInputSequence<'_> {
    pub fn from_str(st: String) -> JInputSequence<'static> {
        let inputSequence = InputSequence::from(st);
        return JInputSequence {
            input_sequence: inputSequence,
        }
    }

    pub fn from_vec_str(vec: Vec<String>) -> JInputSequence<'static> {
        let inputSequence = InputSequence::from(vec);
        return  JInputSequence {
            input_sequence: inputSequence,
        }
    }
}

pub struct JPairInputSequence<'s> {
    pub first: tk::InputSequence<'s>,
    pub second: tk::InputSequence<'s>,
}

pub struct JEncoding {
    pub encoding: Option<tk::tokenizer::Encoding>
}

impl JEncoding {

    //get length

    pub fn get_length(&self) -> usize {
        let e = &self.encoding.as_ref().expect("Unitialized encoding");
        return e.get_ids().to_vec().len();
    }

    pub fn get_ids(&self) -> Vec<u32> {
       let e = &self.encoding.as_ref().expect("Unitialized encoding");
        return e.get_ids().to_vec();
    }

    pub fn get_tokens(&self) -> Vec<String> {
        let e = &self.encoding.as_ref().expect("Unitialized encoding");
        return e.get_tokens().to_vec();
    }

}

pub struct JTokenizer {
    tokenizer: Option<Tokenizer>,
}

impl JTokenizer {
    //FromPretrainedParameters: two Option of Strings
    pub fn from_pretrained(identifier: &str) -> JTokenizer {
        let parameters = FromPretrainedParameters::default();
        let tokenizer = Tokenizer::from_pretrained(identifier, Some(parameters));
        match tokenizer {
            Ok(value) => {
                return JTokenizer {
                    tokenizer: Some(value),
                };
            }
            Err(error) => {
                println!("Problem instantiating tokenizer {:?}", error);
                return JTokenizer { tokenizer: None };
            }
        }
    }

    pub fn encode(&self, input: &JInputSequence) -> JEncoding {
        let singles = EncodeInput::Single(input.input_sequence.clone());
        match &self.tokenizer {
            Some(value) => {
                let encodings = value.encode(singles, true).ok();
                return JEncoding{ encoding: encodings};
            }
            None => {
                println!("cannot encode");
                return JEncoding{ encoding: None};
            }
        }
    }

    pub fn encode_pair(&self, pair: &JPairInputSequence) -> JEncoding {
        let first = pair.first.clone();
        let second = pair.second.clone();
        let pair = EncodeInput::Dual(first, second);
        match &self.tokenizer {
            Some(value) => {
                let encodings = value.encode(pair, true).ok();
                return JEncoding{ encoding: encodings};
            }
            None => {
                println!("cannot encode pair");
                return JEncoding{ encoding: None } ;
            }
        }
    }

    pub fn print_tokenizer(&self) {
        match &self.tokenizer {
            Some(value) => {
                let string = value.to_string(true);
                println!("I was called in rust. tokenizer: {:?}", value);
            }
            None => {
                println!("no tokenizer found");
            }
        }
    }
}

// pub fn tokenize(&self) -> CEncodings {
//     let input = EncodeInput::Single(InputSequence::Raw(Cow::from("Hellow")));
//     let encodings = self.tokenizer.encode(input, false).unwrap();
//     let ids = encodings.get_ids();
//     CEncodings{ ids: ids.to_vec().clone()}
//     //println!("doing a thing! also, number is {}!", self.number);
// }

//maybe inject handle pointer instead (PointerByReference in jna)
// and return error code if allocation fails
//assert pointer not null

// #[no_mangle]
// pub unsafe extern "C" fn JInputSequence_from_str(str: *const c_char) -> *mut JInputSequence<'static> {
//     let cstr = unsafe { CStr::from_ptr(str).to_string_lossy().to_string() };
//     let inputSequence = Box::new(JInputSequence::from_str(cstr));
//     Box::into_raw(inputSequence)
// }

// #[no_mangle]
// pub unsafe extern "C" fn JInputSequence_from_vec_str(vec: **const c_char, len: usize) -> *mut JInputSequence {
//     let slice = unsafe { Vec::from_raw_parts(ptr, len, len) };
//     let mut v = vec![];
//
//     for elem in slice {
//         let s = CStr::from_ptr(elem).to_string_lossy().to_string();
//         v.push(s)
//     }
//     let inputSequence = Box::new(JInputSequence::from_vec_str(v));
//     Box::into_raw(inputSequence)
// }

// #[no_mangle]
// pub unsafe extern "C" fn JInputSequence_drop(p: *mut JInputSequence) {
//     Box::from_raw(p);
// }
//

//TODO: assert not null in all the pointers
#[no_mangle]
pub unsafe extern "C" fn JTokenizer_from_pretrained(identifier: *const c_char) -> *mut JTokenizer {
    let cstr = unsafe { CStr::from_ptr(identifier).to_string_lossy().to_string() };
    let boxed_a = Box::new(JTokenizer::from_pretrained(&cstr));
    Box::into_raw(boxed_a)
}

#[no_mangle]
pub unsafe extern "C" fn JTokenizer_drop(p: *mut JTokenizer) {
    Box::from_raw(p);
}

#[no_mangle]
pub unsafe extern "C" fn JTokenizer_encode_from_str(tokenizer: *mut JTokenizer, input: *const c_char) -> *mut JEncoding  {
    let instance = &*tokenizer;
    let cstr = unsafe { CStr::from_ptr(input).to_string_lossy().to_string() };
    let inputSequence = JInputSequence::from_str(cstr);
    let encodings =  Box::new(instance.encode(&inputSequence));
    return Box::into_raw(encodings);
}

#[no_mangle]
pub unsafe extern "C" fn JTokenizer_encode_from_vec_str(tokenizer: *mut JTokenizer, input: *mut *const c_char, len: size_t) -> *mut JEncoding  {
    let instance = &*tokenizer;
    let slice = unsafe { slice::from_raw_parts(input, len) };
    let mut v = vec![];
    for elem in slice {
        let s = CStr::from_ptr(*elem).to_string_lossy().to_string();
        v.push(s)
    }
    let inputs = JInputSequence::from_vec_str(v);
    let encodings =  Box::new(instance.encode(&inputs));
    return Box::into_raw(encodings);
}

#[no_mangle]
pub unsafe extern "C" fn JTokenizer_print_tokenizer(a: *mut JTokenizer) {
    let a = &*a;
    a.print_tokenizer();
}

#[no_mangle]
pub unsafe extern "C" fn JEncoding_drop(p: *mut JEncoding) {
    Box::from_raw(p);
}

#[no_mangle]
pub unsafe extern "C" fn JEncoding_get_length(a: *mut JEncoding) -> size_t {
    let encodings = &*a;
    return  encodings.get_length();
}

#[no_mangle]
pub unsafe extern "C" fn JEncoding_get_ids(a: *mut JEncoding, buffer: *mut i64, sizeBuffer: size_t)   {
    let encodings = &*a;
    let len =  encodings.get_length();
    let vector: Vec<i64>  = encodings.get_ids().into_iter().map(i64::from).rev().collect();
    println!("I was called in rust. tokenizer: {:?} {:?}", sizeBuffer, len);
    println!("I was called in rust. ids: {:?} ", vector);
    assert_eq!(sizeBuffer, len);
    buffer.copy_from(vector.as_ptr(), sizeBuffer);
}

#[no_mangle]
pub unsafe extern "C" fn JEncoding_get_type_ids(encoding_ptr: *mut JEncoding, buffer: *mut i32, buffer_size: size_t) {
    // preconditions
    let encoding = (&*encoding_ptr).encoding.as_ref().expect("null encoding");
    let type_ids = encoding.get_type_ids();
    let length = type_ids.len();
    assert_eq!(length, buffer_size);
    let ffi_type_ids: Vec<i32> = type_ids.into_iter().map(|i| *i as i32).rev().collect();
    println!("I was called in rust. type_ids: {:?} ", type_ids);
    buffer.copy_from(ffi_type_ids.as_ptr(), length);
}

// #[no_mangle]
// pub unsafe extern "C" fn JEncoding_get_word_ids(encoding_ptr: *mut JEncoding, buffer: *mut i32, buffer_size: size_t) {
//     // preconditions
//     let encoding = (&*encoding_ptr).encoding.as_ref().expect("null encoding");
//     let word_ids = encoding.get_word_ids();
//     let encoding_length = encoding.get_length();
//     assert_eq!(buffer_size, encoding_length);
//     let ffi_word_ids = word_ids.iter().map(|w| i32::from(w.unwrap_or(-1))).collect();
//     buffer.copy_from(ffi_word_ids.as_ptr, encoding_length);
// }

// TODO get_tokens