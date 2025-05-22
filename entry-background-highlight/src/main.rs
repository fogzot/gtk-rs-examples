use gtk::{glib, gdk, prelude::*};

fn main() -> glib::ExitCode {
    let application = gtk::Application::builder()
        .application_id("org.initd.examples.entry-background-highlight")
        .build();

    application.connect_startup(load_css);
    application.connect_activate(build_ui);


    // Run the application
    application.run()
}

static CSS: &'static str = r#"
    entry:focus-within {
        background: rgba(53, 132, 228, 1);
    }
"#;

fn load_css(_application: &gtk::Application) {
    let provider = gtk::CssProvider::new();

    provider.load_from_string(CSS);

    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().expect("could not connect to a display"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::builder()
        .application(application)
        .title("Entry with background highlight")
        .default_width(600)
        .default_height(480)
        .build();

    let entry1 = gtk::Entry::new();
    let entry2 = gtk::Entry::new();

    let row = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(12)
        .margin_start(24)
        .margin_end(24)
        .margin_top(24)
        .margin_bottom(24)
        .build();
    row.append(&entry1);
    row.append(&entry2);

    window.set_child(Some(&row));

    window.present();
}
