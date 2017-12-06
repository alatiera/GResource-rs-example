extern crate gtk;
extern crate gio;
extern crate glib;

mod static_resource;

use gtk::prelude::*;

fn main() {
    gtk::init().expect("Failed to initialize gtk+");

    static_resource::init().expect("Failed to initialize GResource");

    let builder = gtk::Builder::new_from_resource("/org/gtk/resource_example/window.ui");
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
