extern crate gtk;

use gtk::prelude::*;

fn main() {
    gtk::init().expect("Failed to initialize gtk+");

    let builder = gtk::Builder::new_from_string(include_str!("../resources/window.ui"));
    let window: gtk::Window = builder.get_object("window").unwrap();
    window.show_all();

    window.connect_delete_event(|_, _| {
        // Stop the main loop.
        gtk::main_quit();
        // Let the default handler destroy the window.
        Inhibit(false)
    });

    gtk::main();
}
