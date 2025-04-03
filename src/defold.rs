use core::{slice::from_raw_parts, str::FromStr};

use bevy_transform::components::Transform;
use no_std_strings::{zstr, ztr32, ztr64};

use serde::Serialize;

use crate::defold_cpp_interface::{
    create_view_cpp, dmHashReverseSafe64, dmHashString64, dmhash_t, log_error_cpp, log_info_cpp,
    post_message_cpp, set_go_transform_cpp,
};

#[derive(Clone, Copy)]
pub struct URL(ztr64);

impl URL {
    pub fn new(url: ztr64) -> Self {
        Self(url)
    }
}

#[derive(Clone, Copy)]
pub struct MessageName(ztr32);

impl MessageName {
    pub fn new(name: ztr32) -> Self {
        Self(name)
    }
}

pub fn post_message_to_view<T: Serialize>(
    receiver_url: URL,
    message_name: MessageName,
    message_data: T,
) {
    let message_data_as_json: serde_json_core::heapless::String<512> =
        serde_json_core::to_string(&message_data).unwrap_or_else(|_| {
            log_info(ztr64::create("cant serialize message to json"));
            serde_json_core::heapless::String::from_str("{}").unwrap()
        });

    unsafe {
        post_message_cpp(
            receiver_url.0.as_ptr(),
            message_name.0.as_ptr(),
            message_data_as_json.as_ptr(),
            message_data_as_json.len(),
        )
    };
}

pub fn set_go_transform(receiver_url: URL, transform_to_set: Transform) {
    unsafe {
        set_go_transform_cpp(receiver_url.0.as_ptr(), transform_to_set.into());
    }
}

pub enum HashToStringError {
    GetNull,
    Unknown,
}

pub fn hash_to_string(hash: dmhash_t) -> Result<ztr64, HashToStringError> {
    let c_string_ptr = unsafe { dmHashReverseSafe64(hash) };
    if c_string_ptr.is_null() {
        return Err(HashToStringError::GetNull);
    }

    let c_str_bytes = unsafe { core::ffi::CStr::from_ptr(c_string_ptr).to_bytes() };
    let result_string = ztr64::from_raw(c_str_bytes);

    if result_string == ztr64::create("<unknown>") {
        return Err(HashToStringError::Unknown);
    }
    Ok(result_string)
}

pub fn string_to_hash(string_to_convert: &ztr64) -> dmhash_t {
    unsafe { dmHashString64(string_to_convert.as_ptr() as *const cty::c_char) }
}

pub enum CreateViewError {
    NoViewFactory,
    CallbackCallError,
    CreateViewCallbackInvalid,
    CantParseDataToLua,
    GetNullAfterCreate,
    CallbackSetupError,
    InvalidLuaContext,
}

pub fn create_view<T: Serialize>(
    view_factory_id: dmhash_t,
    transform_to_set: Transform,
    create_view_data: T,
) -> Result<URL, CreateViewError> {
    let create_view_data_as_json: serde_json_core::heapless::String<512> =
        serde_json_core::to_string(&create_view_data).unwrap_or_else(|_| {
            log_info(ztr64::create("cant serialize create_view_data to json"));
            serde_json_core::heapless::String::from_str("{}").unwrap()
        });

    let create_result_cpp = unsafe {
        create_view_cpp(
            view_factory_id,
            transform_to_set.into(),
            create_view_data_as_json.as_ptr(),
            create_view_data_as_json.len(),
        )
    };
    match create_result_cpp {
        crate::defold_cpp_interface::CreateViewResultCpp::Success {
            url_raw_ptr,
            url_len,
        } => {
            let url_as_slice = unsafe { from_raw_parts(url_raw_ptr, url_len) };
            Ok(URL::new(ztr64::from_raw(url_as_slice)))
        }
        crate::defold_cpp_interface::CreateViewResultCpp::NoViewFactory => {
            Err(CreateViewError::NoViewFactory)
        }
        crate::defold_cpp_interface::CreateViewResultCpp::GetNullAfterCreate => {
            Err(CreateViewError::GetNullAfterCreate)
        }
        crate::defold_cpp_interface::CreateViewResultCpp::CallbackCallError => {
            Err(CreateViewError::CallbackCallError)
        }
        crate::defold_cpp_interface::CreateViewResultCpp::CreateViewCallbackInvalid => {
            Err(CreateViewError::CreateViewCallbackInvalid)
        }
        crate::defold_cpp_interface::CreateViewResultCpp::CantParseDataToLua => {
            Err(CreateViewError::CantParseDataToLua)
        }
        crate::defold_cpp_interface::CreateViewResultCpp::CallbackSetupError => {
            Err(CreateViewError::CallbackSetupError)
        }
        crate::defold_cpp_interface::CreateViewResultCpp::InvalidLuaContext => {
            Err(CreateViewError::InvalidLuaContext)
        }
    }
}

pub fn log_info<const N: usize>(message: zstr<N>) {
    unsafe {
        log_info_cpp(message.as_ptr());
    }
}

pub fn log_error<const N: usize>(message: zstr<N>) {
    unsafe {
        log_error_cpp(message.as_ptr());
    }
}
