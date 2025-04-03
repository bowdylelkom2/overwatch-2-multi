use gtk::prelude::*;
use gtk::{Button, Label, Window, WindowType};
use std::process::Command;

mod features;

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Overwatch 2 Multi");
    window.set_default_size(400, 200);

    let label = Label::new(Some("Welcome to Overwatch 2 Multi"));
    let button = Button::new_with_label("Launch Overwatch 2");

    button.connect_clicked(|_| {
        Command::new("overwatch-2-multi.exe").spawn().expect("Failed to launch Overwatch 2");
    });

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
    vbox.pack_start(&label, true, true, 0);
    vbox.pack_start(&button, true, true, 0);
    window.add(&vbox);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.show_all();
    gtk::main();
}