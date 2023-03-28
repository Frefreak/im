use gtk::prelude::*;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("text input");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(800, 600);

    let textview = gtk::TextView::new();
    window.add(&textview);

    window.show_all();
}

fn main() {
    let application =
        gtk::Application::new(Some("nz.carso.test"), Default::default());

    application.connect_activate(build_ui);

    application.run();
}
