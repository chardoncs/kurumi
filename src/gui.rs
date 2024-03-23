use poppler::Document;

use crate::{constants::{APP_ID, PAGE_GAP}, error::{Error, ErrorKind}, util::convert_to_url};

use gtk::{gio::{self, glib::{self, Object}, ApplicationFlags}, prelude::*, Application, DrawingArea, EventControllerScroll, EventControllerScrollFlags, ScrolledWindow, Window};

mod imp;

glib::wrapper! {
    pub struct KurumiMainWindow(ObjectSubclass<imp::KurumiMainWindow>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible,
                    gtk::Buildable, gtk::ConstraintTarget, gtk::Native,
                    gtk::Root, gtk::ShortcutManager;
}

impl KurumiMainWindow {
    pub fn new(app: &Application) -> Self {
        Object::builder()
            .property("application", app)
            .build()
    }
}

/// Open PDF using Poppler
fn load_pdf_widget(doc: &Document) {
    let pager = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .spacing(PAGE_GAP)
        .build();

    let ecs = EventControllerScroll::builder()
        .flags(EventControllerScrollFlags::VERTICAL)
        .build();

    ecs.connect_scroll_begin(|_| {
        println!("Scrolling detected.");
    });

    ecs.connect_scroll(|_, _, _| {
        println!("Scrolling in progress.");
        glib::Propagation::Proceed
    });

    pager.add_controller(ecs);

    let total = doc.n_pages();

    for i in 0..total.min(10) {
        let page = doc.page(i).unwrap();
        let (w, h) = page.size();

        let da = DrawingArea::builder()
            .width_request(w.ceil() as i32)
            .height_request(h.ceil() as i32)
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Center)
            .build();
        
        da.set_draw_func(move |_, ctx, _, _| {
            // Fill background
            ctx.set_source_rgba(1.0, 1.0, 1.0, 1.0);
            ctx.rectangle(0.0, 0.0, w, h);
            ctx.fill().unwrap();

            // Render page
            page.render(ctx);
        });

        pager.append(&da);
    }

    let sw = ScrolledWindow::builder()
        .child(&pager)
        .build();

    sw.set_hscrollbar_policy(gtk::PolicyType::Automatic);
    sw.set_vscrollbar_policy(gtk::PolicyType::External);
}

fn build_ui(app: &Application) {
    KurumiMainWindow::new(app).present();
}

/// Create a new GTK window displaying PDF
pub fn new_pdf_window(path: Option<&str>, password: Option<&str>) -> Result<(), Error> {

    gio::resources_register_include!("kurumi-ui.gresource")
        .expect("Register resources loading failed.");

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
                let sw = load_pdf_widget(doc);

                win.first_child().unwrap()
                    .first_child().unwrap()
                    .downcast_ref::<Window>().unwrap();

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

    app.connect_activate(build_ui);

    match app.run() {
        glib::ExitCode::SUCCESS => Ok(()),
        _ => Err(Error::new(ErrorKind::Window, "Window exited with code 1."))
    }
}
