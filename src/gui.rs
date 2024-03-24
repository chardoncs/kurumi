use poppler::Document;

use crate::{constants::APP_ID, error::{gtk_mismatching_error, Error, ErrorKind}, util::{convert_to_url, patch_title}};

use gtk::{gdk::Display, gio::{self, glib, ApplicationFlags}, prelude::*, Application, CssProvider};

use self::window::KurumiMainWindow;

mod key_binding;
mod pdfpage;
mod window;

fn build_ui(app: &Application) {
    KurumiMainWindow::new(app).present();
}

/// Create a new GTK window displaying PDF
pub fn new_pdf_window(path: Option<&str>, password: Option<&str>) -> Result<(), Error> {

    gio::resources_register_include!("kurumi-ui.gresource")
        .expect("Register resources loading failed.");

    let doc = match path {
        Some(path) => Some(
            Document::from_file(convert_to_url(path)?.as_str(), password)
                .or_else(|err| Err(Error::new(ErrorKind::File, err.to_string().as_str())))?
        ),
        None => None,
    };

    let app = Application::builder()
        .application_id(APP_ID)
        .flags(ApplicationFlags::HANDLES_OPEN)
        .build();

    app.connect_startup(|_| {
        load_css();
    });

    app.connect_open(move |app, files, _| {
        app.activate();

        if let Some(win) = app.active_window() {
            let win = win.downcast_ref::<KurumiMainWindow>()
                .expect(gtk_mismatching_error("kurumi window").as_str());

            win.set_doc(doc.clone());
            win.init();

            if let Some(file) = files.first() {
                if let Some(path_buf) = file.path() {
                    win.set_title(Some(patch_title(path_buf.to_str()).as_str()));
                }
            }
        }
    });

    app.connect_activate(build_ui);

    match app.run() {
        glib::ExitCode::SUCCESS => Ok(()),
        _ => Err(Error::new(ErrorKind::Window, "Window exited with code 1."))
    }
}

fn load_css() {
    let provider = CssProvider::new();

    provider.load_from_string(include_str!("../ui/css/style.css"));

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
