mod imp;

use std::borrow::Borrow;

use gtk::{gio, glib::{self, prelude::*, closure_local, object::Cast, property::PropertySet, subclass::types::ObjectSubclassIsExt, Object}, prelude::*, Application, EventControllerScroll, EventControllerScrollFlags, ListItem, NoSelection, ScrolledWindow, SignalListItemFactory};

use crate::{constants::{SCALE_MAX, SCALE_MIN, ZOOM_FACTOR}, mismatching_error, util::{check_page_fit, format_scale_status, patch_title, percentage, PageFitKind}};

use super::{key_binding::BindKeys, pdfpage::{PdfPage, PdfPageObject}};

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
            .property("title", patch_title(None))
            .build()
    }

    fn pages(&self) -> gio::ListStore {
        self.imp()
            .pages
            .borrow()
            .clone()
            .unwrap()
    }

    pub fn doc(&self) -> Option<poppler::Document> {
        self.imp()
            .doc
            .borrow()
            .clone()
    }

    pub fn set_doc(&self, doc: Option<poppler::Document>) {
        self.imp()
            .doc
            .set(doc);
    }

    pub fn page_scale(&self) -> f64 {
        self.imp()
            .scale
            .borrow()
            .get()
    }

    pub fn set_scale(&self, scale: f64) {
        let mut scale = scale.min(SCALE_MAX).max(SCALE_MIN);

        if let Some(doc) = self.doc() {
            if let Some(cur_page) = doc.page(self.imp().cur_page.get()) {
                let label = &self.imp().scale_percentage;
                let window = self.imp()
                    .page_container
                    .parent()
                    .and_downcast::<ScrolledWindow>()
                    .unwrap();
                
                let outer_width = window.size(gtk::Orientation::Horizontal);
                let outer_height = window.size(gtk::Orientation::Vertical);

                let page_fit = check_page_fit(cur_page.size(), (outer_width.into(), outer_height.into()), scale);

                if let PageFitKind::Width(new_scale) | PageFitKind::Page(new_scale) = page_fit {
                    scale = new_scale;
                }

                label.set_label(format_scale_status(scale, page_fit).as_str());
            }
        }

        self.imp()
            .scale
            .set(scale);
    }

    pub fn set_rel_scale<T>(&self, f: T)
    where
        T: Fn(f64) -> f64,
    {
        self.set_scale(f(self.page_scale()));
    }

    fn setup_page_model(&self) {
        let model = gio::ListStore::new::<PdfPageObject>();

        let imp = self.imp();

        imp.pages.replace(Some(model));
        imp.page_container.set_model(Some(&NoSelection::new(Some(self.pages()))));
    }

    fn setup_factory(&self) {
        let doc = self.doc();

        if doc.is_none() {
            return;
        }

        let factory = SignalListItemFactory::new();

        factory.connect_setup(move |_, item| {
            let page = PdfPage::new();

            item.downcast_ref::<gtk::ListItem>()
                .expect(mismatching_error!("gtk::ListItem"))
                .set_child(Some(&page));
        });

        factory.connect_bind(move |_, item| {
            let obj = item
                .downcast_ref::<ListItem>()
                .expect(mismatching_error!("gtk::ListItem"))
                .item()
                .and_downcast::<PdfPageObject>()
                .expect(mismatching_error!("kurumi page model"));

            let page = item
                .downcast_ref::<ListItem>()
                .expect(mismatching_error!("gtk::ListItem"))
                .child()
                .and_downcast::<PdfPage>()
                .expect(mismatching_error!("kurumi page"));

            page.activate(&obj, &doc.clone().unwrap());
        });

        factory.connect_unbind(move |_, item| {
            let page = item
                .downcast_ref::<ListItem>()
                .expect(mismatching_error!("gtk::ListItem"))
                .child()
                .and_downcast::<PdfPage>()
                .expect(mismatching_error!("kurumi page"));

            page.deactivate();
        });
        
        self.imp().page_container.set_factory(Some(&factory));
    }

    fn load_document(&self) {
        if let Some(doc) = self.doc() {

            let total_pages = doc.n_pages();

            let scale = self.imp().scale.get();

            self.pages().extend_from_slice(
                (0..total_pages)
                    .map(|i| PdfPageObject::new(i, scale))
                    .collect::<Vec<PdfPageObject>>()
                    .as_ref()
            );
        }
    }

    pub fn reload(&self) {
        self.pages().remove_all();
        self.load_document();
    }

    fn load_scroll_event(&self) {
        let container = self.imp().page_container.get();

        let pos = self.imp().pos_percentage.get();

        let controller = EventControllerScroll::builder()
            .flags(EventControllerScrollFlags::VERTICAL)
            .build();

        let adj = container.vadjustment();

        let pos1 = pos.clone();

        self.connect_closure("zoom", false, closure_local!(|win: KurumiMainWindow, dy: f64| {
            win.set_rel_scale(|s| s - dy * ZOOM_FACTOR);
            win.reload();
        }));

        let win = self.clone();

        controller.connect_scroll(move |_, _, dy| {

            if let Some(adj) = &adj {
                pos1.set_label(percentage(adj.value(), adj.upper() - adj.lower()).as_str());
            }

            let win_imp = win.imp();

            if win_imp.control_stack.get() > 0 {
                win.emit_by_name::<()>("zoom", &[&dy]);

                gtk::glib::Propagation::Stop
            } else {
                gtk::glib::Propagation::Proceed
            }
        });

        container.add_controller(controller);

        container.parent()
            .and_downcast_ref::<ScrolledWindow>()
            .expect(mismatching_error!("gtk::ScrolledWindow"))
            .connect_edge_reached(move |_, pos_type| {
                match pos_type {
                    gtk::PositionType::Top => pos.set_label("Top"),
                    gtk::PositionType::Bottom => pos.set_label("Bot"),
                    _ => {}
                }
            });
    }

    pub fn init(&self) {
        self.setup_factory();
        self.load_document();
        self.bind_keys();
        self.load_scroll_event();
    }
}

