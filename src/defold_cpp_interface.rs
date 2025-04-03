use bevy_transform::components::Transform;

include!("bindings.rs");

#[repr(C)]
pub struct GoTransform {
    pub rotation: [f32; 4],
    pub translation: [f32; 3],
    pub scale: [f32; 3],
}

impl From<Transform> for GoTransform {
    fn from(value: Transform) -> Self {
        GoTransform {
            rotation: value.rotation.to_array(),
            translation: value.translation.to_array(),
            scale: value.scale.to_array(),
        }
    }
}

unsafe extern "C" {
    pub(crate) unsafe fn set_go_transform_cpp(url: *const u8, go_transform: GoTransform);
}

unsafe extern "C" {
    pub(crate) unsafe fn post_message_cpp(
        url: *const u8,
        message_name: *const u8,
        message_data: *const u8,
        message_data_len: usize,
    );
}

#[repr(C)]
pub enum CreateViewResultCpp {
    Success {
        url_raw_ptr: *const u8,
        url_len: usize,
    },
    NoViewFactory,
    CallbackCallError,
    CreateViewCallbackInvalid,
    CantParseDataToLua,
    GetNullAfterCreate,
    CallbackSetupError,
    InvalidLuaContext,
}

unsafe extern "C" {
    pub unsafe fn create_view_cpp(
        view_factory_id: dmhash_t,
        transform: GoTransform,
        properties_data: *const u8,
        properties_data_len: usize,
    ) -> CreateViewResultCpp;
}

unsafe extern "C" {
    pub(crate) unsafe fn log_info_cpp(message_name: *const u8);
}

unsafe extern "C" {
    pub(crate) unsafe fn log_error_cpp(message_name: *const u8);
}
