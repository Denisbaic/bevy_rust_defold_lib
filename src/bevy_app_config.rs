use bevy_app::{App, Update};
use bevy_time::TimePlugin;
use no_std_strings::ztr64;

use crate::defold;

fn test_log() {
    defold::log_info(ztr64::create("update triggered"));
}

pub(crate) fn get_app() -> App {
    let mut app = App::new();
    app.add_plugins(TimePlugin).add_systems(Update, test_log);
    app
}
