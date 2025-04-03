use alloc::boxed::Box;
use core::mem;

use bevy_app::App;

use crate::bevy_app_config::get_app;

#[unsafe(no_mangle)]
pub extern "C" fn create_and_init_world() -> *mut App {
    let mut app_boxed = Box::new(get_app());
    let app_ptr = app_boxed.as_mut() as *mut App;
    mem::forget(app_boxed); // prevent the Box from being dropped
    app_ptr
}

#[unsafe(no_mangle)]
pub extern "C" fn update_app(app: *mut App) {
    let app = unsafe { &mut *app };
    app.update();
}

#[unsafe(no_mangle)]
pub extern "C" fn destroy_app(app: *mut App) {
    let _ = unsafe { Box::from_raw(app) };
}
