#[macro_use]
extern crate log;
#[macro_use]
extern crate glib;

use gettextrs::*;

#[macro_use]
mod utils;

mod application;
#[rustfmt::skip]
mod config;
#[rustfmt::skip]
mod static_resources;
mod window;
mod window_state;

use application::Application;
use config::{GETTEXT_PACKAGE, LOCALEDIR};

fn main() {
    // Prepare i18n
    setlocale(LocaleCategory::LcAll, "");
    bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR);
    textdomain(GETTEXT_PACKAGE);

    glib::set_application_name(&format!("myeditor{}", config::NAME_SUFFIX));
    glib::set_prgname(Some("myeditor"));

    gtk::init().expect("Unable to start GTK3");

    static_resources::init().expect("Failed to initialize the resource file.");

    let app = Application::new();
    app.run();
}
