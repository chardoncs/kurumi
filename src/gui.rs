use gtk4 as gtk;
use poppler::Document;

use crate::{constants::APP_ID, error::{Error, ErrorKind}, util::convert_to_url};

use gtk::{gio::ApplicationFlags, glib, prelude::*, Application, ApplicationWindow, DrawingArea, Window};

/// Open PDF using Poppler
fn load_pdf_widget(win: &Window, doc: &Document) {

    let pager = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .spacing(20)
        .build();

    let total = doc.n_pages();

    for i in 0..total.min(10) {
        let page = doc.page(i).unwrap();
        let (w, h) = page.size();

        let page_canvas = DrawingArea::builder()
            .width_request(w.ceil() as i32)
            .height_request(h.ceil() as i32)
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Center)
            .build();
        
        page_canvas.set_draw_func(move |_, ctx, _, _| {
            ctx.set_source_rgba(1.0, 1.0, 1.0, 1.0);
            ctx.rectangle(0.0, 0.0, w, h);
            ctx.fill().unwrap();

            page.render(ctx);
        });

        pager.append(&page_canvas);
    }

    win.set_child(Some(&pager));
}

/// Create a new GTK window displaying PDF
pub fn new_pdf_window(path: Option<&str>, password: Option<&str>) -> Result<(), Error> {
    
    let doc_result = match path {
          Some(path) => Some(
            Document::from_file(convert_to_url(path)?.as_str(), password)
                .or_else(|err| Err(Error::new(ErrorKind::File, err.to_string().as_str())))
        ),
        None => None,
    };

    let doc = match doc_result {
        Some(result) => Some(result?),
        None => None,
    };

    let app = Application::builder()
        .application_id(APP_ID)
        .flags(ApplicationFlags::HANDLES_OPEN)
        .build();

    app.connect_open(move |app, files, _| {
        app.activate();

        if let Some(win) = app.active_window() {
            if let Some(ref doc) = doc {
                load_pdf_widget(&win, doc);

                // Change window title to file path
                if let Some(file) = files.first() {
                    if let Some(path) = file.path() {
                        if let Some(path) = path.to_str() {
                            win.set_title(Some(path));
                        }
                    }
                }
            }
        }
    });

    app.connect_activate(move |app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("New window")
            .build();

        window.present();
    });

    match app.run() {
        glib::ExitCode::SUCCESS => Ok(()),
        _ => Err(Error::new(ErrorKind::Window, "Window exited with code 1."))
    }
}
