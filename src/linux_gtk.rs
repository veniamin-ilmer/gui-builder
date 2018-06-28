use gtk;

use gtk::prelude::*;

use gtk::{Button, Entry, Window, WindowType, Fixed};

pub fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_title("test");
    window.set_default_size(240, 200);

    let fixed_all = Fixed::new();
    window.add(&fixed_all);

    let button = Button::new_with_label("Click me!");
    button.set_size_request(100,100);
    fixed_all.put(&button, 10, 10);

    let textbox = Entry::new();
    textbox.set_size_request(100, 50);
    textbox.set_width_chars(0); //Allows textbox to have a small size.
    fixed_all.put(&textbox, 110, 10);

    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    button.connect_clicked(|_| {
        println!("Clicked!");
    });

    textbox.connect_motion_notify_event(|_, y| {
	println!("Mouse move! {} {}", y.get_position().0, y.get_position().1);
        Inhibit(false)
    });

    gtk::main();
}


