/* automatically generated by rust-bindgen 0.71.1 */

pub const _VCRT_COMPILER_PREPROCESSOR: u32 = 1;
pub const _SAL_VERSION: u32 = 20;
pub const __SAL_H_VERSION: u32 = 180000000;
pub const _USE_DECLSPECS_FOR_SAL: u32 = 0;
pub const _USE_ATTRIBUTES_FOR_SAL: u32 = 0;
pub const _CRT_PACKING: u32 = 8;
pub const _HAS_EXCEPTIONS: u32 = 1;
pub const _STL_LANG: u32 = 0;
pub const _HAS_CXX17: u32 = 0;
pub const _HAS_CXX20: u32 = 0;
pub const _HAS_CXX23: u32 = 0;
pub const _HAS_CXX26: u32 = 0;
pub const _HAS_NODISCARD: u32 = 0;
pub const WCHAR_MIN: u32 = 0;
pub const WCHAR_MAX: u32 = 65535;
pub const WINT_MIN: u32 = 0;
pub const WINT_MAX: u32 = 65535;
pub type va_list = *mut cty::c_char;
unsafe extern "C" {
    pub fn __va_start(arg1: *mut *mut cty::c_char, ...);
}
pub type __vcrt_bool = bool;
pub type wchar_t = cty::c_ushort;
unsafe extern "C" {
    pub fn __security_init_cookie();
}
unsafe extern "C" {
    pub fn __security_check_cookie(_StackCookie: usize);
}
unsafe extern "C" {
    pub fn __report_gsfailure(_StackCookie: usize) -> !;
}
unsafe extern "C" {
    pub static mut __security_cookie: usize;
}
pub type int_least8_t = cty::c_schar;
pub type int_least16_t = cty::c_short;
pub type int_least32_t = cty::c_int;
pub type int_least64_t = cty::c_longlong;
pub type uint_least8_t = cty::c_uchar;
pub type uint_least16_t = cty::c_ushort;
pub type uint_least32_t = cty::c_uint;
pub type uint_least64_t = cty::c_ulonglong;
pub type int_fast8_t = cty::c_schar;
pub type int_fast16_t = cty::c_int;
pub type int_fast32_t = cty::c_int;
pub type int_fast64_t = cty::c_longlong;
pub type uint_fast8_t = cty::c_uchar;
pub type uint_fast16_t = cty::c_uint;
pub type uint_fast32_t = cty::c_uint;
pub type uint_fast64_t = cty::c_ulonglong;
pub type intmax_t = cty::c_longlong;
pub type uintmax_t = cty::c_ulonglong;
pub type dmhash_t = u64;
unsafe extern "C" {
    pub fn PostMessage(
        receiver_url: *const cty::c_char,
        message_name: *const cty::c_char,
        message_data_as_json: *const cty::c_char,
        message_data_len: usize,
    );
}
unsafe extern "C" {
    pub fn dmHashReverseSafe64(hash: u64) -> *const cty::c_char;
}
unsafe extern "C" {
    pub fn dmHashString64(string: *const cty::c_char) -> u64;
}
